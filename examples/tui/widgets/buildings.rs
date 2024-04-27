use crossterm::event::Event;
use fiors::{get_building_db, get_recipe_db, FIOClient};
use ratatui::{
    layout::{Constraint, Margin, Rect},
    style::{Color, Style},
    widgets::{
        Block, Borders, Cell, Row, Scrollbar, ScrollbarOrientation, ScrollbarState, Table,
        TableState,
    },
    Frame,
};
use tracing::{span, trace};

use crate::{format_amount, format_price, NeedRefresh};

use super::{handle_scroll, SharedWidgetState, WidgetEnum};

pub struct BuildingsWidget {
    client: &'static FIOClient,
    username: String,
    planet_id: String,
    rows: Vec<Row<'static>>,
    scrollbar_state: ScrollbarState,
    table_state: TableState,
}

impl BuildingsWidget {
    pub fn new(client: &'static FIOClient, username: &str, planet_id: &str) -> Self {
        BuildingsWidget {
            client,
            username: username.to_string(),
            planet_id: planet_id.to_string(),
            rows: Vec::new(),
            scrollbar_state: ScrollbarState::default(),
            table_state: TableState::default(),
        }
    }

    pub fn switch_planets(&mut self, planet_id: &str) {
        self.planet_id = planet_id.to_string();
        self.rows.clear();
        self.table_state.select(None);
        self.scrollbar_state.first();
    }

    /// Return true if we scrolled and need to redraw the widget
    pub fn handle_input(&mut self, event: &Event, current_widget: WidgetEnum) -> NeedRefresh {
        if current_widget != WidgetEnum::Buildings {
            return NeedRefresh::No;
        }

        let i = self.table_state.selected();
        let new_i = handle_scroll(event, i, self.rows.len());
        self.table_state.select(new_i);
        if let Some(idx) = new_i {
            self.scrollbar_state = self.scrollbar_state.position(idx);
        }
        if let Some(idx) = new_i {
            self.scrollbar_state = self.scrollbar_state.position(idx);
        }

        if i != new_i {
            NeedRefresh::Redraw
        } else {
            NeedRefresh::No
        }
    }

    #[tracing::instrument(name = "building::update", skip(self, shared_state), fields(username = %self.username, planet_id = %self.planet_id))]
    pub async fn update(&mut self, shared_state: &mut SharedWidgetState) -> anyhow::Result<()> {
        let prod = self
            .client
            .get_planet_production(&self.username, &self.planet_id)
            .await?;

        let mut rows = Vec::new();

        let recipe_db = get_recipe_db();

        for row in &prod {
            let building = get_building_db().get(row.building_type.as_str()).unwrap();

            let building_cost = self.client.calc_building_cost(building.ticker).await?;
            // assume we repair our buildings after 90 days
            let repair_cost = building_cost - (building_cost * 0.5).floor();
            let daily_repair_cost = repair_cost / 90.0;

            let building_recipes: Vec<_> = recipe_db
                .iter()
                .filter(|r| r.building_ticker == row.building_ticker())
                .collect();

            rows.push(Row::new(vec![
                Cell::from(row.building_ticker()),
                Cell::default(), // recipe
                Cell::from(format!("{:.0}%", 100.0 * row.efficiency)),
                Cell::default(), // instant sell
                Cell::default(), // average sell
                Cell::default(), // our cogm
                Cell::default(), // market cogm
                Cell::default(), // daily output amount
                Cell::default(), // daily profit-
                Cell::default(), // daily profit+
            ]));

            let workforce_costs = self
                .client
                .calc_workforce_costs(&self.username, &self.planet_id, building.ticker, true, true)
                .await?;

            for recipe in building_recipes {
                let recipe_span =
                    span!(tracing::Level::DEBUG, "recipe", recipe = %recipe.standard_recipe_name);
                let _enter = recipe_span.enter();
                let day_scale = row.efficiency * 86400.0 / recipe.duration.as_secs() as f32;
                if recipe.outputs.is_empty() {
                    continue;
                }
                let daily_output_amt = recipe.outputs[0].amount as f32 * day_scale;
                trace!(
                    day_scale,
                    daily_output_amt,
                    building_efficency = row.efficiency
                );

                let mut market_cogm = daily_repair_cost + workforce_costs;
                let mut our_cogm = daily_repair_cost + workforce_costs;

                for input in recipe.inputs {
                    let daily_buy_amt = input.amount as f32 * day_scale;
                    let cx_info = self
                        .client
                        .get_exchange_info(&format!("{}.CI1", input.ticker))
                        .await
                        .unwrap();

                    let market_costs =
                        if let Some(total) = cx_info.instant_buy(daily_buy_amt.ceil() as u32) {
                            total.total_value
                        } else if let Some(x) = cx_info.price {
                            x * daily_buy_amt.ceil()
                        } else if let Some(x) = cx_info.get_any_price() {
                            x * daily_buy_amt.ceil()
                        } else {
                            0.0
                        };

                    market_cogm += market_costs;

                    let our_cogm_costs = shared_state
                        .cogm
                        .get(input.ticker)
                        .map(|cost| *cost * daily_buy_amt.ceil());
                    our_cogm += our_cogm_costs.unwrap_or(market_costs);

                    trace!(input.ticker, daily_buy_amt, market_costs, our_cogm_costs)
                }

                let cx_info = self
                    .client
                    .get_exchange_info(&format!("{}.CI1", recipe.outputs[0].ticker))
                    .await?;

                let market_instant_sell = cx_info
                    .instant_sell(daily_output_amt.ceil() as u32)
                    .map(|x| x.total_value / daily_output_amt.ceil());
                let market_average = cx_info.price;

                // best case profit uses our lowest cogm and the higest market sell price
                let (best_market, worst_market) = match (market_instant_sell, market_average) {
                    (None, None) => (None, None),
                    (None, Some(x)) => (Some(x), Some(x)),
                    (Some(x), None) => (Some(x), Some(x)),
                    (Some(x), Some(y)) => (Some(x.max(y)), Some(x.min(y))),
                };

                // .map(|price| price * daily_output_amt);

                let best_daily_costs = our_cogm.min(market_cogm);
                let best_profits = best_market.map(|x| (x * daily_output_amt) - best_daily_costs);
                let worst_profits = worst_market.map(|x| (x * daily_output_amt) - best_daily_costs);

                let best_profit_style = best_profits
                    .map(|x| {
                        if x > 0.0 {
                            Style::default().fg(Color::Green)
                        } else {
                            Style::default().fg(Color::Red)
                        }
                    })
                    .unwrap_or_default();

                let worst_profit_style = worst_profits
                    .map(|x| {
                        if x > 0.0 {
                            Style::default().fg(Color::Green)
                        } else {
                            Style::default().fg(Color::Red)
                        }
                    })
                    .unwrap_or_default();

                rows.push(Row::new(vec![
                    Cell::default(),
                    Cell::from(recipe.standard_recipe_name.to_string()),
                    Cell::default(),
                    Cell::from(format_price(our_cogm / daily_output_amt)),
                    Cell::from(format_price(market_cogm / daily_output_amt)),
                    Cell::from(market_instant_sell.map(format_price).unwrap_or_default()),
                    Cell::from(market_average.map(format_price).unwrap_or_default()),
                    Cell::from(format_amount(daily_output_amt)),
                    Cell::from(worst_profits.map(format_price).unwrap_or_default())
                        .style(worst_profit_style),
                    Cell::from(best_profits.map(format_price).unwrap_or_default())
                        .style(best_profit_style),
                ]));
            }
        }

        self.scrollbar_state = self.scrollbar_state.content_length(rows.len());
        self.rows = rows;

        Ok(())
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect, _current_widget: WidgetEnum) {
        let widths = [
            Constraint::Length(3), // building name
            Constraint::Fill(1),
            Constraint::Length(4), // efficiency
            Constraint::Length(6), // instant sell
            Constraint::Length(6), // average sell
            Constraint::Length(6), // our cogm
            Constraint::Length(6), // market cogm
            Constraint::Length(6), // Daily amount
            Constraint::Length(6), // Worst Daily profit
            Constraint::Length(6), // Best Daily profit
        ];
        let table = Table::new(self.rows.clone(), widths)
            .header(Row::new(vec![
                Cell::default(),      // building name
                Cell::default(),      // recipe
                Cell::from("Eff%"),   // efficiency
                Cell::from("O-COGM"), // our cogm
                Cell::from("M-COGM"), // market cogm
                Cell::from("Inst"),   // instant sell price
                Cell::from("Avg"),    // average sell price
                Cell::from("DAmt"),   // daily amount
                Cell::from("DProf-"), // daily profit
                Cell::from("DProf+"), // daily profit
            ]))
            .highlight_style(Style::default().fg(Color::Indexed(14)))
            .highlight_spacing(ratatui::widgets::HighlightSpacing::Always)
            .highlight_symbol(">")
            .block(Block::new().title("Table").borders(Borders::ALL));

        frame.render_stateful_widget(table, area, &mut self.table_state);
        let scrollbar = Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(None)
            .end_symbol(None);
        frame.render_stateful_widget(
            scrollbar,
            area.inner(&Margin {
                vertical: 1,
                horizontal: 0,
            }),
            &mut self.scrollbar_state,
        );
    }
}

use crossterm::event::Event;
use fiors::{get_building_db, get_recipe_db, types, FIOClient};
use ratatui::{
    layout::{Constraint, Margin, Rect},
    style::{Color, Style},
    symbols,
    widgets::{
        Block, Borders, Cell, Paragraph, Row, Scrollbar, ScrollbarOrientation, ScrollbarState,
        Table, TableState, Wrap,
    },
    Frame,
};

use crate::{format_price, NeedRefresh};

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

    pub async fn update(&mut self, shared_state: &mut SharedWidgetState) -> anyhow::Result<()> {
        let prod = self
            .client
            .get_planet_production(&self.username, &self.planet_id)
            .await?;

        let wf = self
            .client
            .get_planet_workforce_for_user(&self.username, &self.planet_id)
            .await
            .unwrap();

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
                Cell::default(),
                Cell::default(),
            ]));

            let workforce_costs = self
                .client
                .calc_workforce_costs(&self.username, &self.planet_id, building.ticker, true, true)
                .await?;

            for recipe in building_recipes {
                let day_scale = 86400.0 / recipe.duration.as_secs() as f32;
                if recipe.outputs.is_empty() {
                    continue;
                }
                let daily_output_amt = recipe.outputs[0].amount as f32 * day_scale;

                let mut cogm = daily_repair_cost + workforce_costs;

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

                    cogm += market_costs;

                    let our_cogm_costs = shared_state
                        .cogm
                        .get(input.ticker)
                        .map(|cost| *cost * daily_buy_amt.ceil());
                }

                rows.push(Row::new(vec![
                    Cell::default(),
                    Cell::from(format!("{}", recipe.standard_recipe_name)),
                    Cell::from(format!("{}", format_price(cogm / daily_output_amt))),
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
            Constraint::Length(10), // cogm
        ];
        let table = Table::new(self.rows.clone(), widths)
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

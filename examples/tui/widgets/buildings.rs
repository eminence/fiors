use crossterm::event::{Event, KeyCode, KeyEvent};
use fiors::{get_building_db, get_material_db, get_recipe_db, types::ResourceType, FIOClient};
use ratatui::{
    layout::{Constraint, Margin, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{
        Block, Borders, Cell, Row, Scrollbar, ScrollbarOrientation, ScrollbarState, Table,
        TableState,
    },
    Frame,
};
use tracing::{span, trace};

use crate::{format_amount, format_price, get_style_for_material, NeedRefresh};

use super::{handle_scroll, SharedWidgetState, WidgetEnum};

pub struct BuildingsWidget {
    client: &'static FIOClient,
    username: String,
    planet_id: String,
    rows: Vec<Row<'static>>,
    reciepe_columns: usize,
    scrollbar_state: ScrollbarState,
    table_state: TableState,
    use_lux1: bool,
    use_lux2: bool,
}

impl BuildingsWidget {
    pub fn new(client: &'static FIOClient, username: &str, planet_id: &str) -> Self {
        BuildingsWidget {
            client,
            username: username.to_string(),
            planet_id: planet_id.to_string(),
            rows: Vec::new(),
            reciepe_columns: 0,
            scrollbar_state: ScrollbarState::default(),
            table_state: TableState::default(),
            use_lux1: true,
            use_lux2: true,
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

        if let Event::Key(KeyEvent {
            code: KeyCode::Char('1'),
            ..
        }) = event
        {
            self.use_lux1 = !self.use_lux1;
            trace!(use_lux1 = self.use_lux1, "toggled L1");
            return NeedRefresh::APIRefresh;
        }
        if let Event::Key(KeyEvent {
            code: KeyCode::Char('2'),
            ..
        }) = event
        {
            self.use_lux2 = !self.use_lux2;
            trace!(use_lux1 = self.use_lux1, "toggled L2");
            return NeedRefresh::APIRefresh;
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

        let planet = self.client.get_planet(&self.planet_id).await?;
        let planet_cxid = planet.get_cx_mid().unwrap_or("CI1");

        let mut rows = Vec::new();
        // each recipe will put its inputs and outputs in this vec, which will get stitched together with "rows" once we know how many colums we need
        let mut input_output_rows = Vec::new();

        let recipe_db = get_recipe_db();
        let material_db = get_material_db();

        let planet_inventory = self
            .client
            .get_storage_for_user(&self.username, &self.planet_id)
            .await?;

        for row in &prod {
            let building = get_building_db().get(row.building_type.as_str()).unwrap();

            let building_cost = self
                .client
                .calc_building_cost(building.ticker, planet_cxid)
                .await?;
            // assume we repair our buildings after 90 days
            let repair_cost = building_cost - (building_cost * 0.5).floor();
            let daily_repair_cost = repair_cost / 90.0;

            let building_recipes: Vec<_> = recipe_db
                .iter()
                .filter(|r| r.building_ticker == row.building_ticker())
                .collect();

            input_output_rows.push(vec![Some(Cell::from(row.building_ticker()))]);
            rows.push(vec![
                Cell::default(), // empty fill column
                Cell::default(), // L1
                Cell::default(), // L2
                Cell::from(format!("{:.0}%", 100.0 * row.efficiency)),
                Cell::default(), // instant sell
                Cell::default(), // average sell
                Cell::default(), // our cogm
                Cell::default(), // market cogm
                Cell::default(), // daily output amount
                Cell::default(), // daily profit-
                Cell::default(), // daily profit+
            ]);

            // the current building efficiency already takes into account the supplies provided at this base.
            // so figure out what supplies we are currently providing
            // TODO

            let workforce_costs = self
                .client
                .calc_workforce_costs(
                    &self.username,
                    &self.planet_id,
                    building.ticker,
                    self.use_lux1,
                    self.use_lux2,
                )
                .await?;

            // if the build recipe has inputs, additional costs will be added later
            let mut market_cogm = daily_repair_cost + workforce_costs;
            let mut our_cogm = daily_repair_cost + workforce_costs;

            for recipe in building_recipes {
                let recipe_span =
                    span!(tracing::Level::DEBUG, "recipe", recipe = %recipe.standard_recipe_name);
                let _enter = recipe_span.enter();
                let day_scale = row.efficiency * 86400.0 / recipe.duration.as_secs() as f32;
                if recipe.outputs.is_empty() {
                    // resource extraction buildings will have no outputs, and need special handling here.
                    // we need to look at what resources are avaialble on the planet

                    let Some((rtype, a, b)) = (match recipe.building_ticker {
                        "COL" => Some((ResourceType::Gaseous, 60.0, 4.0)),
                        "EXT" => Some((ResourceType::Mineral, 70.0, 2.0)),
                        "RIG" => Some((ResourceType::Liquid, 70.0, 5.0)),
                        _ => None,
                    }) else {
                        continue;
                    };

                    for planet_resource in
                        planet.resources.iter().filter(|r| r.resource_type == rtype)
                    {
                        let mut this_reciepe_row = vec![None];

                        let material = material_db
                            .values()
                            .find(|mat| mat.material_id == planet_resource.material_id)
                            .unwrap();

                        let cx_info = self
                            .client
                            .get_exchange_info(&format!("{}.{planet_cxid}", material.ticker,))
                            .await?;
                        // how much we can produce per day:
                        // note: this doesn't take efficiency into account
                        let daily_amount_base = planet_resource.factor * a;
                        // how man units we make for each production cycle
                        let units_per_cycle = (daily_amount_base / b).ceil();
                        let cycle_time = units_per_cycle / daily_amount_base / row.efficiency; // in days, taking into account efficiency
                                                                                               // we need this to calculate how much production fees we'll pay
                        let cycles_per_day = 1.0 / cycle_time;

                        this_reciepe_row.push(Some({
                            let a = Span::raw(format!("{:>3}x", units_per_cycle));
                            let m = Span::raw(format!("{:^3}", material.ticker))
                                .style(get_style_for_material(material.ticker));
                            Cell::from(Line::from(vec![a, m]))
                        }));

                        rows.push(vec![
                            Cell::default(),                                   // empty fill column
                            Cell::from(if self.use_lux1 { "Y" } else { " " }), // L1
                            Cell::from(if self.use_lux2 { "Y" } else { " " }), // L2
                            Cell::default(),                                   // efficiency
                            Cell::from(format_price(
                                our_cogm / (daily_amount_base * row.efficiency),
                            )),
                            Cell::from(format_price(
                                market_cogm / (daily_amount_base * row.efficiency),
                            )),
                            Cell::from(""), // market instant sell
                            Cell::from(cx_info.price.map(format_price).unwrap_or_default()), // market average sell
                            Cell::from(format_amount(daily_amount_base * row.efficiency)),
                            Cell::from("") // worst profit
                                .style(Style::default()),
                            Cell::from("") // best profit
                                .style(Style::default()),
                        ]);
                        input_output_rows.push(this_reciepe_row);
                    }

                    continue;
                }
                let daily_output_amt = recipe.outputs[0].amount as f32 * day_scale;
                trace!(
                    day_scale,
                    daily_output_amt,
                    building_efficency = row.efficiency
                );

                let mut this_reciepe_row = vec![None];

                this_reciepe_row.push(Some({
                    let a = Span::raw(format!("{:>3}x", recipe.outputs[0].amount));
                    let m = Span::raw(format!("{:^3}", recipe.outputs[0].ticker))
                        .style(get_style_for_material(recipe.outputs[0].ticker));
                    Cell::from(Line::from(vec![a, m]))
                }));

                for input in recipe.inputs {
                    let have_input_in_inventory = planet_inventory
                        .as_ref()
                        .map(|sto| sto.items.contains_key(input.ticker))
                        .unwrap_or(false);

                    let a = Span::raw(format!("{:>3}x", input.amount)).style(
                        if have_input_in_inventory {
                            Style::default().bold()
                        } else {
                            Style::default().dim()
                        },
                    );
                    let m = Span::raw(format!("{:^3}", input.ticker))
                        .style(get_style_for_material(input.ticker));
                    this_reciepe_row.push(Some(Cell::from(Line::from(vec![a, m]))));

                    let daily_buy_amt = input.amount as f32 * day_scale;
                    let cx_info = self
                        .client
                        .get_exchange_info(&format!("{}.{planet_cxid}", input.ticker))
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
                    .get_exchange_info(&format!("{}.{planet_cxid}", recipe.outputs[0].ticker))
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

                input_output_rows.push(this_reciepe_row);
                rows.push(vec![
                    Cell::default(),                                   // empty fill column
                    Cell::from(if self.use_lux1 { "Y" } else { " " }), // L1
                    Cell::from(if self.use_lux2 { "Y" } else { " " }), // L2
                    Cell::default(),                                   // efficiency
                    Cell::from(format_price(our_cogm / daily_output_amt)),
                    Cell::from(format_price(market_cogm / daily_output_amt)),
                    Cell::from(market_instant_sell.map(format_price).unwrap_or_default()),
                    Cell::from(market_average.map(format_price).unwrap_or_default()),
                    Cell::from(format_amount(daily_output_amt)),
                    Cell::from(worst_profits.map(format_price).unwrap_or_default())
                        .style(worst_profit_style),
                    Cell::from(best_profits.map(format_price).unwrap_or_default())
                        .style(best_profit_style),
                ]);
            }
        }

        self.scrollbar_state = self.scrollbar_state.content_length(rows.len());

        // across all our recipes, what's the total number of columns we need for inputs and outputs?
        assert_eq!(input_output_rows.len(), rows.len());
        let max = input_output_rows.iter().map(|v| v.len()).max().unwrap();
        trace!(max, "max columns");

        self.rows = input_output_rows
            .into_iter()
            .zip(rows.into_iter())
            .map(|(mut ior, r)| {
                // if the first element of ior is Some(cell), then it's the name of the building
                if let Some(building) = ior.remove(0) {
                    // then this row is just the building name, plus `max` empty cells
                    let mut v = vec![building];
                    v.resize(max + 1, Cell::default());
                    v.extend(r);
                    Row::new(v)
                } else {
                    // the first element of r is the output, so remove that first and tack it on the end later
                    let output = ior.remove(0);
                    let mut v = vec![Cell::default()]; // empty building name
                    v.extend(
                        ior.into_iter()
                            .map(|maybe_cell| maybe_cell.unwrap_or_default()),
                    ); // inputs
                    v.resize(max, Cell::default()); // fill
                    v.push(output.unwrap_or_default()); // add output
                    v.extend(r); // add inputs

                    Row::new(v)
                }
            })
            .collect();
        self.reciepe_columns = max;

        shared_state.help_text.extend(vec![Span::raw(
            "This page shows all possible production receipes for each building at your base. ",
        )]);

        Ok(())
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect, _current_widget: WidgetEnum) {
        let mut widths = vec![Constraint::Length(3)]; // building name
        widths.resize(self.reciepe_columns + 1, Constraint::Length(7)); // inputs and outputs
        widths.push(Constraint::Fill(1)); // empty fill column

        widths.extend([
            Constraint::Length(1), // L1
            Constraint::Length(1), // L2
            Constraint::Length(4), // efficiency
            Constraint::Length(6), // instant sell
            Constraint::Length(6), // average sell
            Constraint::Length(6), // our cogm
            Constraint::Length(6), // market cogm
            Constraint::Length(6), // Daily amount
            Constraint::Length(6), // Worst Daily profit
            Constraint::Length(6), // Best Daily profit
        ]);

        let mut headers = vec![Cell::default()];
        headers.resize(self.reciepe_columns + 1, Cell::default());
        headers.extend([
            Cell::default(),      // empty fill column
            Cell::default(),      // L1
            Cell::default(),      // L2
            Cell::from("Eff%"),   // efficiency
            Cell::from("O-COGM"), // our cogm
            Cell::from("M-COGM"), // market cogm
            Cell::from("Inst"),   // instant sell price
            Cell::from("Avg"),    // average sell price
            Cell::from("DAmt"),   // daily amount
            Cell::from("DProf-"), // daily profit
            Cell::from("DProf+"), // daily profit
        ]);
        assert_eq!(widths.len(), headers.len());

        let table = Table::new(self.rows.clone(), widths)
            .header(Row::new(headers))
            .highlight_style(Style::default().bg(Color::DarkGray))
            .highlight_spacing(ratatui::widgets::HighlightSpacing::Always)
            .highlight_symbol(">")
            .block(
                Block::new()
                    .title("Production Buildings")
                    .borders(Borders::ALL),
            );

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

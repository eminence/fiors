use std::collections::HashMap;

use anyhow::Context;
use crossterm::event::{Event, KeyCode, KeyEvent};
use fiors::{get_material_db, FIOClient};
use ratatui::{
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::{self, Color, Modifier, Style},
    text::{Line, Span},
    widgets::{self, Block, Borders, Row, Scrollbar, Table},
    Frame,
};

use crate::{format_amount, format_price, get_style_for_days, get_style_for_material, NeedRefresh};

use super::{handle_scroll, SharedWidgetState, WidgetEnum};

pub struct ProductionWidget {
    client: &'static FIOClient,
    username: String,
    planet_id: String,
    production_rows: Vec<widgets::Row<'static>>,
    consumption_rows: Vec<widgets::Row<'static>>,
    needs_rows: Vec<widgets::Row<'static>>,

    table_state: [widgets::TableState; 3],
    scrollbar_state: [widgets::ScrollbarState; 3],
}

impl ProductionWidget {
    pub fn new(client: &'static FIOClient, username: &str, planet_id: &str) -> Self {
        Self {
            client,
            username: username.to_string(),
            planet_id: planet_id.to_string(),
            production_rows: Default::default(),
            consumption_rows: Default::default(),
            needs_rows: Default::default(),
            table_state: Default::default(),
            scrollbar_state: Default::default(),
        }
    }

    pub fn switch_planets(&mut self, planet_id: &str) {
        self.planet_id = planet_id.to_string();
        for t in &mut self.table_state {
            t.select(None);
        }
        for t in &mut self.scrollbar_state {
            t.first();
        }
        self.production_rows.clear();
        self.consumption_rows.clear();
        self.needs_rows.clear();
    }
    /// Return true if we scrolled and need to redraw the widget
    pub fn handle_input(&mut self, event: &Event, current_widget: WidgetEnum) -> NeedRefresh {
        let (state_idx, table_vec) = match current_widget {
            WidgetEnum::Production => (0, &self.production_rows),
            WidgetEnum::Consumption => (1, &self.consumption_rows),
            WidgetEnum::Needs => (2, &self.needs_rows),
            _ => return NeedRefresh::No,
        };

        let i = self.table_state[state_idx].selected();
        let new_i = handle_scroll(event, i, table_vec.len());
        self.table_state[state_idx].select(new_i);
        if let Some(idx) = new_i {
            self.scrollbar_state[state_idx] = self.scrollbar_state[state_idx].position(idx);
        }

        if i != new_i {
            NeedRefresh::Redraw
        } else {
            NeedRefresh::No
        }
    }

    pub async fn update(&mut self, shared_state: &mut SharedWidgetState) -> anyhow::Result<()> {
        let mut production_rows = Vec::new();
        let mut consumption_rows = Vec::new();
        let mut needs_rows = Vec::new();

        // get our base inventory for this planet
        let inv = self
            .client
            .get_storage_for_user(&self.username, &self.planet_id)
            .await?
            .context("No inventory found")?;

        let production_lines = self
            .client
            .get_planet_production(&self.username, &self.planet_id)
            .await?;

        // a map from (material, building) to daily production
        let mut total_daily_production: HashMap<(String, String), f32> = HashMap::new();
        // map from material to daily consumption
        let mut total_daily_consumption = HashMap::new();
        for prod in &production_lines {
            // if prod.building_type != "prefabPlant1" { continue }
            // dbg!(&prod);
            let daily = prod.daily_production();
            for (mat, amt) in daily.outputs {
                *total_daily_production
                    .entry((mat, prod.building_ticker().to_string()))
                    .or_insert(0.0) += amt;
            }
            for (mat, amt) in daily.inputs {
                *total_daily_consumption.entry(mat).or_insert(0.0) += amt;
            }
        }

        // get our workforce requirements
        let workforce = self
            .client
            .get_planet_workforce_for_user(&self.username, &self.planet_id)
            .await?;

        for (_workforce_type, details) in workforce.details {
            for need in details.needs {
                // let entry = total_needs.entry(need.ticker.clone()).or_default();
                // (*entry).0 = need.essential;
                // (*entry).1 += need.units_per_interval * num_days_inventory;
                if need.units_per_interval > 0.0 {
                    *total_daily_consumption.entry(need.ticker).or_insert(0.0) +=
                        need.units_per_interval;
                }
            }
        }

        let total_daily_production: Vec<_> = {
            let mut v: Vec<_> = total_daily_production.into_iter().collect();
            v.sort_by(|(a, _), (b, _)| {
                let a_cat = get_material_db().get(a.0.as_str()).unwrap().category;
                let b_cat = get_material_db().get(b.0.as_str()).unwrap().category;

                a_cat.cmp(&b_cat).then(a.0.cmp(&b.0))
            });
            v
        };

        for ((material, building), amount) in &total_daily_production {
            let net_amount = amount
                - total_daily_consumption
                    .get(material)
                    .copied()
                    .unwrap_or_default();

            // what is our COGM?
            let cogm = self
                .client
                .calc_cost_of_goods_manufactured(
                    &self.username,
                    &self.planet_id,
                    &building,
                    &material,
                )
                .await?;

            // what's the CX price range
            let cx = self
                .client
                .get_exchange_info(&format!("{material}.CI1"))
                .await?;

            let cx_min = cx.bid.unwrap_or(cx.price).min(cx.price).min(cx.low);
            let cx_max = cx.ask.unwrap_or(cx.price).max(cx.price).max(cx.high);

            production_rows.push(Row::new(vec![
                Span::raw("Recurring"),
                Span::raw(format_amount(*amount)),
                Span::raw(format!("{}", material)).style(get_style_for_material(&material)),
                Span::raw(if net_amount < 0.0 {
                    format!("-{}", format_amount(-net_amount))
                } else {
                    format!(" {}", format_amount(net_amount))
                }),
                Span::raw(format!("${}", format_price(cogm))),
                Span::raw(format!(
                    "${} - ${}",
                    format_price(cx_min),
                    format_price(cx_max)
                )),
            ]));
        }

        // now consider production lines that are handling non-recurring orders
        let mut things_already_reported = Vec::new();
        for prod in &production_lines {
            for order in &prod.orders {
                if order.recurring {
                    continue;
                }
                for output in &order.outputs {
                    if things_already_reported.contains(&output.material_ticker) {
                        continue;
                    }
                    let cogm = self
                        .client
                        .calc_cost_of_goods_manufactured(
                            &self.username,
                            &self.planet_id,
                            prod.building_ticker(),
                            &output.material_ticker,
                        )
                        .await?;

                    // what's the CX price range
                    let cx = self
                        .client
                        .get_exchange_info(&format!("{}.CI1", output.material_ticker))
                        .await?;

                    let cx_min = cx.bid.unwrap_or(cx.price).min(cx.price).min(cx.low);
                    let cx_max = cx.ask.unwrap_or(cx.price).max(cx.price).max(cx.high);

                    production_rows.push(Row::new(vec![
                        Span::raw("Producing"),
                        Span::raw(format_amount(output.material_amount as f32)),
                        Span::raw(format!("{}", output.material_ticker))
                            .style(get_style_for_material(&output.material_ticker)),
                        Span::raw(" ---"),
                        Span::raw(format!("${}", format_price(cogm))),
                        Span::raw(format!(
                            "${} - ${}",
                            format_price(cx_min),
                            format_price(cx_max)
                        )),
                    ]));
                    things_already_reported.push(output.material_ticker.clone());
                }
            }
        }

        shared_state
            .needs
            .extend(total_daily_consumption.clone().into_iter());

        let total_daily_consumption: Vec<_> = {
            let mut v: Vec<_> = total_daily_consumption.into_iter().collect();
            v.sort_by(|(a, _), (b, _)| {
                let a_cat = get_material_db().get(a.as_str()).unwrap().category;
                let b_cat = get_material_db().get(b.as_str()).unwrap().category;

                a_cat.cmp(&b_cat).then(a.cmp(&b))
            });
            v
        };

        for (material, amount) in total_daily_consumption {
            let net_amount = amount
                - total_daily_production
                    .iter()
                    .filter(|((m, _), _)| *m == material)
                    .map(|(_, a)| *a)
                    .sum::<f32>();
            if net_amount > 0.0 {
                let inv_amount = inv.items.get(&*material).map(|i| i.quantity).unwrap_or(0);
                let days = inv_amount as f32 / net_amount;

                consumption_rows.push(Row::new(vec![
                    Span::raw("Consuming"),
                    Span::raw(format_amount(net_amount)),
                    Span::raw(format!("{}", material)).style(get_style_for_material(&material)),
                    Span::raw("per day"),
                    Span::raw("lasting"),
                    Span::raw(format!("{:.1} days", days)).style(get_style_for_days(days)),
                ]));

                // assuming we want 21 days worth of materials, how much should we buy?
                let amount_in_inventory =
                    inv.items.get(&*material).map(|i| i.quantity).unwrap_or(0);
                let target_amount = net_amount * 21.0;
                let amount_to_buy = target_amount - amount_in_inventory as f32;
                if amount_to_buy > 0.0 {
                    // are any other bases producing a surplus of this material?
                    // TODO

                    needs_rows.push(Row::new(vec![
                        Span::raw(format_amount(amount_to_buy)),
                        Span::raw(format!("{}", material)).style(get_style_for_material(&material)),
                    ]));
                }
            }
        }

        self.scrollbar_state[0] = self.scrollbar_state[0].content_length(production_rows.len());
        self.production_rows = production_rows;
        self.scrollbar_state[1] = self.scrollbar_state[1].content_length(consumption_rows.len());
        self.consumption_rows = consumption_rows;
        self.scrollbar_state[2] = self.scrollbar_state[2].content_length(needs_rows.len());
        self.needs_rows = needs_rows;

        Ok(())
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect, current_widget: WidgetEnum) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints(
                [
                    Constraint::Ratio(1, 3), // production
                    Constraint::Ratio(1, 3), // consumption
                    Constraint::Ratio(1, 3), // needs
                ]
                .as_ref(),
            )
            .split(area);

        let production_table = Table::new(
            self.production_rows.clone(),
            [
                Constraint::Length(9), // "Producing"
                Constraint::Length(4), // amount
                Constraint::Length(3), // ticker
                Constraint::Length(5), // " Net"
                Constraint::Fill(1),   // "COGM"
                Constraint::Fill(1),   // CX
            ],
        )
        .header(
            Row::new(vec!["", "Amt", "Mat", "Net", "COGM", "CX Range"])
                .style(Style::default().add_modifier(Modifier::BOLD)),
        )
        .highlight_style(
            Style::default().fg(if current_widget == WidgetEnum::Production {
                Color::Indexed(14)
            } else {
                Color::White
            }),
        )
        .block(
            Block::default()
                .title("Production")
                .border_style(style::Style::default().fg(
                    if current_widget == WidgetEnum::Production {
                        Color::Cyan
                    } else {
                        Color::White
                    },
                ))
                .borders(Borders::ALL),
        );

        let consumption_table = Table::new(
            self.consumption_rows.clone(),
            [
                Constraint::Length(9), // "Consuming"
                Constraint::Length(4), // amount
                Constraint::Length(3), // ticker
                Constraint::Length(7), // "Per day"
                Constraint::Length(7), // "lasting"
                Constraint::Fill(1),   // days
            ],
        )
        .block(
            Block::default()
                .title("Net Consumption")
                .border_style(style::Style::default().fg(
                    if current_widget == WidgetEnum::Consumption {
                        Color::Cyan
                    } else {
                        Color::White
                    },
                ))
                .borders(Borders::ALL),
        );

        let consumption_table = if current_widget == WidgetEnum::Consumption {
            consumption_table.highlight_style(Style::default().fg(Color::Indexed(14)))
        } else {
            consumption_table
        };

        let needs_table = Table::new(
            self.needs_rows.clone(),
            [
                Constraint::Length(6), // amount
                Constraint::Length(3), // ticker
            ],
        )
        .highlight_style(Style::default().fg(if current_widget == WidgetEnum::Needs {
            Color::Indexed(14)
        } else {
            Color::White
        }))
        .block(
            Block::default()
                .title("Needs to acquire")
                .border_style(
                    style::Style::default().fg(if current_widget == WidgetEnum::Needs {
                        Color::Cyan
                    } else {
                        Color::White
                    }),
                )
                .borders(Borders::ALL),
        );

        frame.render_stateful_widget(production_table, chunks[0], &mut self.table_state[0]);
        frame.render_stateful_widget(consumption_table, chunks[1], &mut self.table_state[1]);
        frame.render_stateful_widget(needs_table, chunks[2], &mut self.table_state[2]);

        let scrollbar = Scrollbar::default()
            .orientation(widgets::ScrollbarOrientation::VerticalRight)
            .begin_symbol(None)
            .end_symbol(None);

        for (idx, state) in &mut self.scrollbar_state.iter_mut().enumerate() {
            let area = chunks[idx].inner(&Margin {
                vertical: 1,
                horizontal: 0,
            });

            frame.render_stateful_widget(scrollbar.clone(), area, state);
        }
    }
}

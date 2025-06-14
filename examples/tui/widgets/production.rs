use std::collections::HashMap;

use anyhow::Context;
use crossterm::event::Event;
use fiors::{get_material_db, COGMSource, FIOClient};
use ratatui::{
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::{self, Color, Modifier, Style},
    text::Span,
    widgets::{self, Block, Borders, Row, Scrollbar, Table},
    Frame,
};
use tracing::{debug, instrument, span, Level};

use crate::{
    format_amount, format_price, get_style_for_days, get_style_for_material, widgets::OverrideType,
    NeedRefresh,
};

use super::{handle_scroll, SharedWidgetState, WidgetEnum};

pub struct ProductionWidget {
    client: &'static FIOClient,
    username: String,
    planet_id: String,
    production_rows: Vec<widgets::Row<'static>>,
    consumption_rows: Vec<widgets::Row<'static>>,
    needs_rows: Vec<widgets::Row<'static>>,
    needs_volume: f32,
    needs_weight: f32,

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
            needs_volume: 0.0,
            needs_weight: 0.0,
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
        self.needs_volume = 0.0;
        self.needs_weight = 0.0;
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

    #[instrument(name="production::update", skip(self, shared_state), fields(planet_id=self.planet_id))]
    pub async fn update(&mut self, shared_state: &mut SharedWidgetState) -> anyhow::Result<()> {
        let mut production_rows = Vec::new();
        let mut consumption_rows = Vec::new();
        let mut needs_rows = Vec::new();

        let planet = self.client.get_planet(&self.planet_id).await?;
        let planet_cxid = planet.get_cx_mid().unwrap_or("CI1");

        // get our base inventory for this planet
        let inv = self
            .client
            .get_storage_for_user(&self.username, &self.planet_id)
            .await?
            .context("No inventory found")?;

        // get benten station warehouse
        let benten_cx_inv = self
            .client
            .get_storage_for_user(&self.username, "c2ae5c534bf694c3b1aee295176f7651")
            .await?;

        let production_lines = self
            .client
            .get_planet_production(&self.username, &self.planet_id)
            .await?;

        // a map from (material, building) to daily production
        let mut total_daily_production: HashMap<(String, String), f32> = HashMap::new();
        // map from material to daily consumption
        let mut total_daily_consumption = HashMap::new();
        for prod in &production_lines {
            let prod_span = span!(
                Level::DEBUG,
                "calc daily prod",
                building = prod.building_ticker()
            );
            let _enter = prod_span.enter();
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

            // what is our COGM if we bought everything from the market?
            let market_cogm = self
                .client
                .calc_cost_of_goods_manufactured(
                    &self.username,
                    &self.planet_id,
                    building,
                    material,
                    None,
                )
                .await?
                .unwrap_or_default();
            let market_cogm = COGMSource::market(market_cogm);
            tracing::debug!("market cogm for {} on {} is {:?}", material, planet.name, market_cogm);


            // what is our COGM if accounted for our own production?
            let our_cogm = self
                .client
                .calc_cost_of_goods_manufactured(
                    &self.username,
                    &self.planet_id,
                    building,
                    material,
                    Some(&shared_state.cogm),
                )
                .await?
                .unwrap_or_default();
            let our_cogm = COGMSource::our(our_cogm, &planet.name);
            tracing::debug!("our cogm for {} on {} is {:?}", material, planet.name, our_cogm);


            let best_cogm = COGMSource::min(&market_cogm, &our_cogm);
            tracing::debug!("best_cogm for {} on {} is {:?}", material, planet.name, best_cogm);
            

            shared_state
                .cogm
                .entry(material.clone())
                .and_modify(|x| x.update_if_better(best_cogm))
                .or_insert(best_cogm.clone());

            // what's the CX price range
            let cx = self
                .client
                .get_exchange_info(&format!("{material}.{planet_cxid}"))
                .await?;

            let mut prices: Vec<f32> = [cx.price, cx.bid, cx.ask, cx.low, cx.high]
                .iter()
                .flatten()
                .copied()
                .collect();
            prices.sort_by(f32::total_cmp);

            let cx_min = *prices.first().unwrap();
            let cx_max = *prices.last().unwrap();

            production_rows.push(Row::new(vec![
                Span::raw("Recurring"),
                Span::raw(format_amount(*amount)),
                Span::raw(material.to_string()).style(get_style_for_material(material)),
                Span::raw(if net_amount < 0.0 {
                    format!("-{}", format_amount(-net_amount))
                } else {
                    format!(" {}", format_amount(net_amount))
                }),
                Span::raw(format!("${}", format_price(our_cogm.get_cost()))),
                Span::raw(format!("${}", format_price(market_cogm.get_cost()))),
                Span::raw(format!(
                    "${} - ${}",
                    format_price(cx_min),
                    format_price(cx_max)
                )),
            ]));
        }

        // now consider production lines that are handling non-recurring orders
        let mut things_already_reported: Vec<(String, &str)> = Vec::new();
        for prod in &production_lines {
            for order in &prod.orders {
                if order.recurring {
                    continue;
                }
                for output in &order.outputs {
                    if things_already_reported
                        .contains(&(output.material_ticker.to_string(), prod.building_ticker()))
                    {
                        continue;
                    }
                    let market_cogm = self
                        .client
                        .calc_cost_of_goods_manufactured(
                            &self.username,
                            &self.planet_id,
                            prod.building_ticker(),
                            &output.material_ticker,
                            None,
                        )
                        .await?
                        .unwrap_or_default();
                    let market_cogm = COGMSource::market(market_cogm);

                    let our_cogm = self
                        .client
                        .calc_cost_of_goods_manufactured(
                            &self.username,
                            &self.planet_id,
                            prod.building_ticker(),
                            &output.material_ticker,
                            Some(&shared_state.cogm),
                        )
                        .await?
                        .unwrap_or_default();
                    let our_cogm = COGMSource::our(our_cogm, &planet.name);

                    let best_cogm = COGMSource::min(&market_cogm, &our_cogm);
                    let worst_cogm = fiors::COGMSource::max(&market_cogm, &our_cogm);

                    shared_state
                        .cogm
                        .entry(output.material_ticker.clone())
                        .and_modify(|x| x.update_if_better(best_cogm))
                        .or_insert(best_cogm.clone());

                    // what's the CX price range
                    let cx = self
                        .client
                        .get_exchange_info(&format!("{}.{planet_cxid}", output.material_ticker))
                        .await?;

                    let mut prices: Vec<f32> = [cx.price, cx.bid, cx.ask, cx.low, cx.high]
                        .iter()
                        .flatten()
                        .copied()
                        .collect();
                    prices.sort_by(f32::total_cmp);

                    let cx_min = *prices.first().unwrap();
                    let cx_max = *prices.last().unwrap();

                    production_rows.push(Row::new(vec![
                        Span::raw("Producing"),
                        Span::raw(format_amount(output.material_amount as f32)),
                        Span::raw(output.material_ticker.to_string())
                            .style(get_style_for_material(&output.material_ticker)),
                        Span::raw(" ---"),
                        Span::raw(format!("${}", format_price(best_cogm.get_cost()))),
                        Span::raw(format!("${}", format_price(worst_cogm.get_cost()))),
                        Span::raw(format!(
                            "${} - ${}",
                            format_price(cx_min),
                            format_price(cx_max)
                        )),
                    ]));
                    things_already_reported
                        .push((output.material_ticker.clone(), prod.building_ticker()));
                }
            }
        }

        let resupply_period = shared_state
            .overrides
            .planet_resupply_period
            .get(
                shared_state
                    .planet_id_map
                    .get(&self.planet_id)
                    .unwrap_or(&self.planet_id),
            )
            .copied()
            .unwrap_or(21);

        // our material overrides for this planet
        let materials_override_for_this_planet = shared_state.overrides.planet_materials.get(
            shared_state
                .planet_id_map
                .get(&self.planet_id)
                .unwrap_or(&self.planet_id),
        );

        shared_state
            .needs
            .entry(self.planet_id.clone())
            .or_default()
            .extend(total_daily_consumption.clone().into_iter());

        let excess_map = shared_state
            .excess
            .entry(self.planet_id.clone())
            .or_default();
        excess_map.clear();
        for (ticker, item) in &inv.items {
            let long_term_needed = total_daily_consumption.get(ticker.as_str()).unwrap_or(&0.0)
                * (resupply_period as f32);
            let needed_due_to_override = materials_override_for_this_planet
                .and_then(|x| x.get(ticker.as_str()).copied())
                .unwrap_or_default();
            let long_term_needed = needed_due_to_override.with(long_term_needed);
            if item.quantity > long_term_needed.ceil() as u32 {
                excess_map.insert(ticker.clone(), item.quantity as f32 - long_term_needed);
            }
        }

        // if we have materials overrides, split the overrides into two lists: materials that we're already consuming and materials that we're not
        let (consumed_materials_overrides, new_materials_overrides): (
            HashMap<_, _>,
            HashMap<_, _>,
        ) = materials_override_for_this_planet
            .map(|x| {
                x.iter()
                    .partition(|(mat, _)| total_daily_consumption.contains_key(*mat))
            })
            .unwrap_or_default();

        debug!(?consumed_materials_overrides, ?new_materials_overrides);

        enum ConsumptionType {
            Rate(f32),
            Amount(f32),
        }

        let total_daily_consumption: Vec<_> = {
            let mut v: Vec<_> = total_daily_consumption
                .into_iter()
                .map(|(mat, rate)| (mat, ConsumptionType::Rate(rate)))
                .collect();

            for (mat, amt) in new_materials_overrides {
                v.push((mat.clone(), ConsumptionType::Amount(amt.as_value())));
            }

            v.sort_by(|(a, _), (b, _)| {
                let a_cat = get_material_db().get(a.as_str()).unwrap().category;
                let b_cat = get_material_db().get(b.as_str()).unwrap().category;

                a_cat.cmp(&b_cat).then(a.cmp(b))
            });

            v
        };

        for (material, amount) in total_daily_consumption {
            let amount_to_buy = match amount {
                ConsumptionType::Rate(amount) => {
                    let net_amount_per_day = amount
                        - total_daily_production
                            .iter()
                            .filter(|((m, _), _)| *m == material)
                            .map(|(_, a)| *a)
                            .sum::<f32>();

                    if net_amount_per_day > 0.0 {
                        let inv_amount = inv.items.get(&*material).map(|i| i.quantity).unwrap_or(0);
                        let days = inv_amount as f32 / net_amount_per_day;

                        consumption_rows.push(Row::new(vec![
                            Span::raw("Consuming"),
                            Span::raw(format_amount(net_amount_per_day)),
                            Span::raw(material.to_string())
                                .style(get_style_for_material(&material)),
                            Span::raw("per day"),
                            Span::raw("lasting"),
                            Span::raw(format!("{:.1} days", days)).style(get_style_for_days(days)),
                        ]));

                        // assuming we want 21 days worth of materials, how much should we buy?
                        let amount_in_inventory =
                            inv.items.get(&*material).map(|i| i.quantity).unwrap_or(0);

                        let target_amount_based_on_consumption =
                            net_amount_per_day * (resupply_period as f32);
                        let target_amount = if let Some(override_amount) =
                            consumed_materials_overrides.get(&material)
                        {
                            override_amount.with(target_amount_based_on_consumption)
                        } else {
                            target_amount_based_on_consumption
                        };

                        target_amount - amount_in_inventory as f32
                    } else {
                        0.0
                    }
                }
                ConsumptionType::Amount(a) => {
                    let amount_in_inventory =
                        inv.items.get(&*material).map(|i| i.quantity).unwrap_or(0);

                    (a - amount_in_inventory as f32).max(0.0)
                }
            };

            if amount_to_buy > 0.0 {
                self.needs_volume += amount_to_buy * get_material_db().get(material.as_str()).unwrap().volume;
                self.needs_weight += amount_to_buy * get_material_db().get(material.as_str()).unwrap().weight;
                
                
                // are any other bases producing a surplus of this material?

                let mut e = Span::raw("");

                let mut planets_with_excess: Vec<_> = shared_state
                    .excess
                    .iter()
                    .filter_map(|(planet_id, excess)| {
                        // ignore our own planet:
                        if planet_id == &self.planet_id {
                            return None;
                        }
                        excess.get(&material).map(|x| {
                            (
                                shared_state
                                    .planet_id_map
                                    .get(planet_id)
                                    .map(|s| s.as_str())
                                    .unwrap_or("?"),
                                *x,
                            )
                        })
                    })
                    .collect();

                // also include bention station warehouse
                if let Some(wh) = &benten_cx_inv {
                    if let Some(x) = wh.items.get(&material) {
                        planets_with_excess.push(("Benten", x.quantity as f32));
                    }
                }

                planets_with_excess.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

                if !planets_with_excess.is_empty() {
                    let (amount_from_market, planets) = planets_with_excess.iter().fold(
                        (amount_to_buy, "Take from".to_string()),
                        |(amt_left, s), (p, amt_on_planet)| {
                            if amt_left <= 0.0 {
                                (0.0, s)
                            } else if *amt_on_planet >= amt_left {
                                (0.0, format!("{} {}: {}", s, p, format_amount(amt_left)))
                            } else if amt_left > *amt_on_planet {
                                (
                                    amt_left - amt_on_planet,
                                    format!("{} {}: {}", s, p, format_amount(*amt_on_planet)),
                                )
                            } else {
                                (0.0, s)
                            }
                        },
                    );

                    if amount_from_market > 0.0 {
                        e = Span::raw(format!(
                            "{planets} (plus {} from market)",
                            format_amount(amount_from_market)
                        ));
                    } else {
                        e = Span::raw(planets.to_string());
                    }
                } else {
                    // we need to buy from the market
                    // but also check to see if we can make this material on-planet
                    // TODO
                    
                    let cx_info = self
                        .client
                        .get_exchange_info(&format!("{}.{planet_cxid}", material))
                        .await?;
                    if let Some(a) = cx_info.instant_buy(amount_to_buy.ceil() as u32) {
                        e = Span::raw(format!(
                            "Buy for ${} at {}/u from CI1",
                            format_price(a.total_value),
                            format_price(a.price_limit)
                        ));
                    }
                }

                needs_rows.push(Row::new(vec![
                    Span::raw(format_amount(amount_to_buy)),
                    Span::raw(material.to_string()).style(get_style_for_material(&material)),
                    e,
                ]));
            }
        }

        // Do we have any materials in our inventory that we need to move to benten?
        if let Some(planet_overrides) = materials_override_for_this_planet {
            for (mat, item) in &inv.items {
                if let Some(OverrideType::Maxiumum(m)) = planet_overrides.get(mat) {
                    if item.quantity > *m {
                        needs_rows.push(Row::new(vec![
                            Span::raw(format_amount((item.quantity - *m) as f32)),
                            Span::raw(mat.to_string()).style(get_style_for_material(mat)),
                            Span::raw("Send to Benten".to_string()),
                        ]));
                    }
                }
            }
    
        }
       

        shared_state.help_text.extend(vec![
            Span::raw("This mode shows your production and consumption. "),
            Span::raw(format!(
                "Needs are based on a {resupply_period} day resupply period. "
            )),
            Span::raw("Press "),
            Span::styled("tab", crate::HELP_TEXT_KEY_STYLE),
            Span::raw(" to switch widgets. "),
        ]);

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
                Constraint::Fill(1),   // "COGM"
                Constraint::Fill(2),   // CX
            ],
        )
        .header(
            Row::new(vec!["", "Amt", "Mat", "Net", "COGM", "COGM", "CX Range"])
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
                Constraint::Fill(1),
            ],
        )
        .highlight_style(Style::default().fg(if current_widget == WidgetEnum::Needs {
            Color::Indexed(14)
        } else {
            Color::White
        }))
        .block(
            Block::default()
                .title(format!("Need to acquire ({:.1}t / {:.1}m³)", self.needs_weight, self.needs_volume))
                .border_style(
                    style::Style::default().fg(if current_widget == WidgetEnum::Needs {
                        Color::Cyan
                    } else {
                        Color::White
                    }),
                )
                .borders(Borders::ALL),
        );

        let scrollbar = Scrollbar::default()
            .orientation(widgets::ScrollbarOrientation::VerticalRight)
            .begin_symbol(None)
            .end_symbol(None);

        for (idx, ((table, row_count), margin)) in
            [production_table, consumption_table, needs_table]
                .into_iter()
                .zip([
                    self.production_rows.len(),
                    self.consumption_rows.len(),
                    self.needs_rows.len(),
                ])
                .zip([3u16, 2, 2]) // production widget has a different margin due to table header
                .enumerate()
        {
            frame.render_stateful_widget(table, chunks[idx], &mut self.table_state[idx]);

            if chunks[idx].height - margin < row_count as u16 {
                let area = chunks[idx].inner(&Margin {
                    vertical: 1,
                    horizontal: 0,
                });

                frame.render_stateful_widget(
                    scrollbar.clone(),
                    area,
                    &mut self.scrollbar_state[idx],
                );
            }
        }
    }
}

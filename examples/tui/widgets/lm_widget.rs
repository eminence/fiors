use anyhow::Context;
use crossterm::event::{Event, KeyCode, KeyEvent, MouseEvent, MouseEventKind};
use fiors::{
    get_material_db,
    materials::MaterialCategory,
    types::{LocalMarketContract, Ticker},
    FIOClient,
};
use ratatui::{
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::{self, Color, Modifier, Style},
    text::{Line, Span},
    widgets::{self, Block, Borders, Paragraph, Row, Scrollbar, Table, Wrap},
    Frame,
};

use crate::{format_amount, format_price, get_style_for_material, NeedRefresh};

use super::{handle_scroll, SharedWidgetState, WidgetEnum};

/// Local market data, for a given planet
pub struct LocalMarketWidget {
    client: &'static FIOClient,
    username: String,
    planet_id: String,
    table_rows: Vec<widgets::Row<'static>>,
    notes: Vec<Line<'static>>,
    table_state: widgets::TableState,
    scrollbar_state: widgets::ScrollbarState,
    details: Option<Vec<Line<'static>>>,
}

impl LocalMarketWidget {
    pub fn new(client: &'static FIOClient, username: &str, planet_id: &str) -> Self {
        // let scrollbar = widgets::Scrollbar::default();
        Self {
            client,
            username: username.to_string(),
            planet_id: planet_id.to_string(),
            table_rows: Default::default(),
            table_state: Default::default(),
            scrollbar_state: Default::default(),
            notes: Default::default(),
            details: None,
        }
    }

    pub fn switch_planets(&mut self, planet_id: &str) {
        self.planet_id = planet_id.to_string();
        self.table_state.select(None);
        self.scrollbar_state.first();
        self.notes.clear();
        self.table_rows.clear();
        self.details = None;
    }

    /// Return true if we need to redraw the widget
    pub fn handle_input(&mut self, event: &Event, current_widget: WidgetEnum) -> NeedRefresh {
        if current_widget != WidgetEnum::LocalMarket {
            return NeedRefresh::No;
        }

        let mut redraw = NeedRefresh::No;
        if let Event::Key(KeyEvent {
            code: KeyCode::Enter,
            ..
        }) = event
        {
            if self.details.is_none() && self.table_state.selected().is_some() {
                self.details = Some(Vec::new());
            } else {
                self.details = None;
            }

            return redraw.update(NeedRefresh::APIRefresh);
        }

        let i = self.table_state.selected();
        let new_i = handle_scroll(event, i, self.table_rows.len());
        self.table_state.select(new_i);
        if let Some(idx) = new_i {
            self.scrollbar_state = self.scrollbar_state.position(idx);
        }

        let new_scroll = i != new_i;
        if self.details.is_some() && new_scroll {
            self.details = Some(Vec::new());
            redraw = redraw.update(NeedRefresh::APIRefresh);
        } else if new_scroll {
            redraw = redraw.update(NeedRefresh::Redraw);
        }

        redraw
    }

    fn add_details(&mut self, ad: &LocalMarketContract, cx: &Ticker) {
        if let Some(details) = &mut self.details {
            // details.push(format!("selected={:?} idx={:?}", self.table_state.selected(), table_idx).into());
            if let Some(i) = cx.instant_sell(ad.material_amount) {
                details.push(
                    format!(
                        "Instantly selling {} {} on the CX would net {}, {:.0}% of the LM ask",
                        ad.material_amount,
                        ad.material_ticker,
                        format_price(i.total_value),
                        100.0 * i.total_value / ad.total_price
                    )
                    .into(),
                );
            } else {
                details.push("No possible to instant sell this on CX".into());
            }

            if let Some(i) = cx.instant_buy(ad.material_amount) {
                details.push(
                    format!(
                        "Instantly buying {} {} on the CX would cost {}, {:.0}% of the LM ask",
                        ad.material_amount,
                        ad.material_ticker,
                        format_price(i.total_value),
                        100.0 * i.total_value / ad.total_price
                    )
                    .into(),
                );
            }
        }
    }

    //
    pub async fn update(&mut self, shared_state: &mut SharedWidgetState) -> anyhow::Result<()> {
        let lm = self.client.get_planet_localmarket(&self.planet_id).await?;
        // keep track of our own selling orders, we'll need this later
        let mut our_selling_orders = Vec::new();
        let mut notes = Vec::new();
        let mut rows = Vec::new();

        let mut table_idx = 0;

        for ad in lm.selling_ads.iter() {
            let ticker_style = get_style_for_material(&ad.material_ticker);

            let mut symbol = " ";
            if ad.creator_company_code == "EM32" {
                our_selling_orders.push(ad.material_ticker.clone());
                continue;
            }

            let cx = self
                .client
                .get_exchange_info(&format!("{}.CI1", ad.material_ticker))
                .await?;

            let price_per_unit = ad.total_price / ad.material_amount as f32;
            if price_per_unit < cx.ask.unwrap_or(cx.price) {
                symbol = "+";
                notes.push(Line::from(vec![
                    Span::raw("Good deal on "),
                    Span::raw(format!("{}", ad.material_ticker)).style(ticker_style),
                    Span::raw(" if we need any"),
                ]));
            }

            // Could we buy this local market order and instantly sell it on the CX for a profit?
            let instant = cx.instant_sell(ad.material_amount);
            if let Some(instant) = instant {
                if instant.total_value > ad.total_price {
                    symbol = "!";
                    notes.push(Line::from(vec![
                        Span::raw("Can by "),
                        Span::raw(format!("{}", ad.material_ticker)).style(ticker_style),
                        Span::raw(format!(
                            " for {} and instantly sell for {}",
                            ad.total_price, instant.total_value
                        )),
                    ]));
                }
            }

            let cx_ask = cx
                .ask
                .map(|b| format!("{:.0}%", 100.0 * price_per_unit / b))
                .unwrap_or("N/A".into());

            rows.push(Row::new(vec![
                Span::raw(symbol),
                Span::raw(format!("{}", ad.creator_company_name)),
                Span::raw("selling"),
                Span::raw(format_amount(ad.material_amount as f32)),
                Span::raw(format!("{}", ad.material_ticker))
                    .style(get_style_for_material(&ad.material_ticker)),
                Span::raw(format!("{} {}", format_price(ad.total_price), ad.currency)),
                Span::raw(format!("{}/u", format_price(price_per_unit))),
                Span::raw(cx_ask),
            ]));

            if self.table_state.selected() == Some(table_idx) {
                self.add_details(&ad, &cx);
            }
            table_idx += 1;
        }
        for ad in &lm.buying_ads {
            let ticker_style = get_style_for_material(&ad.material_ticker);
            let mut symbol = " ";
            let price_per_unit = ad.total_price / ad.material_amount as f32;
            let cx = self
                .client
                .get_exchange_info(&format!("{}.CI1", ad.material_ticker))
                .await?;
            if price_per_unit > cx.bid.unwrap_or(cx.price) {
                notes.push(Line::raw(format!("Good deal on {}", ad.material_ticker)));
                symbol = "+";
            }

            // compare the ad price to the instant buy price on the CX
            let instant = cx.instant_buy(ad.material_amount);
            let cx_instant = instant
                .map(|i| format!("{:.0}%", 100.0 * ad.total_price / i.total_value))
                .unwrap_or("N/A".into());

            rows.push(Row::new(vec![
                Span::raw(symbol),
                Span::raw(format!("{}", ad.creator_company_name)),
                Span::raw("buying"),
                Span::raw(format_amount(ad.material_amount as f32)),
                Span::raw(format!("{}", ad.material_ticker)).style(ticker_style),
                Span::raw(format!("{} {}", format_price(ad.total_price), ad.currency)),
                Span::raw(format!("{}/u", format_price(price_per_unit))),
                Span::raw(cx_instant),
            ]));

            if self.table_state.selected() == Some(table_idx) {
                self.add_details(&ad, &cx);
            }
            table_idx += 1;
        }

        let planet = self.client.get_planet(&self.planet_id).await?;
        // get our base inventory for this planet
        let inv = self
            .client
            .get_storage_for_user(&self.username, &self.planet_id)
            .await?
            .context("No inventory found")?;

        for (needed_material, needed_amount) in &shared_state.needs {
            let inv_amount = inv
                .items
                .get(needed_material.as_str())
                .map(|i| i.quantity)
                .unwrap_or(0);
            let excess_amount = inv_amount as f32 - (*needed_amount * 21.0);

            if excess_amount < 1.0 {
                continue;
            }

            let need_category = get_material_db()
                .get(needed_material.as_str())
                .unwrap()
                .category;
            if !our_selling_orders.contains(&needed_material)
                && !["DW", "RAT", "COF"].contains(&needed_material.as_str())
                && (need_category == MaterialCategory::ConsumablesBasic
                    || need_category == MaterialCategory::ConsumablesLuxury)
                && excess_amount > 10.0
            {
                let ticker_style = get_style_for_material(&needed_material);
                let cx = self
                    .client
                    .get_exchange_info(&format!("{needed_material}.CI1"))
                    .await?;
                let lm_fee = if planet.local_market_fee_factor > 0.0 {
                    50.0 + (30.0 * planet.local_market_fee_factor)
                } else {
                    0.0
                };
                let proposed_price = ((10.0 * cx.high * 1.15 + lm_fee) / 10.0).floor() * 10.0;
                notes.push(
                    Line::from(vec![
                        Span::raw("We have excesses "),
                        Span::raw(format!("{}", needed_material)).style(ticker_style),
                        Span::raw(format!(
                            ", sell 10 units on LM at proposed price of {}?",
                            proposed_price
                        )),
                    ])
                    .into(),
                );
            }
        }

        self.scrollbar_state = self.scrollbar_state.content_length(rows.len());
        self.table_rows = rows;
        self.notes = notes;

        Ok(())
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect, current_widget: WidgetEnum) {
        let table = Table::new(
            self.table_rows.clone(),
            [
                Constraint::Length(1), // symbol
                Constraint::Fill(2),   // company
                Constraint::Length(7), // action
                Constraint::Length(4), // amount
                Constraint::Length(3), // ticker
                Constraint::Fill(1),   // total price
                Constraint::Fill(1),   // ppu
                Constraint::Length(5), // percent of CX
            ],
        )
        .header(
            Row::new(vec![
                "", "Company", "Action", "Amt", "Mat", "Price", "PPU", "CX %",
            ])
            .style(Style::default().add_modifier(Modifier::BOLD)),
        )
        .highlight_style(
            Style::default().fg(if current_widget == WidgetEnum::LocalMarket {
                Color::Indexed(14)
            } else {
                Color::White
            }),
        )
        .block(
            Block::default()
                .title("Local Market")
                .border_style(style::Style::default().fg(
                    if current_widget == WidgetEnum::LocalMarket {
                        Color::Cyan
                    } else {
                        Color::White
                    },
                ))
                .borders(Borders::ALL),
        );

        // construct paragraph to hold other info
        let paragraph = Paragraph::new(self.notes.clone())
        .wrap(Wrap { trim: true })
        .block(
            Block::default()
                .title("LM notes")
                .border_style(style::Style::default().fg(
                    if current_widget == WidgetEnum::LMNotes {
                        Color::Cyan
                    } else {
                        Color::White
                    },
                ))
                .borders(Borders::ALL)
                ,
        );

        let chunks = if self.details.is_some() {
            Layout::default()
                .direction(Direction::Vertical)
                .margin(0)
                .constraints(
                    [
                        Constraint::Fill(1), // table
                        Constraint::Fill(1), // paragraph
                        Constraint::Min(5),  // details
                    ]
                    .as_ref(),
                )
                .split(area)
        } else {
            Layout::default()
                .direction(Direction::Vertical)
                .margin(0)
                .constraints(
                    [
                        Constraint::Fill(1), // table
                        Constraint::Fill(1), // paragraph
                    ]
                    .as_ref(),
                )
                .split(area)
        };

        let scrollbar = Scrollbar::default()
            .orientation(widgets::ScrollbarOrientation::VerticalRight)
            .begin_symbol(None)
            .end_symbol(None);

        frame.render_stateful_widget(table, chunks[0], &mut self.table_state);
        // the -3 is to account for the top and bottom borders and table header
        if chunks[0].height - 3 < self.table_rows.len() as u16 {
            frame.render_stateful_widget(
                scrollbar,
                chunks[0].inner(&Margin {
                    vertical: 1,
                    horizontal: 0,
                }),
                &mut self.scrollbar_state,
            );
        }
        frame.render_widget(paragraph, chunks[1]);

        if let Some(details) = &self.details {
            let details = Paragraph::new(details.clone())
                .block(Block::default().title("Details").borders(Borders::ALL))
                .wrap(Wrap { trim: true });
            frame.render_widget(details, chunks[2]);
        }
    }
}

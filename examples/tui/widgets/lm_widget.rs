use crossterm::event::{Event, KeyCode, KeyEvent, MouseEvent, MouseEventKind};
use fiors::FIOClient;
use ratatui::{
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::{self, Color, Modifier, Style},
    text::{Line, Span},
    widgets::{self, Block, Borders, Paragraph, Row, Scrollbar, Table},
    Frame,
};

use crate::get_style_for_material;

use super::{handle_scroll, WidgetEnum};

/// Local market data, for a given planet
pub struct LocalMarketWidget {
    client: &'static FIOClient,
    planet_id: String,
    table_rows: Vec<widgets::Row<'static>>,
    notes: Vec<Line<'static>>,
    table_state: widgets::TableState,
    scrollbar_state: widgets::ScrollbarState,
}

impl LocalMarketWidget {
    pub fn new(client: &'static FIOClient, planet_id: &str) -> Self {
        // let scrollbar = widgets::Scrollbar::default();
        Self {
            client,
            planet_id: planet_id.to_string(),
            table_rows: Default::default(),
            table_state: Default::default(),
            scrollbar_state: Default::default(),
            notes: Default::default(),
        }
    }

    pub fn switch_planets(&mut self, planet_id: &str) {
        self.planet_id = planet_id.to_string();
        self.table_state.select(None);
        self.scrollbar_state.first();
    }

    /// Return true if we scrolled and need to redraw the widget
    pub fn handle_input(&mut self, event: &Event, current_widget: WidgetEnum) -> bool {
        if current_widget != WidgetEnum::LocalMarket {
            return false;
        }

        let i = self.table_state.selected();
        let new_i = handle_scroll(event, i, self.table_rows.len());
        self.table_state.select(new_i);
        if let Some(idx) = new_i {
            self.scrollbar_state = self.scrollbar_state.position(idx);
        }

        i != new_i
    }

    //
    pub async fn update(&mut self) -> anyhow::Result<()> {
        let lm = self.client.get_planet_localmarket(&self.planet_id).await?;
        let mut our_selling_orders = Vec::new();
        let mut notes = Vec::new();
        let mut rows = Vec::new();
        for ad in &lm.selling_ads {
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
                Span::raw(ad.material_amount.to_string()),
                Span::raw(format!("{}", ad.material_ticker))
                    .style(get_style_for_material(&ad.material_ticker)),
                Span::raw(format!("{:.1} {}", ad.total_price, ad.currency)),
                Span::raw(format!("{:.1}/u", price_per_unit)),
                Span::raw(cx_ask),
            ]));
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

            let cx_bid = cx
                .bid
                .map(|b| format!("{:.0}%", 100.0 * price_per_unit / b))
                .unwrap_or("N/A".into());

            rows.push(Row::new(vec![
                Span::raw(symbol),
                Span::raw(format!("{}", ad.creator_company_name)),
                Span::raw("buying"),
                Span::raw(ad.material_amount.to_string()),
                Span::raw(format!("{}", ad.material_ticker)).style(ticker_style),
                Span::raw(format!("{:.1} {}", ad.total_price, ad.currency)),
                Span::raw(format!("{:.1}/u", price_per_unit)),
                Span::raw(cx_bid),
            ]));
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
                Constraint::Length(5), // amount
                Constraint::Length(3), // ticker
                Constraint::Fill(1),   // total price
                Constraint::Fill(1),   // ppu
                Constraint::Length(5), // percent of CX
            ],
        )
        .header(
            Row::new(vec![
                "", "Company", "Action", "Amt", "Tck", "Price", "PPU", "CX %",
            ])
            .style(Style::default().add_modifier(Modifier::BOLD)),
        )
        .highlight_style(Style::default().fg(if current_widget == WidgetEnum::LocalMarket {
            Color::Indexed(14)
        } else {
            Color::White
        }))
        .block(
            Block::default()
                .title("Local Market")
                .border_style(style::Style::default().fg(if current_widget == WidgetEnum::LocalMarket {
                    Color::Cyan
                } else {
                    Color::White
                }))
                .borders(Borders::ALL),
        );

        // construct paragraph to hold other info
        let paragraph = Paragraph::new(self.notes.clone()).block(
            Block::default()
                .title("LM notes")
                .border_style(style::Style::default().fg(if current_widget == WidgetEnum::LMNotes {
                    Color::Cyan
                } else {
                    Color::White
                }))
                .borders(Borders::ALL),
        );

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints(
                [
                    Constraint::Fill(1), // table
                    Constraint::Fill(1), // paragraph
                ]
                .as_ref(),
            )
            .split(area);

        //

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
    }
}

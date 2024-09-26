use anyhow::Context;
use chrono::Utc;
use crossterm::event::{Event, KeyCode, KeyEvent};
use fiors::{get_material_db, FIOClient};
use ratatui::{
    layout::{Constraint, Margin, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::Span,
    widgets::{
        Block, Borders, Cell, Clear, Row, Scrollbar, ScrollbarOrientation, ScrollbarState, Table,
        TableState,
    },
    Frame,
};

use crate::{centered_rect, format_price, get_style_for_material, NeedRefresh};

use super::{handle_scroll, DepthChartWidget, SharedWidgetState, WidgetEnum};

pub struct MarketWidget {
    client: &'static FIOClient,
    username: String,
    buy_rows: Vec<Row<'static>>,
    sell_rows: Vec<Row<'static>>,
    table_state: TableState,
    scrollbar_state: ScrollbarState,
    want_depth_widget: bool,
    depth_widget: Option<DepthChartWidget>,
}

impl MarketWidget {
    pub fn new(client: &'static FIOClient, username: &str) -> Self {
        Self {
            client,
            username: username.to_string(),
            buy_rows: Vec::new(),
            sell_rows: Vec::new(),
            table_state: TableState::default(),
            scrollbar_state: ScrollbarState::default(),
            want_depth_widget: false,
            depth_widget: None,
        }
    }

    /// Return true if we scrolled and need to redraw the widget
    pub fn handle_input(&mut self, event: &Event, current_widget: WidgetEnum) -> NeedRefresh {
        if current_widget != WidgetEnum::Market {
            return NeedRefresh::No;
        }

        let mut redraw = NeedRefresh::No;

        if let Event::Key(KeyEvent {
            code: KeyCode::Char('D'),
            ..
        }) = event
        {
            self.want_depth_widget = !self.want_depth_widget;
            tracing::trace!("Toggled depth chart widget, and requesting API refresh");
            return NeedRefresh::APIRefresh;
        }

        let i = self.table_state.selected();
        let new_i = handle_scroll(event, i, self.buy_rows.len() + self.sell_rows.len());
        self.table_state.select(new_i);
        if let Some(idx) = new_i {
            self.scrollbar_state = self.scrollbar_state.position(idx);
        }

        let new_scroll = i != new_i;
        if new_scroll {
            redraw = redraw.update(NeedRefresh::Redraw);
        }

        redraw
    }

    #[tracing::instrument(name = "market::update", skip(self, _shared_state))]
    pub async fn update(&mut self, _shared_state: &mut SharedWidgetState) -> anyhow::Result<()> {
        let own_orders = self.client.get_cxos_for_user(&self.username).await?;

        let mut buy_rows = Vec::new();
        let mut sell_rows = Vec::new();

        let now = Utc::now();

        for order in own_orders {
            if order.status == "FILLED" {
                continue;
            }

            let material = get_material_db()
                .get(&order.material_ticker.as_str())
                .context("Material not found")?;

            let row = Row::new(vec![
                Cell::from(match order.order_type.as_str() {
                    "BUYING" => Span::styled("Buy", Style::default().fg(Color::Green)),
                    _ => Span::styled("Sell", Style::default().fg(Color::Red)),
                }),
                Cell::from(Span::styled(
                    order.material_ticker.to_string(),
                    get_style_for_material(&order.material_ticker),
                )),
                Cell::from(Span::styled(
                    material.name,
                    Style::default().fg(Color::White),
                )),
                Cell::from(Span::styled(
                    order.initial_amount.to_string(),
                    Style::default().fg(Color::White),
                )),
                Cell::from(Span::styled(
                    order.amount.to_string(),
                    Style::default().fg(Color::White),
                )),
                Cell::from(Span::styled(
                    format!("{}", order.limit),
                    Style::default().fg(Color::White),
                )),
                Cell::from(Span::raw(format_price(
                    order.limit * order.initial_amount as f32,
                ))),
                Cell::from(Span::raw(order.limit_currency)),
                Cell::from(Span::styled(
                    match order.status.as_str() {
                        "FILLED" => "Filled",
                        "PLACED" => "Placed",
                        "PARTIALLY_FILLED" => "Partially filled",
                        _ => "Unknown",
                    },
                    Style::default().fg(Color::White),
                )),
                Cell::from(Span::raw({
                    let duration = now - order.created;

                    format!(
                        "{} days {} hours",
                        duration.num_days(),
                        (duration - chrono::Duration::days(duration.num_days())).num_hours()
                    )
                })),
            ]);

            if order.order_type == "BUYING" {
                buy_rows.push(row);
            } else {
                sell_rows.push(row);
            }
        }

        if self.want_depth_widget && self.depth_widget.is_none() {
            let mut dw = DepthChartWidget::default();

            let idx = self.table_state.selected().unwrap_or(0);
            // let row = buy_rows.iter().chain(sell_rows.iter()).nth(idx).unwrap();

            let ticker = self
                .client
                .get_exchange_info(&format!("{}.{}", "SF", "CI1"))
                .await?;
            dw.update_with_ticker(&ticker);
            self.depth_widget = Some(dw);
        } else if !self.want_depth_widget && self.depth_widget.is_some() {
            self.depth_widget = None;
        }

        self.scrollbar_state = self
            .scrollbar_state
            .content_length(buy_rows.len() + sell_rows.len());
        self.buy_rows = buy_rows;
        self.sell_rows = sell_rows;

        Ok(())
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect, current_widget: WidgetEnum) {
        if let Some(depth_widget) = &mut self.depth_widget {
            // let area = centered_rect(90, 90, area);

            // frame.render_widget(Clear, area);
            depth_widget.render(frame, area, current_widget);

            return;
        }

        let widths = [
            Constraint::Length(6),  // Order type
            Constraint::Length(6),  // Ticker
            Constraint::Length(20), // Name
            Constraint::Length(6),  // Initial amount
            Constraint::Length(6),  // Amount
            Constraint::Length(10), // Limit
            Constraint::Length(10), // Total
            Constraint::Length(3),  // Currency
            Constraint::Length(16), // Status
            Constraint::Fill(1),    // Age
        ];

        let table = Table::new(
            self.buy_rows.iter().chain(self.sell_rows.iter()).cloned(),
            widths,
        )
        .header(
            Row::new(vec![
                Cell::from("Order type"),
                Cell::from("Ticker"),
                Cell::from("Name"),
                Cell::from("Initial amount"),
                Cell::from("Remaining amount"),
                Cell::from("Limit"),
                Cell::from("Total"),
                Cell::from("Currency"),
                Cell::from("Status"),
                Cell::from("Age"),
            ])
            .add_modifier(Modifier::BOLD),
        )
        .highlight_style(
            Style::default().fg(if current_widget == WidgetEnum::Market {
                Color::Indexed(14)
            } else {
                Color::White
            }),
        )
        .highlight_spacing(ratatui::widgets::HighlightSpacing::Always)
        .highlight_symbol(">")
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Market Orders"),
        );

        frame.render_stateful_widget(table, area, &mut self.table_state);
        if area.height - 3 < (self.sell_rows.len() + self.buy_rows.len()) as u16 {
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
}

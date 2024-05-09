use anyhow::Context;
use crossterm::event::Event;
use fiors::{get_material_db, FIOClient};
use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Cell, Padding, Row, Table},
    Frame,
};

use crate::{format_amount, NeedRefresh};

use super::{SharedWidgetState, WidgetEnum};

struct Grid<T> {
    cells: Vec<Vec<T>>,
    cols: usize,
}

impl<T> Grid<T> {
    fn new(cols: usize) -> Self {
        Self {
            cells: Vec::new(),
            cols,
        }
    }

    fn push(&mut self, item: T) {
        if self.cells.is_empty() {
            self.cells.push(Vec::new());
        }

        let last = self.cells.last_mut().unwrap();
        last.push(item);

        if last.len() == self.cols {
            self.cells.push(Vec::new());
        }
    }

    fn iter_rows(&self) -> impl Iterator<Item = &[T]> {
        self.cells.iter().map(Vec::as_slice)
    }
}

struct InvItem {
    name: String,
    amount: u32,
    value: f32,
    bg_color: Color,
    fg_color: Color,
}

pub struct InventoryWidget {
    client: &'static FIOClient,
    username: String,
    planet_id: String,
    inv_items: Vec<InvItem>,
}

impl InventoryWidget {
    pub fn new(client: &'static FIOClient, username: &str, planet_id: &str) -> Self {
        Self {
            client,
            username: username.to_string(),
            planet_id: planet_id.to_string(),
            inv_items: Vec::new(),
        }
    }

    pub fn switch_planets(&mut self, planet_id: &str) {
        self.planet_id = planet_id.to_string();
        self.inv_items.clear();
    }

    /// Return true if we scrolled and need to redraw the widget
    pub fn handle_input(&mut self, event: &Event, current_widget: WidgetEnum) -> NeedRefresh {
        NeedRefresh::No
    }

    #[tracing::instrument(name="inventory::update", skip(self, shared_state), fields(planet_id=self.planet_id))]
    pub async fn update(&mut self, shared_state: &mut SharedWidgetState) -> anyhow::Result<()> {
        let inv = self
            .client
            .get_storage_for_user(&self.username, &self.planet_id)
            .await?
            .context("No inventory found")?;

        let mut sorted_inv = inv.items.values().collect::<Vec<_>>();
        sorted_inv.sort_by_key(|item| {
            let matinfo = get_material_db()
                .get(item.ticker.as_str())
                .expect("Unknown item");
            matinfo.category
        });

        self.inv_items = sorted_inv
            .into_iter()
            .map(|item| {
                let matinfo = get_material_db()
                    .get(item.ticker.as_str())
                    .expect("Unknown item");

                let bg_color = matinfo.category.get_bg_color();
                let fg_color = matinfo.category.get_fg_color();

                let bg_color = Color::Rgb(bg_color.0, bg_color.1, bg_color.2);
                let fg_color = Color::Rgb(fg_color.0, fg_color.1, fg_color.2);

                // let cx = self.client.get_exchange_info(format!("")).await?;

                InvItem {
                    name: item.ticker.clone(),
                    amount: item.quantity,
                    value: 0.0,
                    bg_color,
                    fg_color,
                }
            })
            .collect();

        shared_state
            .help_text
            .extend(vec![Span::raw("This page shows your inventory. ")]);
        Ok(())
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect, _current_widget: WidgetEnum) {
        let cols = ((area.width - 3) as f32 / 10.0).floor() as usize;
        tracing::trace!(cols, area.width);
        let mut grid = Grid::new(cols);

        for item in &self.inv_items {
            grid.push(item);
        }

        let mut rows = Vec::new();
        for grid_row in grid.iter_rows() {
            let cells: Vec<_> = grid_row
                .iter()
                .map(|item| {
                    let bg_color = item.bg_color;
                    let fg_color = item.fg_color;

                    Cell::new(Text::from(vec![
                        Line::from(item.name.clone())
                            .centered()
                            .style(Style::default().fg(fg_color).bg(bg_color)),
                        Line::from(item.amount.to_string())
                            .centered()
                            .style(Style::default().fg(fg_color).bg(bg_color)),
                        Line::from(""),
                    ]))
                })
                .collect();
            rows.push(Row::new(cells).height(3).bottom_margin(0));
        }

        let widths = vec![Constraint::Length(5); cols];

        let table = Table::new(rows, widths)
            .flex(ratatui::layout::Flex::SpaceBetween)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Inventory")
                    .padding(Padding::uniform(1)),
            );

        frame.render_widget(table, area);
    }
}

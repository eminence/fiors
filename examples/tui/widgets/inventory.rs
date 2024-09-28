use crossterm::event::Event;
use fiors::{get_material_db, types::StorageType, FIOClient};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Cell, Padding, Row, Table},
    Frame,
};

use crate::NeedRefresh;

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
    warehouse_items: Vec<InvItem>,
    inv_weight_load: f32,
    inv_volume_load: f32,
    warehouse_weight_load: f32,
    warehouse_volume_load: f32,
}

impl InventoryWidget {
    pub fn new(client: &'static FIOClient, username: &str, planet_id: &str) -> Self {
        Self {
            client,
            username: username.to_string(),
            planet_id: planet_id.to_string(),
            inv_items: Vec::new(),
            warehouse_items: Vec::new(),
            inv_weight_load: 0.0,
            inv_volume_load: 0.0,
            warehouse_weight_load: 0.0,
            warehouse_volume_load: 0.0,
        }
    }

    pub fn switch_planets(&mut self, planet_id: &str) {
        self.planet_id = planet_id.to_string();
        self.inv_items.clear();
    }

    /// Return true if we scrolled and need to redraw the widget
    pub fn handle_input(&mut self, _event: &Event, _current_widget: WidgetEnum) -> NeedRefresh {
        NeedRefresh::No
    }

    #[tracing::instrument(name="inventory::update", skip(self, shared_state), fields(planet_id=self.planet_id))]
    pub async fn update(&mut self, shared_state: &mut SharedWidgetState) -> anyhow::Result<()> {
        let planet_site = self
            .client
            .get_planetsite_for_user(&self.username, &self.planet_id)
            .await?;
        tracing::trace!(?planet_site);

        let all_inv = self.client.get_all_storage_for_user(&self.username).await?;

        let warehouse_info = self
            .client
            .get_warehouse_info_for_user(&self.username)
            .await?;

        let warehouse_id = warehouse_info
            .iter()
            .find(|w| w.location_natural_id == planet_site.planet_identifier);

        // iterate through our storage and look for both base storage and warehouse storage
        let inv = all_inv
            .iter()
            .find(|inv| {
                inv.storage_type == StorageType::Store && inv.addressable_id == planet_site.site_id
            })
            .ok_or_else(|| anyhow::anyhow!("No inventory found for planet"))?;

        self.inv_volume_load = inv.volume_load / inv.volume_capacity;
        self.inv_weight_load = inv.weight_load / inv.weight_capacity;

        let mut sorted_inv = inv
            .items
            .values()
            .filter(|item| !item.ticker.is_empty())
            .collect::<Vec<_>>();
        sorted_inv.sort_by_key(|item| {
            let matinfo = get_material_db()
                .get(item.ticker.as_str())
                .ok_or_else(|| anyhow::anyhow!("Unknown item {item:?}"))
                .unwrap();
            matinfo.category
        });

        tracing::trace!(
            "all_inv_len={} warehouse_id={:?}",
            all_inv.len(),
            warehouse_id
        );
        if let Some(whid) = warehouse_id {
            let inv = all_inv
                .iter()
                .find(|inv| inv.storage_id == whid.store_id)
                .ok_or_else(|| anyhow::anyhow!("No inventory found for planet"))?;

            tracing::trace!("we have warehouse storage data {:?}", inv);
            self.warehouse_volume_load = inv.volume_load / inv.volume_capacity;
            self.warehouse_weight_load = inv.weight_load / inv.weight_capacity;

            let mut sorted_inv = inv
                .items
                .values()
                .filter(|item| !item.ticker.is_empty())
                .collect::<Vec<_>>();
            sorted_inv.sort_by_key(|item| {
                let matinfo = get_material_db()
                    .get(item.ticker.as_str())
                    .ok_or_else(|| anyhow::anyhow!("Unknown item {item:?}"))
                    .unwrap();
                matinfo.category
            });

            self.warehouse_items = sorted_inv
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
        } else {
            self.warehouse_items.clear();
        }

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

        let mut inv_grid = Grid::new(cols);

        for item in &self.inv_items {
            inv_grid.push(item);
        }

        let inv_rows: Vec<_> = inv_grid
            .iter_rows()
            .map(|grid_row| {
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
                Row::new(cells).height(3).bottom_margin(0)
            })
            .collect();

        let mut warehouse_grid = Grid::new(cols);

        for item in &self.warehouse_items {
            warehouse_grid.push(item);
        }

        let warehouse_rows: Vec<_> = warehouse_grid
            .iter_rows()
            .map(|grid_row| {
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
                Row::new(cells).height(3).bottom_margin(0)
            })
            .collect();

        // the inventory to warehouse display ration is based on the number of items in each
        let inv_rows_count = inv_grid.iter_rows().count() as u32;
        let warehouse_rows_count = warehouse_grid.iter_rows().count() as u32;
        let total_rows = inv_rows_count + warehouse_rows_count;

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints(
                [
                    Constraint::Ratio(inv_rows_count, total_rows), // main inventory
                    Constraint::Ratio(warehouse_rows_count, total_rows), // warehouse
                ]
                .as_ref(),
            )
            .split(area);

        let widths = vec![Constraint::Length(5); cols];

        let table = Table::new(inv_rows, &widths)
            .flex(ratatui::layout::Flex::SpaceBetween)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Inventory")
                    .title(format!(
                        "Wgt: {:.0}% Vol: {:.0}%",
                        self.inv_weight_load * 100.0,
                        self.inv_volume_load * 100.0
                    ))
                    .padding(Padding::uniform(1)),
            );

        frame.render_widget(table, chunks[0]);

        // render the warehouse inventory:

        if !self.warehouse_items.is_empty() {
            let table = Table::new(warehouse_rows, &widths)
                .flex(ratatui::layout::Flex::SpaceBetween)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Warehouse")
                        .title(format!(
                            "Wgt: {:.0}% Vol: {:.0}%",
                            self.inv_weight_load * 100.0,
                            self.inv_volume_load * 100.0
                        ))
                        .padding(Padding::uniform(1)),
                );

            frame.render_widget(table, chunks[1]);
        }
    }
}

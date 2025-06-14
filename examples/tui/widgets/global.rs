use std::collections::{HashMap, HashSet};

use crossterm::event::Event;
use fiors::{get_recipe_db, FIOClient};
use ratatui::{layout::{Constraint, Rect}, style::{Color, Modifier, Style, Stylize}, widgets::{Block, Borders, Cell, Row, Table, TableState}, Frame};

use crate::{format_amount, NeedRefresh};

use super::{production, SharedWidgetState, WidgetEnum};

enum MaterialState {
    /// A material that we need to acquire from the market
    Acquire(u32),
    /// A material that we need to produce (and where we can produce it)
    Produce(u32, String),
}

pub struct GalacticInvWidget {
    client: &'static FIOClient,
    username: String,

    /// A list of materials that we need to acquire or produce
    materials: Vec<(String, MaterialState)>,
    table_state: TableState,

}
impl GalacticInvWidget {
    pub(crate) fn new(client: &'static fiors::FIOClient, username: &str) -> Self {
        Self {
            client,
            username: username.to_string(),
            materials: vec![],
            table_state: Default::default(),
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect, current_widget: WidgetEnum) {

        let mut rows = Vec::new();
        for (mat, state) in &self.materials {
            let cells = vec![
                Cell::from(mat.as_str()),
                Cell::from(match state {
                    MaterialState::Acquire(x) => x.to_string(),
                    MaterialState::Produce(x, _) => format_amount(*x as f32),
                }),
                Cell::from(match state {
                    MaterialState::Acquire(_) => "Market",
                    MaterialState::Produce(_, x) => x.as_str(),
                }),
            ];
            rows.push(Row::new(cells));
        }

        let widths = [
            Constraint::Length(4),  // Ticker
            Constraint::Length(10), // Ammount
            Constraint::Fill(1),
        ];
        
        let table = Table::new(
            rows,
            widths,
        )
        .header(
            Row::new(vec![
                Cell::from("Ticker"),
                Cell::from("Ammount"),
                Cell::from("Where")
            ])
            .add_modifier(Modifier::BOLD),
        )
        .highlight_style(
            Style::default().fg(if current_widget == WidgetEnum::Global {
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
                .title("Needed Materials"),
        );
        frame.render_stateful_widget(table, area, &mut self.table_state);


    }

    /// Return true if we scrolled and need to redraw the widget
    pub fn handle_input(&mut self, event: &Event, current_widget: WidgetEnum) -> NeedRefresh {
        NeedRefresh::No
    }

    #[tracing::instrument(name = "global::update", skip(self, shared_state))]
    pub async fn update(&mut self, shared_state: &mut SharedWidgetState) -> anyhow::Result<()> {

        let mut material_map = HashMap::new();
        let mut needs = Vec::new();
        /// A map from material to the planet(s) where it is produced
        let mut production_map: HashMap<String, HashSet<String>> = HashMap::new();

        let all_inv = self.client.get_all_storage_for_user(&self.username).await?;
        for store in all_inv {
            for (mat, item) in store.items {
                let e = material_map.entry(mat).or_insert(0);
                *e += item.quantity;
            }
        }

        // before looking at what we need, get a map of all production lines on each planet
        let db = get_recipe_db();

        for (planet_id, planet_name) in &shared_state.planet_id_map {
            let production_lines = self.client.get_planet_production(&self.username, &planet_id).await?;
            let all_buildings = production_lines.into_iter().map(|pl| pl.building_ticker());
            for building_ticker in all_buildings {
                for reciepe in db.iter().filter(|r| r.building_ticker == building_ticker) {
                    for mat in reciepe.outputs {
                        production_map.entry(mat.ticker.to_string()).or_default().insert(planet_name.clone());
                    }
                }
            }
        }

        for (needed_mat, needed_amt) in &shared_state.overrides.galactic_needs {
            let have_in_inv = material_map.get(needed_mat.as_str()).unwrap_or(&0);
            if needed_amt > have_in_inv {
                if let Some(x) = production_map.get(needed_mat) {
                    needs.push((needed_mat.clone(), MaterialState::Produce(needed_amt - have_in_inv, format!("{x:?}"))));
                } else {
                    needs.push((needed_mat.clone(), MaterialState::Acquire(needed_amt - have_in_inv)));
                }
            }
        }

        needs.sort_by(|a, b| a.0.cmp(&b.0));

        self.materials = needs;
    

        Ok(())
    }
}

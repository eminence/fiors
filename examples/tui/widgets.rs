mod lm_widget;
use std::collections::HashMap;

use crossterm::event::{Event, KeyCode, KeyEvent, MouseEvent, MouseEventKind};
pub use lm_widget::LocalMarketWidget;

mod production;
pub use production::ProductionWidget;

mod buildings;
pub use buildings::BuildingsWidget;

mod debug;
pub use debug::DebugWidget;

mod inventory;
pub use inventory::InventoryWidget;

use ratatui::text::Span;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum WidgetEnum {
    Production,
    LocalMarket,
    Consumption,
    LMNotes,
    Needs,
    Buildings,
    Inventory,
}

impl WidgetEnum {
    pub fn next(self) -> Self {
        match self {
            Self::Production => Self::LocalMarket,
            Self::LocalMarket => Self::Consumption,
            Self::Consumption => Self::LMNotes,
            Self::LMNotes => Self::Needs,
            Self::Needs => Self::Production,
            Self::Buildings => Self::Buildings,
            Self::Inventory => Self::Inventory,
        }
    }
    pub fn prev(self) -> Self {
        match self {
            Self::Production => Self::Needs,
            Self::LocalMarket => Self::Production,
            Self::Consumption => Self::LocalMarket,
            Self::LMNotes => Self::Consumption,
            Self::Needs => Self::LMNotes,
            Self::Buildings => Self::Buildings,
            Self::Inventory => Self::Inventory,
        }
    }
}

fn handle_scroll(event: &Event, selected: Option<usize>, table_row_len: usize) -> Option<usize> {
    let scroll_amount: i32 = match event {
        Event::Key(KeyEvent {
            code: KeyCode::PageUp,
            ..
        }) => -5,
        Event::Key(KeyEvent {
            code: KeyCode::PageDown,
            ..
        }) => 5,
        Event::Key(KeyEvent {
            code: KeyCode::Up, ..
        }) => -1,
        Event::Key(KeyEvent {
            code: KeyCode::Down,
            ..
        }) => 1,
        Event::Key(KeyEvent {
            code: KeyCode::Home,
            ..
        }) => i32::MIN,
        Event::Key(KeyEvent {
            code: KeyCode::End, ..
        }) => i32::MAX,
        Event::Mouse(MouseEvent {
            kind: MouseEventKind::ScrollDown,
            ..
        }) => 1,
        Event::Mouse(MouseEvent {
            kind: MouseEventKind::ScrollUp,
            ..
        }) => -1,
        Event::Key(KeyEvent {
            code: KeyCode::Esc, ..
        }) => return None,
        _ => return selected,
    };

    if selected.is_none() {
        return Some(0);
    }

    let i = selected.unwrap_or(0);
    let new_i = if scroll_amount == i32::MAX {
        table_row_len - 1
    } else if scroll_amount > 0 {
        i.saturating_add(scroll_amount as usize)
            .min(table_row_len - 1)
    } else if scroll_amount == i32::MIN {
        0
    } else if scroll_amount < 0 {
        i.saturating_sub(scroll_amount.unsigned_abs() as usize)
    } else {
        i
    };

    Some(new_i)
}

/// State that is shared between all widgets
#[derive(Debug, Default)]
pub struct SharedWidgetState {
    /// A map from planet_id to planet_name
    pub planet_id_map: HashMap<String, String>,

    /// For each planet_id, A map between a material and how much we need per day
    pub needs: HashMap<String, HashMap<String, f32>>,

    /// For each planet_id, a map between a material and how much we have in excess
    pub excess: HashMap<String, HashMap<String, f32>>,

    /// The best COGM for each material
    pub cogm: HashMap<String, f32>,

    // To be displayed in the debug window
    // pub debug_messages: Vec<String>,
    pub help_text: Vec<Span<'static>>,
}

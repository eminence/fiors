mod lm_widget;
use std::collections::HashMap;

use crossterm::event::{Event, KeyCode, KeyEvent, MouseEvent, MouseEventKind};
pub use lm_widget::LocalMarketWidget;

mod production;
pub use production::ProductionWidget;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum WidgetEnum {
    Production,
    LocalMarket,
    Consumption,
    LMNotes,
    Needs,
}

impl WidgetEnum {
    pub fn next(self) -> Self {
        match self {
            Self::Production => Self::LocalMarket,
            Self::LocalMarket => Self::Consumption,
            Self::Consumption => Self::LMNotes,
            Self::LMNotes => Self::Needs,
            Self::Needs => Self::Production,
        }
    }
    pub fn prev(self) -> Self {
        match self {
            Self::Production => Self::Needs,
            Self::LocalMarket => Self::Production,
            Self::Consumption => Self::LocalMarket,
            Self::LMNotes => Self::Consumption,
            Self::Needs => Self::LMNotes,
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
        i.saturating_sub(scroll_amount.abs() as usize)
    } else {
        i
    };

    Some(new_i)
}

/// State that is shared between all widgets
#[derive(Debug, Default)]
pub struct SharedWidgetState {
    /// A map between a material and how much we need for a 3 week supply
    pub needs: HashMap<String, f32>,
}

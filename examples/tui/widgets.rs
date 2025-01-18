mod lm_widget;
use std::{collections::HashMap, time::Instant};

use crossterm::event::{Event, KeyCode, KeyEvent, MouseEvent, MouseEventKind};
pub use lm_widget::LocalMarketWidget;

mod production;
pub use production::ProductionWidget;

mod buildings;
pub use buildings::BuildingsWidget;

mod inventory;
pub use inventory::InventoryWidget;

mod market;
pub use market::MarketWidget;

mod depth;
pub use depth::DepthChartWidget;

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
    Market,
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
            Self::Market => Self::Market,
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
            Self::Market => Self::Market,
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

#[derive(Debug, Copy, Clone)]
pub enum OverrideType {
    /// Hold this much, even if our production needs are higher or lower
    Absolute(u32),
    /// Hold at least this much, or more if our production needs are higher
    Minimum(u32),
    /// Hold exactly this much.  Any excess needs to be shipped off planet
    Maxiumum(u32),
    None,
}

impl OverrideType {
    pub fn with(&self, current: f32) -> f32 {
        match self {
            Self::Absolute(x) => *x as f32,
            Self::Minimum(x) => current.max(*x as f32),
            Self::Maxiumum(x) => current.min(*x as f32),
            Self::None => current,
        }
    }
    pub fn as_value(&self) -> f32 {
        match self {
            Self::Absolute(x) => *x as f32,
            Self::Minimum(x) => *x as f32,
            Self::Maxiumum(x) => *x as f32,
            Self::None => 0.0,
        }
    }
}

impl Default for OverrideType {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug)]
pub struct Overrides {
    /// How much of a given material we should keep in our inventory
    pub planet_materials: HashMap<String, HashMap<String, OverrideType>>,

    /// How often we should resupply a planet
    pub planet_resupply_period: HashMap<String, i32>,

    /// When we last loaded these overrides from disk
    pub last_updated: Instant,
}

impl Default for Overrides {
    fn default() -> Self {
        Self {
            planet_materials: Default::default(),
            planet_resupply_period: Default::default(),
            last_updated: Instant::now(),
        }
    }
}

impl Overrides {
    pub fn read() -> anyhow::Result<Self> {
        let x: toml::Table = toml::from_str(std::fs::read_to_string("override.toml")?.as_str())?;

        let mut planet_materials: HashMap<String, HashMap<String, OverrideType>> = HashMap::new();
        let mut planet_resupply_period = HashMap::new();

        for (planet, data) in x {
            let toml::Value::Table(data) = data else {
                continue;
            };
            if let Some(toml::Value::Table(materials)) = data.get("materials") {
                let m = planet_materials.entry(planet.clone()).or_default();
                m.extend(materials.into_iter().filter_map(|(k, v)| {
                    if let toml::Value::Integer(v) = v {
                        Some((k.to_string(), OverrideType::Minimum(*v as u32)))
                    } else {
                        None
                    }
                }));
            }
            if let Some(toml::Value::Table(materials)) = data.get("materials-override") {
                let m = planet_materials.entry(planet.clone()).or_default();
                m.extend(materials.into_iter().filter_map(|(k, v)| {
                    if let toml::Value::Integer(v) = v {
                        Some((k.to_string(), OverrideType::Absolute(*v as u32)))
                    } else {
                        None
                    }
                }));
            }
            if let Some(toml::Value::Table(materials)) = data.get("materials-max") {
                let m = planet_materials.entry(planet.clone()).or_default();
                m.extend(materials.into_iter().filter_map(|(k, v)| {
                    if let toml::Value::Integer(v) = v {
                        Some((k.to_string(), OverrideType::Maxiumum(*v as u32)))
                    } else {
                        None
                    }
                }));
            }
            if let Some(toml::Value::Integer(i)) = data.get("resupply") {
                planet_resupply_period.insert(planet.clone(), *i as i32);
            }
        }

        Ok(Self {
            planet_materials,
            planet_resupply_period,
            last_updated: Instant::now(),
        })
    }
}

/// State that is shared between all widgets
#[derive(Debug, Default)]
pub struct SharedWidgetState {
    /// A map from planet_id to planet_name
    pub planet_id_map: HashMap<String, String>,

    /// For each planet_id, A map between a material and how much we need per day
    pub needs: HashMap<String, HashMap<String, f32>>,

    /// For each planet_id, a map between a material and how much we have in excess in storage
    pub excess: HashMap<String, HashMap<String, f32>>,

    /// The best COGM for each material
    pub cogm: HashMap<String, f32>,

    // To be displayed in the debug window
    // pub debug_messages: Vec<String>,
    pub help_text: Vec<Span<'static>>,

    pub overrides: Overrides,
}

#[test]
fn test_toml() {
    let x = Overrides::read().unwrap();
    dbg!(x);
}

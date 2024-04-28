use std::{collections::HashMap, ops::AddAssign, time::Duration};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer};
use tracing::trace;

use crate::get_building_db;

#[derive(Debug, Clone)]
pub struct Storage {
    pub name: Option<String>,
    /// Addressable ID
    ///
    /// This ID will match the SiteId of a planet
    pub addressable_id: String,
    /// Storage ID
    ///
    /// This ID can be used in-game as an argument to the `INV` command
    pub storage_id: String,
    pub storage_type: StorageType,
    pub items: HashMap<String, Item>,
}

#[derive(Debug, Copy, Clone)]
pub enum StorageType {
    Store,
    Warehouse,
    ShipStore,
    StlFuelStore,
    FtlFuelStore,
}

#[derive(Debug, Clone)]
pub struct Item {
    pub ticker: String,
    pub quantity: u32,
    pub total_weight: f32,
    pub total_volume: f32,
}

impl StorageType {
    pub fn from_str(s: &str) -> Self {
        match s {
            "STORE" => Self::Store,
            "WAREHOUSE_STORE" => Self::Warehouse,
            "SHIP_STORE" => Self::ShipStore,
            "STL_FUEL_STORE" => Self::StlFuelStore,
            "FTL_FUEL_STORE" => Self::FtlFuelStore,
            x => panic!("Unknown inventory stype {x:?}"),
        }
    }
}

impl Storage {
    pub(crate) fn from_json(v: serde_json::Value) -> anyhow::Result<Self> {
        #[derive(Deserialize)]
        #[allow(unused)]
        struct Inner {
            #[serde(rename = "Name")]
            name: Option<String>,
            #[serde(rename = "AddressableId")]
            addressable_id: String,
            #[serde(rename = "StorageId")]
            storage_id: String,
            #[serde(rename = "FixedStore")]
            fixed_store: bool,
            #[serde(rename = "Type")]
            r#type: String,
            #[serde(rename = "StorageItems")]
            storage_items: Vec<InnerItem>,
        }
        #[derive(Deserialize)]
        #[allow(unused)]
        struct InnerItem {
            #[serde(rename = "MaterialAmount")]
            material_amount: u32,
            #[serde(rename = "MaterialName")]
            material_name: Option<String>,
            #[serde(rename = "MaterialTicker")]
            material_ticker: Option<String>,

            #[serde(rename = "TotalWeight")]
            total_weight: f32,
            #[serde(rename = "TotalVolume")]
            total_volume: f32,

            /// Can be "INVENTORY" or "BLOCKED"
            #[serde(rename = "Type")]
            r#type: String,
        }

        let inner: Inner = serde_json::from_value(v)?;

        Ok(Self {
            name: inner.name,
            storage_type: StorageType::from_str(&inner.r#type),
            addressable_id: inner.addressable_id,
            storage_id: inner.storage_id,
            items: inner
                .storage_items
                .into_iter()
                .filter_map(|item| {
                    if item.r#type == "BLOCKED" {
                        return None;
                    }
                    let name = item.material_ticker.clone().unwrap_or_default();
                    Some((
                        name,
                        Item {
                            ticker: item.material_ticker.unwrap_or_default(),
                            quantity: item.material_amount,
                            total_volume: item.total_volume,
                            total_weight: item.total_weight,
                        },
                    ))
                })
                .collect(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Ticker {
    /// The full ticker name like "SF.CI1"
    pub name: String,
    pub currency: String,
    /// The average buy/sell price
    pub price: Option<f32>,
    /// The current ask (sell) price
    ///
    /// This is the current low price for oepn sell orders.
    ///
    /// If this is `None`, then there are no sellers.
    pub ask: Option<f32>,
    /// The current bid (buy request) price
    ///
    /// This is the currnet high price for open buy orders
    ///
    /// If this is `None`, then there are no buyers.
    pub bid: Option<f32>,

    /// The highest price seen in the past 24 hours
    pub high: Option<f32>,
    /// The lowest price seen in the past 24 hours
    pub low: Option<f32>,

    pub buying_orders: Vec<MarketOrder>,
    pub selling_orders: Vec<MarketOrder>,
}

impl Ticker {
    /// For when you really just need some price for this thing and aren't picky about what kind of price
    pub fn get_any_price(&self) -> Option<f32> {
        self.price
            .or(self.ask)
            .or(self.bid)
            .or(self.high)
            .or(self.low)
    }
}

/// Info about an instant buy or sell order
#[derive(Debug, Copy, Clone)]
pub struct InstantOrder {
    /// The total value of the order
    ///
    /// For buy orders, this is how much it would cost.
    /// For sell orders, this is how much you would get.
    pub total_value: f32,
    /// The price limit you would need to instantly buy or sell
    pub price_limit: f32,
}

impl Ticker {
    /// Returns the total cost to buy `quantity` units of this ticker
    ///
    /// Returns `None` if there are not enough sell orders to buy the requested quantity
    pub fn instant_buy(&self, mut quantity: u32) -> Option<InstantOrder> {
        // sort the sell orders, with lowest orders at the end
        let mut orders = self.selling_orders.clone();
        orders.sort_by(|a, b| a.item_cost.partial_cmp(&b.item_cost).unwrap());
        orders.reverse();

        let mut price = 0.0;
        while let Some(order) = orders.pop() {
            let Some(item_count) = order.item_count else {
                // this is a market maker with unlimited stock, so it can
                // fullfill the entire order
                price += order.item_cost * quantity as f32;
                return Some(InstantOrder {
                    total_value: price,
                    price_limit: order.item_cost,
                });
            };
            if item_count >= quantity {
                price += order.item_cost * quantity as f32;
                return Some(InstantOrder {
                    total_value: price,
                    price_limit: order.item_cost,
                });
            } else {
                price += order.item_cost * item_count as f32;
                quantity -= item_count;
            }
        }

        None
    }

    /// Returns
    pub fn instant_sell(&self, mut quantity: u32) -> Option<InstantOrder> {
        // sort the buy orders, with highest orders at the end
        let mut orders = self.buying_orders.clone();
        orders.sort_by(|a, b| a.item_cost.partial_cmp(&b.item_cost).unwrap());

        let mut price = 0.0;
        while let Some(order) = orders.pop() {
            let Some(item_count) = order.item_count else {
                // this is a market maker with unlimited stock, so it can
                // fullfill the entire order
                price += order.item_cost * quantity as f32;
                return Some(InstantOrder {
                    total_value: price,
                    price_limit: order.item_cost,
                });
            };
            if item_count >= quantity {
                price += order.item_cost * quantity as f32;
                return Some(InstantOrder {
                    total_value: price,
                    price_limit: order.item_cost,
                });
            } else {
                price += order.item_cost * item_count as f32;
                quantity -= item_count;
            }
        }

        None
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct MarketOrder {
    /// Company Code
    ///
    /// Might be null if a company has been liquidated
    #[serde(rename = "CompanyCode")]
    pub company_code: Option<String>,
    #[serde(rename = "CompanyName")]
    pub company_name: String,

    /// The price per unit
    #[serde(rename = "ItemCost")]
    pub item_cost: f32,

    /// The total number of units
    ///
    /// If none, then this is a market maker with unlimited stock
    #[serde(rename = "ItemCount")]
    pub item_count: Option<u32>,
}

impl Ticker {
    pub(crate) fn from_json(v: serde_json::Value) -> anyhow::Result<Self> {
        #[derive(Deserialize)]
        #[allow(unused)]
        #[serde(rename_all = "PascalCase")]
        struct Inner {
            material_ticker: String,
            exchange_code: String,
            currency: String,
            ask: Option<f32>,
            bid: Option<f32>,

            high: Option<f32>,
            low: Option<f32>,
            price: Option<f32>,

            buying_orders: Vec<MarketOrder>,
            selling_orders: Vec<MarketOrder>,
        }

        let inner: Inner = serde_json::from_value(v)?;

        Ok(Self {
            name: format!("{}.{}", inner.material_ticker, inner.exchange_code),
            currency: inner.currency,
            ask: inner.ask,
            bid: inner.bid,
            buying_orders: inner.buying_orders,
            selling_orders: inner.selling_orders,
            high: inner.high,
            low: inner.low,
            price: inner.price,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Planet {
    pub name: String,
    /// The name of the form AB-123x
    pub natural_id: String,
    pub id: String,
    pub has_local_market: bool,
    pub local_market_fee_factor: f32,
}

impl Planet {
    pub(crate) fn from_json(v: serde_json::Value) -> anyhow::Result<Self> {
        #[derive(Deserialize)]
        #[allow(unused)]
        #[serde(rename_all = "PascalCase")]
        struct Inner {
            planet_id: String,
            planet_name: String,
            planet_natural_id: String,
            has_local_market: bool,
            local_market_fee_factor: f32,
        }

        let inner: Inner = serde_json::from_value(v)?;

        Ok(Self {
            name: inner.planet_name,
            natural_id: inner.planet_natural_id,
            id: inner.planet_id,
            has_local_market: inner.has_local_market,
            local_market_fee_factor: inner.local_market_fee_factor,
        })
    }
}

#[derive(Debug, Clone)]
pub struct PlanetWorkforce {
    pub planet_id: String,
    pub planet_natural_id: String,

    pub details: HashMap<&'static str, WorkforceDetails>,
    // pub pioneers: WorkforceDetails,
    // pub settlers: WorkforceDetails,
    // pub technicians: WorkforceDetails,
    // pub engineers: WorkforceDetails,
    // pub scientists: WorkforceDetails,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct WorkforceDetails {
    pub capacity: u32,
    pub population: u32,
    pub required: u32,
    pub satisfaction: f32,
    #[serde(rename = "WorkforceNeeds")]
    pub needs: Vec<WorkforceNeed>,
    #[serde(rename = "WorkforceTypeName")]
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct WorkforceNeed {
    pub essential: bool,
    #[serde(rename = "MaterialTicker")]
    pub ticker: String,
    pub satisfaction: f32,
    /// The total number of units needed per day, based on your current population
    pub units_per_interval: f32,

    /// The number of units needed per day per 100 workers
    pub units_per_one_hundred: f32,
}

impl PlanetWorkforce {
    pub const PIONEER: &'static str = "PIONEER";
    pub const SETTLER: &'static str = "SETTLER";
    pub const TECHNICIAN: &'static str = "TECHNICIAN";
    pub const ENGINEER: &'static str = "ENGINEER";
    pub const SCIENTIST: &'static str = "SCIENTIST";

    pub(crate) fn from_json(v: serde_json::Value) -> anyhow::Result<Self> {
        #[derive(Deserialize)]
        #[allow(unused)]
        #[serde(rename_all = "PascalCase")]
        struct Inner {
            planet_id: String,
            planet_name: String,
            planet_natural_id: String,

            workforces: Vec<WorkforceDetails>,
        }

        let inner: Inner = serde_json::from_value(v)?;

        let mut needs = HashMap::new();

        for details in inner.workforces {
            match details.name.as_str() {
                "PIONEER" => needs.insert(Self::PIONEER, details),
                "SETTLER" => needs.insert(Self::SETTLER, details),
                "TECHNICIAN" => needs.insert(Self::TECHNICIAN, details),
                "ENGINEER" => needs.insert(Self::ENGINEER, details),
                "SCIENTIST" => needs.insert(Self::SCIENTIST, details),
                other => panic!("Unknown workforce type {other:?}"),
            };
        }

        Ok(Self {
            planet_id: inner.planet_id,
            planet_natural_id: inner.planet_natural_id,
            details: needs,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct LocalMarket {
    #[serde(rename = "BuyingAds")]
    pub buying_ads: Vec<LocalMarketContract>,
    #[serde(rename = "SellingAds")]
    pub selling_ads: Vec<LocalMarketContract>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LocalMarketContract {
    pub delivery_time: u32,
    pub material_ticker: String,
    pub material_amount: u32,
    #[serde(rename = "Price")]
    pub total_price: f32,
    #[serde(rename = "PriceCurrency")]
    pub currency: String,
    pub creator_company_name: String,
    pub creator_company_code: String,
}

impl LocalMarket {
    pub(crate) fn from_json(v: serde_json::Value) -> anyhow::Result<Self> {
        Ok(serde_json::from_value(v)?)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialInfo {
    pub material_id: String,
    pub category_name: String,
    pub category_id: String,
    pub name: String,
    pub ticker: String,
    pub weight: f32,
    pub volume: f32,
}

impl MaterialInfo {
    pub(crate) fn from_json(v: serde_json::Value) -> anyhow::Result<Self> {
        Ok(serde_json::from_value(v)?)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ProductionLine {
    pub site_id: String,
    pub planet_id: String,
    pub planet_natural_id: String,
    pub planet_name: String,
    #[serde(rename = "Type")]
    pub building_type: String,
    pub capacity: u32,
    pub efficiency: f32,
    pub condition: f32,
    pub orders: Vec<ProductionOrderDetails>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ProductionOrderDetails {
    pub inputs: Vec<ProductionOrderMaterial>,
    pub outputs: Vec<ProductionOrderMaterial>,
    #[serde(rename = "CreatedEpochMs", deserialize_with = "ms_to_date")]
    pub created: DateTime<Utc>,
    #[serde(rename = "StartedEpochMs", deserialize_with = "optional_ms_to_date")]
    pub started: Option<DateTime<Utc>>,
    #[serde(rename = "DurationMs", deserialize_with = "ms_to_duration")]
    pub duration: Duration,
    pub completed_percentage: Option<f32>,
    #[serde(rename = "CompletionEpochMs", deserialize_with = "optional_ms_to_date")]
    pub completion: Option<DateTime<Utc>>,
    pub recurring: bool,
    pub standard_recipe_name: String,
}

impl ProductionOrderDetails {
    pub fn get_building_ticker(&self) -> &str {
        self.standard_recipe_name.split(':').next().unwrap()
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ProductionOrderMaterial {
    pub material_name: String,
    pub material_ticker: String,
    pub material_amount: u32,
}

#[derive(Debug)]
pub struct DailyProduction {
    // pub building_ticker: String,
    pub inputs: HashMap<String, f32>,
    pub outputs: HashMap<String, f32>,
}

impl ProductionLine {
    pub(crate) fn from_json(v: serde_json::Value) -> anyhow::Result<Self> {
        Ok(serde_json::from_value(v)?)
    }

    pub fn building_ticker(&self) -> &'static str {
        let db = get_building_db();
        db.get(self.building_type.as_str())
            .expect("Unknown building type")
            .ticker
    }

    /// For each building, calculate the daily production of inputs and outputs, across all queued orders
    pub fn daily_production(&self) -> DailyProduction {
        if self.orders.is_empty() {
            return DailyProduction {
                inputs: Default::default(),
                outputs: Default::default(),
            };
        }

        let queued_orders: Vec<_> = self
            .orders
            .iter()
            .filter(|order| order.started.is_none() && order.recurring)
            .collect();

        let total_duration_days = self
            .orders
            .iter()
            .filter(|order| order.started.is_none())
            .map(|order| order.duration.as_secs_f32())
            .sum::<f32>()
            / 86400.0;

        // let building_ticker = self.orders.first().unwrap().get_building_ticker();

        trace!(queued_orders.len = queued_orders.len(), total_duration_days = total_duration_days);

        // dbg!(&queued_orders);
        let mut total_inputs = HashMap::new();
        let mut total_outputs = HashMap::new();

        for order in queued_orders {
            let duration_days = order.duration.as_secs_f32() / 86400.0;
            let scale = duration_days / total_duration_days;
            trace!(duration_days, scale, order.outputs = ?order.outputs);

            for input_material in &order.inputs {
                let input_per_day =
                    scale * self.capacity as f32 * input_material.material_amount as f32
                        / duration_days;

                total_inputs
                    .entry(input_material.material_ticker.clone())
                    .or_insert(0.0)
                    .add_assign(input_per_day);
                // println!(
                //     "{} input {} per day: {input_per_day}",
                //     self.building_type, input_material.material_ticker
                // );
            }

            for output_material in &order.outputs {
                // note: this output per day already takes into account production line conditions, workforce efficiency, satisfaction, and experts
                let output_per_day =
                    scale * self.capacity as f32 * output_material.material_amount as f32
                        / duration_days;

                total_outputs
                    .entry(output_material.material_ticker.clone())
                    .or_insert(0.0)
                    .add_assign(output_per_day);
                // println!(
                //     "{} output {} per day: {output_per_day}",
                //     self.building_type, output_material.material_ticker
                // );
            }
        }

        DailyProduction {
            inputs: total_inputs,
            outputs: total_outputs,
            // building_ticker: building_ticker.to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BuildingInfo {
    pub name: String,
    pub ticker: String,
    pub expertise: Option<String>,
    pub pioneers: u32,
    pub settlers: u32,
    pub technicians: u32,
    pub engineers: u32,
    pub scientists: u32,
    pub area_cost: u32,
    pub building_costs: Vec<BuildingCost>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BuildingCost {
    pub amount: u32,
    pub commodity_ticker: String,
}

impl BuildingInfo {
    pub(crate) fn from_json(v: serde_json::Value) -> anyhow::Result<Self> {
        Ok(serde_json::from_value(v)?)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Recipe {
    pub building_ticker: String,
    pub recipe_name: String,
    pub standard_recipe_name: String,
    pub inputs: Vec<RecipeMaterial>,
    pub outputs: Vec<RecipeMaterial>,
    #[serde(rename = "TimeMs", deserialize_with = "ms_to_duration")]
    pub duration: Duration,
}

impl Recipe {
    pub(crate) fn from_json(v: serde_json::Value) -> anyhow::Result<Self> {
        Ok(serde_json::from_value(v)?)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RecipeMaterial {
    pub ticker: String,
    pub amount: u32,
}

#[cfg(feature = "gendb")]
impl quote::ToTokens for RecipeMaterial {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::quote;
        let ticker = self.ticker.as_str();
        let amt = self.amount;

        tokens.extend(quote! {
            StaticRecipeMaterial {
                ticker: #ticker,
                amount: #amt,
            }
        });
    }
}

#[cfg(feature = "gendb")]
impl quote::ToTokens for Recipe {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::quote;

        let building_ticker = self.building_ticker.as_str();
        let recipe_name = self.recipe_name.as_str();
        let standard_recipe_name = self.standard_recipe_name.as_str();
        let duration_ms = self.duration.as_millis() as u64;

        let inputs = &self.inputs;
        let outputs = &self.outputs;

        tokens.extend(quote! {
            StaticRecipeInfo {
                building_ticker: #building_ticker,
                recipe_name: #recipe_name,
                standard_recipe_name: #standard_recipe_name,
                duration: std::time::Duration::from_millis(#duration_ms),
                inputs: &[ #(#inputs),* ],
                outputs: &[ #(#outputs),* ],
            }
        });
    }
}

fn optional_ms_to_date<'de, D>(d: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<i64> = Option::deserialize(d)?;
    match s {
        Some(s) => Ok(Some(DateTime::from_timestamp_millis(s).ok_or_else(
            || serde::de::Error::custom("Failed to convert epochms to date"),
        )?)),
        None => Ok(None),
    }
}

fn ms_to_date<'de, D>(d: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: i64 = i64::deserialize(d)?;

    DateTime::from_timestamp_millis(s)
        .ok_or_else(|| serde::de::Error::custom("Failed to convert epochms to date"))
}

fn ms_to_duration<'de, D>(d: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let s: u64 = u64::deserialize(d)?;

    Ok(Duration::from_millis(s))
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::types::Ticker;

    use super::Storage;

    #[test]
    fn test_storage() {
        let data: serde_json::Value = serde_json::from_reader(
            std::fs::File::open("test_data/storage_eminence32.json").unwrap(),
        )
        .unwrap();

        if let Value::Array(list) = data {
            for obj in list.into_iter() {
                let sto = Storage::from_json(obj).unwrap();
                dbg!(sto);
            }
        }
    }

    #[test]
    fn test_ticker() {
        let data: serde_json::Value = serde_json::from_reader(
            std::fs::File::open("test_data/exchange_PWO_CI1.json").unwrap(),
        )
        .unwrap();

        let sfci1 = Ticker::from_json(data).unwrap();
        // dbg!(sfci1);

        assert_eq!(267.0, sfci1.instant_buy(1).unwrap().total_value);
        assert_eq!(267.0 * 2.0, sfci1.instant_buy(2).unwrap().total_value);
        assert_eq!(267.0 * 298.0, sfci1.instant_buy(298).unwrap().total_value);
        assert_eq!(
            267.0 * 298.0 + 2.0 * 270.0,
            sfci1.instant_buy(300).unwrap().total_value
        );

        let cotci1 = Ticker::from_json(
            serde_json::from_reader(
                std::fs::File::open("test_data/exchange_COT_CI1.json").unwrap(),
            )
            .unwrap(),
        )
        .unwrap();

        assert!(cotci1.instant_buy(1).is_none());
    }
}

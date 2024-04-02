use serde::Deserialize;

#[derive(Debug)]
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
    pub items: Vec<Item>,
}

#[derive(Debug, Copy, Clone)]
pub enum StorageType {
    Store,
    Warehouse,
    ShipStore,
    StlFuelStore,
    FtlFuelStore,
}

#[derive(Debug)]
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
                    Some(Item {
                        ticker: item.material_ticker.unwrap_or_default(),
                        quantity: item.material_amount,
                        total_volume: item.total_volume,
                        total_weight: item.total_weight,
                    })
                })
                .collect(),
        })
    }
}

#[derive(Debug)]
pub struct Ticker {
    /// The full ticker name like "SF.CI1"
    pub name: String,
    pub currency: String,
    /// The current ask (buy) price
    ///
    /// This is the current low price for oepn sell orders
    pub ask: f32,
    /// The current bid (sell request) price
    ///
    /// This is the currnet high price for open buy orders
    pub bid: f32,
}

impl Ticker {
    pub(crate) fn from_json(v: serde_json::Value) -> anyhow::Result<Self> {
        #[derive(Deserialize)]
        #[allow(unused)]
        struct Inner {
            #[serde(rename = "MaterialTicker")]
            material_ticker: String,
            #[serde(rename = "ExchangeCode")]
            exchange_code: String,
            #[serde(rename = "Currency")]
            currency: String,
            #[serde(rename = "Ask")]
            ask: f32,
            #[serde(rename = "Bid")]
            bid: f32,
        }

        let inner: Inner = serde_json::from_value(v)?;

        Ok(Self {
            name: format!("{}.{}", inner.material_ticker, inner.exchange_code),
            currency: inner.currency,
            ask: inner.ask,
            bid: inner.bid,
        })
    }
}

pub struct Planet {
    pub name: String,
    /// The name of the form AB-123x
    pub natural_id: String,
    pub id: String,
    pub has_local_market: bool,
}

impl Planet {
    pub(crate) fn from_json(v: serde_json::Value) -> anyhow::Result<Self> {
        #[derive(Deserialize)]
        #[allow(unused)]
        struct Inner {
            #[serde(rename = "PlanetId")]
            planet_id: String,
            #[serde(rename = "PlanetName")]
            planet_name: String,
            #[serde(rename = "PlanetNaturalId")]
            planet_natural_id: String,
            #[serde(rename = "HasLocalMarket")]
            has_local_market: bool,
            #[serde(rename = "LocalMarketFeeFactor")]
            local_market_fee_factor: f32,
        }

        let inner: Inner = serde_json::from_value(v)?;

        Ok(Self {
            name: inner.planet_name,
            natural_id: inner.planet_natural_id,
            id: inner.planet_id,
            has_local_market: inner.has_local_market,
        })
    }
}

#[derive(Debug)]
pub struct PlanetWorkforce {
    pub planet_id: String,
    pub planet_natural_id: String,
    pub pioneers: WorkforceDetails,
    pub settlers: WorkforceDetails,
    pub technicians: WorkforceDetails,
    pub engineers: WorkforceDetails,
    pub scientists: WorkforceDetails,
}

#[derive(Debug, Deserialize, Default)]
pub struct WorkforceDetails {
    #[serde(rename = "Capacity")]
    pub capacity: u32,
    #[serde(rename = "Population")]
    pub population: u32,
    #[serde(rename = "Required")]
    pub required: u32,
    #[serde(rename = "Satisfaction")]
    pub satisfaction: f32,
    #[serde(rename = "WorkforceNeeds")]
    pub needs: Vec<WorkforceNeed>,
    #[serde(rename = "WorkforceTypeName")]
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct WorkforceNeed {
    #[serde(rename = "Essential")]
    pub essential: bool,
    #[serde(rename = "MaterialTicker")]
    pub ticker: String,
    #[serde(rename = "Satisfaction")]
    pub satisfaction: f32,
    /// The total number of units needed per day, based on your current population
    #[serde(rename = "UnitsPerInterval")]
    pub units_per_interval: f32,

    /// The number of units needed per day per 100 workers
    #[serde(rename = "UnitsPerOneHundred")]
    pub units_per_one_hundred: f32,
}

impl PlanetWorkforce {
    pub(crate) fn from_json(v: serde_json::Value) -> anyhow::Result<Self> {
        #[derive(Deserialize)]
        #[allow(unused)]
        struct Inner {
            #[serde(rename = "PlanetId")]
            planet_id: String,
            #[serde(rename = "PlanetName")]
            planet_name: String,
            #[serde(rename = "PlanetNaturalId")]
            planet_natural_id: String,

            #[serde(rename = "Workforces")]
            workforces: Vec<WorkforceDetails>,
        }

        let inner: Inner = serde_json::from_value(v)?;

        let mut pioneers = None;
        let mut settlers = None;
        let mut technicians = None;
        let mut engineers = None;
        let mut scientists = None;

        for details in inner.workforces {
            match details.name.as_str() {
                "PIONEER" => pioneers = Some(details),
                "SETTLER" => settlers = Some(details),
                "TECHNICAN" => technicians = Some(details),
                "ENGINEER" => engineers = Some(details),
                "SCIENTIST" => scientists = Some(details),
                _ => {}
            }
        }

        Ok(Self {
            planet_id: inner.planet_id,
            planet_natural_id: inner.planet_natural_id,
            pioneers: pioneers.unwrap_or_default(),
            settlers: settlers.unwrap_or_default(),
            technicians: technicians.unwrap_or_default(),
            engineers: engineers.unwrap_or_default(),
            scientists: scientists.unwrap_or_default(),
        })
    }
}

#[derive(Debug, Deserialize, Default)]
pub struct LocalMarket {
    #[serde(rename = "BuyingAds")]
    pub buying_ads: Vec<MarketContract>,
    #[serde(rename = "SellingAds")]
    pub selling_ads: Vec<MarketContract>,
}

#[derive(Debug, Deserialize)]
pub struct MarketContract {
    #[serde(rename = "DeliveryTime")]
    pub delivery_time: u32,
    #[serde(rename = "MaterialTicker")]
    pub material_ticker: String,
    #[serde(rename = "MaterialAmount")]
    pub material_amount: u32,
    #[serde(rename = "Price")]
    pub total_price: f32,
    #[serde(rename = "PriceCurrency")]
    pub currency: String,

    #[serde(rename = "CreatorCompanyName")]
    pub creator_company_name: String,

    #[serde(rename = "CreatorCompanyCode")]
    pub creator_company_code: String,
}

impl LocalMarket {
    pub(crate) fn from_json(v: serde_json::Value) -> anyhow::Result<Self> {
        Ok(serde_json::from_value(v)?)
    }
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
        let data: serde_json::Value =
            serde_json::from_reader(std::fs::File::open("test_data/exchange_SF.CI1.json").unwrap())
                .unwrap();

        let sfci1 = Ticker::from_json(data).unwrap();
        dbg!(sfci1);
    }
}

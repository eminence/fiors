use std::{
    collections::HashMap,
    fs::File,
    path::{Path, PathBuf},
    sync::atomic::AtomicU64,
    time::Duration,
};

use anyhow::{bail, Context};
use chrono::Utc;
use dashmap::DashMap;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

mod material_db;
pub use material_db::get_material_db;

mod building_db;
pub use building_db::get_building_db;

mod recipe_db;
pub use recipe_db::get_recipe_db;
use tracing::{instrument, trace, warn};
use types::WarehouseInfo;

use crate::types::{Item, WorkforceDetails};

pub mod materials;
pub mod types;

struct CachedData<T> {
    data: T,
    expiry: chrono::DateTime<Utc>,
}

impl<T: Clone> CachedData<T> {
    fn new(data: T, expiry: std::time::Duration) -> Self {
        Self {
            data,
            expiry: Utc::now() + expiry,
        }
    }
}

pub struct FIOClient {
    url_root: &'static str,
    auth_token: String,
    expiry: Option<chrono::DateTime<Utc>>,
    client: reqwest::Client,
    pub local_cache_dir: Option<PathBuf>,
    retry_delay: AtomicU64,

    planet_cache: DashMap<String, CachedData<types::Planet>>,

    /// Map from (username, storage_id) to a list of all stores (including warehouses)
    storage_user_cache: DashMap<String, CachedData<Vec<types::Storage>>>,

    /// Map from (username, storage_id) to storage info (base storage only)
    storage_cache: DashMap<(String, String), CachedData<Option<types::Storage>>>,

    /// Map from username to list of storage info.
    /// 
    /// The WarehouseInfo struct doesn't contain the actual storage data, but rather the storage ID and the type of storage
    warehouse_cache: DashMap<String, CachedData<Vec<types::WarehouseInfo>>>,

    planet_info_cache: DashMap<String, CachedData<Vec<types::Planet>>>,
    workforce_cache: DashMap<(String, String), CachedData<types::PlanetWorkforce>>,
    localmarket_cache: DashMap<String, CachedData<types::LocalMarket>>,
    exchange_cache: DashMap<String, CachedData<types::Ticker>>,
    planet_production_cache: DashMap<(String, String), CachedData<Vec<types::ProductionLine>>>,
    own_orders_cache: DashMap<String, CachedData<Vec<types::OwnMarketOrder>>>,
}

impl FIOClient {
    pub async fn new_with_password(username: String, password: String) -> anyhow::Result<Self> {
        // first post a login and extract the authtoken and expiry

        let post_data = serde_json::json!({"UserName": username, "Password": password});
        let client = reqwest::ClientBuilder::new()
            .timeout(Duration::from_secs(60))
            .build()?;
        let url_root = "https://rest.fnar.net";

        let req = client
            .post(format!("{url_root}/auth/login"))
            .json(&post_data)
            .header("Accept", "application/json")
            .build()?;

        let resp = client.execute(req).await?;

        if resp.status().as_u16() == 401 {
            bail!("Failed to authenticate")
        }

        let mut resp_body: HashMap<String, serde_json::Value> = resp.json().await?;

        let auth_token = resp_body
            .remove("AuthToken")
            .context("Missing AuthToken from login response")?
            .as_str()
            .context("Unexpected type for AuthToken")?
            .to_string();
        let expiry = resp_body
            .remove("Expiry")
            .context("Missing Expiry from login response")?;

        let expiry = chrono::DateTime::parse_from_rfc3339(
            expiry.as_str().context("Unexpected type for expiry")?,
        )?
        .to_utc();

        Ok(Self {
            url_root,
            auth_token,
            expiry: Some(expiry),
            client,
            local_cache_dir: None,
            retry_delay: AtomicU64::new(500),
            planet_cache: DashMap::new(),
            storage_user_cache: DashMap::new(),
            storage_cache: DashMap::new(),
            planet_info_cache: DashMap::new(),
            warehouse_cache: DashMap::new(),
            workforce_cache: DashMap::new(),
            localmarket_cache: DashMap::new(),
            exchange_cache: DashMap::new(),
            planet_production_cache: DashMap::new(),
            own_orders_cache: DashMap::new(),
        })
    }

    pub fn new_with_key(auth_token: String) -> Self {
        let client = reqwest::ClientBuilder::new()
            .timeout(Duration::from_secs(60))
            .build()
            .unwrap();
        let url_root = "https://rest.fnar.net";
        Self {
            url_root,
            auth_token,
            expiry: None,
            client,
            local_cache_dir: None,
            retry_delay: AtomicU64::new(500),
            planet_cache: DashMap::new(),
            storage_user_cache: DashMap::new(),
            storage_cache: DashMap::new(),
            planet_info_cache: DashMap::new(),
            warehouse_cache: DashMap::new(),
            workforce_cache: DashMap::new(),
            localmarket_cache: DashMap::new(),
            exchange_cache: DashMap::new(),
            planet_production_cache: DashMap::new(),
            own_orders_cache: DashMap::new(),
        }
    }

    pub fn new_from_env() -> anyhow::Result<Self> {
        let auth_token = std::env::var("FIO_AUTH_TOKEN")?;
        Ok(Self::new_with_key(auth_token))
    }

    fn should_retry(&self) -> bool {
        self.retry_delay.load(std::sync::atomic::Ordering::Relaxed) < 15000
    }

    async fn retry_sleep(&self) {
        let time_ms = self.retry_delay.load(std::sync::atomic::Ordering::Relaxed);
        tokio::time::sleep(Duration::from_millis(time_ms)).await;
    }
    fn increase_retry(&self) {
        let time_ms = self.retry_delay.load(std::sync::atomic::Ordering::Relaxed);
        self.retry_delay.store(
            (time_ms as f32 * 1.75) as u64,
            std::sync::atomic::Ordering::Relaxed,
        );
        // println!("Retry delay now {:?}", self.retry_delay);
    }
    fn decrease_retry(&self) {
        let time_ms = self.retry_delay.load(std::sync::atomic::Ordering::Relaxed);

        self.retry_delay.store(
            time_ms.saturating_sub(500).max(500),
            std::sync::atomic::Ordering::Relaxed,
        );
        // println!("Retry delay now {:?}", self.retry_delay);
    }

    #[instrument(skip(self))]
    async fn request<T: DeserializeOwned + Serialize>(
        &self,
        url: &str,
    ) -> anyhow::Result<Option<T>> {
        let get_cache = |path: &Path| {
            path.join(url.trim_matches('/').replace(['/', '.'], "_"))
                .with_extension("json")
        };

        if let Some(cache) = self
            .local_cache_dir
            .as_deref()
            .map(get_cache)
            .and_then(|path| File::open(path).ok())
        {
            let md = cache
                .metadata()
                .ok()
                .and_then(|md| md.modified().ok())
                .and_then(|md| md.elapsed().ok())
                .map(|d| d.as_secs() < 3600)
                .unwrap_or(false);

            trace!("Trying to load data from local disk cache");
            if md {
                if let Ok(loaded_from_cache) = serde_json::from_reader(cache) {
                    // println!("Returning from cache");
                    return Ok(Some(loaded_from_cache));
                }
            }
        }

        while self.should_retry() {
            let resp = self
                .client
                .get(format!("{}{url}", self.url_root))
                .header("Authorization", &self.auth_token)
                .header("accept", "application/json")
                .send()
                .await;

            if matches!(resp, Err(ref e) if e.is_timeout()) {
                warn!("Request timed out, retrying");
                self.retry_sleep().await;
                self.increase_retry();
                continue;
            }

            let resp = resp?;

            let status = resp.status();
            if status.as_u16() == 522 {
                warn!("Got a 522, retrying");
            }

            if status.as_u16() == 429 || status.as_u16() == 522 {
                self.retry_sleep().await;
                self.increase_retry();
                continue;
            }
            self.decrease_retry();

            if status.as_u16() == 204 {
                return Ok(None);
            } else if status.is_success() {
                let data: T = resp.json().await?;
                if let Some(cache) = self
                    .local_cache_dir
                    .as_deref()
                    .map(get_cache)
                    .map(|path| {
                        if let Some(p) = path.parent() {
                            let _ = std::fs::create_dir_all(p);
                        }
                        path
                    })
                    .and_then(|path| File::create(path).ok())
                {
                    serde_json::to_writer(cache, &data)?;
                }
                return Ok(Some(data));
            } else {
                bail!("Request not successful: {:?}", status);
            }
        }

        bail!("Failed after too many retries")
    }

    /// Returns the username of the logged in user
    pub async fn is_auth(&self) -> anyhow::Result<String> {
        while self.should_retry() {
            let resp = self
                .client
                .get(format!("{}/auth", self.url_root))
                .header("Authorization", &self.auth_token)
                .header("accept", "text/plain")
                .send()
                .await?;

            let status = resp.status();
            if status.as_u16() == 429 {
                // dbg!(resp.headers());
                self.retry_sleep().await;
                self.increase_retry();
                continue;
            }
            self.decrease_retry();

            if status.is_success() {
                let data = resp.text().await?;
                return Ok(data);
            } else {
                bail!("Request not successful: {:?}", status);
            }
        }
        bail!("Failed after too many retries")
    }

    pub async fn get_planet(&self, planet_id: &str) -> anyhow::Result<types::Planet> {
        if let Some(cached) = self.planet_cache.get(planet_id) {
            if cached.expiry > Utc::now() {
                return Ok(cached.data.clone());
            }
        }
        let resp: Option<serde_json::Value> = self.request(&format!("/planet/{planet_id}")).await?;

        // planet info is cached for 24 hours
        if let Some(planet) = resp {
            let data = types::Planet::from_json(planet).with_context(|| {
                format!("Failed to construct Planet object for planet_id={planet_id}")
            })?;
            self.planet_cache.insert(
                planet_id.to_string(),
                CachedData::new(data.clone(), Duration::from_secs(86400)),
            );
            Ok(data)
        } else {
            bail!("Planet does not exist")
        }
    }

    pub async fn get_planets_for_user(&self, user: &str) -> anyhow::Result<Vec<()>> {
        let _resp: Option<serde_json::Value> =
            self.request(&format!("/sites/planets/{user}")).await?;
        todo!();
    }

    pub async fn get_planetsite_for_user(&self, user: &str, planet_id: &str) -> anyhow::Result<types::PlanetSite> {
        let resp: Option<serde_json::Value> =
            self.request(&format!("/sites/{user}/{planet_id}")).await?;
        
        if let Some(v) = resp {
            Ok(serde_json::from_value(v)?)
        } else {
            bail!("No planet found")
        }
    }

    pub async fn get_warehouse_info_for_user(&self, user: &str) -> anyhow::Result<Vec<types::WarehouseInfo>> {
        if let Some(cached) = self.warehouse_cache.get(user) {
            if cached.expiry > Utc::now() {
                return Ok(cached.data.clone());
            }
        }

        let resp: Option<serde_json::Value> = self.request(&format!("/sites/warehouses/{user}")).await?;

        let mut v = Vec::new();
        if let Some(Value::Array(list)) = resp {
            for obj in list.into_iter() {
                let sto:WarehouseInfo = serde_json::from_value(obj).unwrap();

                v.push(sto);
            }
        }

        // warehouse info is cached for 1 hour
        self.warehouse_cache.insert(
            user.to_string(),
            CachedData::new(v.clone(), Duration::from_secs(3600)),
        );
        Ok(v)
        
    }

    pub async fn get_all_storage_for_user(
        &self,
        user: &str,
    ) -> anyhow::Result<Vec<types::Storage>> {
        if let Some(cached) = self.storage_user_cache.get(user) {
            if cached.expiry > Utc::now() {
                return Ok(cached.data.clone());
            }
        }
        let resp: Option<serde_json::Value> = self.request(&format!("/storage/{user}")).await?;

        let mut v = Vec::new();
        if let Some(Value::Array(list)) = resp {
            for obj in list.into_iter() {
                let sto = types::Storage::from_json(obj).unwrap();

                v.push(sto);
            }
        }

        // storage info is cached for 15 minutes
        self.storage_user_cache.insert(
            user.to_string(),
            CachedData::new(v.clone(), Duration::from_secs(900)),
        );
        Ok(v)
    }

    /// Store can be a StorageId, a PlanetId, PlanetNaturalId, or PlanetName
    pub async fn get_storage_for_user(
        &self,
        user: &str,
        store: &str,
    ) -> anyhow::Result<Option<types::Storage>> {
        if let Some(cached) = self
            .storage_cache
            .get(&(user.to_string(), store.to_string()))
        {
            if cached.expiry > Utc::now() {
                return Ok(cached.data.clone());
            }
        }

        let resp: Option<serde_json::Value> =
            self.request(&format!("/storage/{user}/{store}")).await?;

        let data = if let Some(sto) = resp {
            Some(types::Storage::from_json(sto)?)
        } else {
            None
        };

        // storage info is cached for 15 minutes
        self.storage_cache.insert(
            (user.to_string(), store.to_string()),
            CachedData::new(data.clone(), Duration::from_secs(900)),
        );

        Ok(data)
    }

    /// Returns a list of planet IDs (AB-123x) where the given user has storage
    pub async fn get_storage_planets_for_user(
        &self,
        user: &str,
    ) -> anyhow::Result<Vec<types::Planet>> {
        if let Some(cached) = self.planet_info_cache.get(user) {
            if cached.expiry > Utc::now() {
                return Ok(cached.data.clone());
            }
        }

        let resp: Option<Vec<String>> = self.request(&format!("/storage/planets/{user}")).await?;

        let mut v = Vec::new();
        if let Some(planets) = resp {
            for planet in planets {
                let planet = self.get_planet(&planet).await?;
                v.push(planet)
            }
        }

        // storage info is cached for 24 hours
        self.planet_info_cache.insert(
            user.to_string(),
            CachedData::new(v.clone(), Duration::from_secs(86400)),
        );

        Ok(v)
    }

    pub async fn get_planet_workforce_for_user(
        &self,
        user: &str,
        planet: &str,
    ) -> anyhow::Result<types::PlanetWorkforce> {
        if let Some(cached) = self
            .workforce_cache
            .get(&(user.to_string(), planet.to_string()))
        {
            if cached.expiry > Utc::now() {
                return Ok(cached.data.clone());
            }
        }

        let resp: Option<serde_json::Value> =
            self.request(&format!("/workforce/{user}/{planet}")).await?;

        let data = types::PlanetWorkforce::from_json(resp.context("No workforce on planet")?)?;

        // workforce info is cached for 2 hours
        self.workforce_cache.insert(
            (user.to_string(), planet.to_string()),
            CachedData::new(data.clone(), Duration::from_secs(7200)),
        );

        Ok(data)
    }

    pub async fn get_planet_localmarket(&self, planet: &str) -> anyhow::Result<types::LocalMarket> {
        if let Some(cached) = self.localmarket_cache.get(planet) {
            if cached.expiry > Utc::now() {
                return Ok(cached.data.clone());
            }
        }

        let resp: Option<serde_json::Value> = self
            .request(&format!("/localmarket/planet/{planet}"))
            .await?;

        let data = if let Some(data) = resp {
            types::LocalMarket::from_json(data)?
        } else {
            Default::default()
        };

        // localmarket info is cached for 10 minutes
        self.localmarket_cache.insert(
            planet.to_string(),
            CachedData::new(data.clone(), Duration::from_secs(600)),
        );
        Ok(data)
    }

    pub async fn get_exchange_info(&self, ticker: &str) -> anyhow::Result<types::Ticker> {
        if let Some(cached) = self.exchange_cache.get(ticker) {
            if cached.expiry > Utc::now() {
                return Ok(cached.data.clone());
            }
        }

        // it's more efficient to get the full exchange info (and cache it), than it is to request info on each ticker we need
        let resp: Option<Vec<serde_json::Value>> = self.request("/exchange/full").await?;

        if let Some(data) = resp {
            // ticker data is cached for 15 minutes
            for ticker_data in data {
                let x = serde_json::to_string_pretty(&ticker_data).unwrap();
                let individual_ticker = types::Ticker::from_json(ticker_data).with_context(|| x)?;
                self.exchange_cache.insert(
                    individual_ticker.name.clone(),
                    CachedData::new(individual_ticker, Duration::from_secs(900)),
                );
            }

            // now get our data out of the cache
            Ok(self
                .exchange_cache
                .get(ticker)
                .map(|t| t.data.clone())
                .context("No exchange info found for this ticker")?)
        } else {
            bail!("No exchange info found for this ticker")
        }
    }

    pub async fn get_all_materials(&self) -> anyhow::Result<HashMap<String, types::MaterialInfo>> {
        let resp: Option<Vec<serde_json::Value>> = self.request("/material/allmaterials").await?;

        let mut map = HashMap::new();
        if let Some(materials) = resp {
            for material in materials {
                if let Ok(material) = types::MaterialInfo::from_json(material.clone()) {
                    map.insert(material.ticker.clone(), material);
                } else {
                    println!("Failed to parse material");
                    println!("{:?}", material);
                }
            }
        }

        Ok(map)
    }

    pub async fn get_all_buildings(&self) -> anyhow::Result<HashMap<String, types::BuildingInfo>> {
        let resp: Option<Vec<serde_json::Value>> = self.request("/building/allbuildings").await?;

        let mut map = HashMap::new();
        if let Some(buildings) = resp {
            for building in buildings {
                if let Ok(building) = types::BuildingInfo::from_json(building.clone()) {
                    map.insert(building.ticker.clone(), building);
                } else {
                    println!("Failed to parse building");
                    println!("{:?}", building);
                }
            }
        }

        Ok(map)
    }

    pub async fn get_all_recipes(&self) -> anyhow::Result<Vec<types::Recipe>> {
        let resp: Option<Vec<serde_json::Value>> = self.request("/recipes/allrecipes").await?;

        resp.unwrap()
            .into_iter()
            .map(types::Recipe::from_json)
            .collect()
    }

    pub async fn get_planet_production(
        &self,
        username: &str,
        planet: &str,
    ) -> anyhow::Result<Vec<types::ProductionLine>> {
        if let Some(cached) = self
            .planet_production_cache
            .get(&(username.to_string(), planet.to_string()))
        {
            if cached.expiry > Utc::now() {
                return Ok(cached.data.clone());
            }
        }

        let resp: Option<Vec<serde_json::Value>> = self
            .request(&format!("/production/{username}/{planet}"))
            .await?;

        let mut v = Vec::new();
        if let Some(orders) = resp {
            for order in orders {
                let order = types::ProductionLine::from_json(order)?;
                v.push(order);
            }
        }

        // production info is cached for 15 minutes
        self.planet_production_cache.insert(
            (username.to_string(), planet.to_string()),
            CachedData::new(v.clone(), Duration::from_secs(900)),
        );

        Ok(v)
    }

    pub async fn get_cxos_for_user(
        &self,
        username: &str,
    ) -> anyhow::Result<Vec<types::OwnMarketOrder>> {
        if let Some(cached) = self.own_orders_cache.get(username) {
            if cached.expiry > Utc::now() {
                return Ok(cached.data.clone());
            }
        }

        let resp: Option<Vec<serde_json::Value>> =
            self.request(&format!("/cxos/{username}")).await?;

        let mut v = Vec::new();
        if let Some(orders) = resp {
            for order in orders {
                let order = serde_json::from_value(order)?;
                v.push(order);
            }
        }

        // cache for 15 minutes
        self.own_orders_cache.insert(
            username.to_string(),
            CachedData::new(v.clone(), Duration::from_secs(900)),
        );

        Ok(v)
    }

    pub async fn calc_building_cost(
        &self,
        building_ticker: &str,
        planet_cxid: &str,
    ) -> anyhow::Result<f32> {
        let building = get_building_db().get(building_ticker).unwrap();
        let mut total_cost = 0.0;
        for (ticker, amount) in building.building_cost.iter() {
            let cx_info = self
                .get_exchange_info(&format!("{}.{planet_cxid}", ticker))
                .await?;
            let total = cx_info
                .instant_buy(*amount)
                .map(|o| o.total_value)
                .or_else(|| cx_info.get_any_price())
                .unwrap();
            total_cost += total;
        }

        Ok(total_cost)
    }

    /// Returns Ok(None) if the given product is not produced on the given planet
    #[instrument(skip(self, username, cogm), fields(cogm.is_some = cogm.is_some()))]
    pub async fn calc_cost_of_goods_manufactured(
        &self,
        username: &str,
        planet: &str,
        building_ticker: &str,
        material_ticker: &str,
        cogm: Option<&HashMap<String, f32>>,
    ) -> anyhow::Result<Option<f32>> {
        trace!(username);

        let planet_obj = self.get_planet(&planet).await?;
        let planet_cxid = planet_obj.get_cx_mid().unwrap_or("CI1");

        let prods = self.get_planet_production(username, planet).await?;
        for prod in prods {
            if !prod.orders.iter().any(|order| {
                order.standard_recipe_name.starts_with(building_ticker)
                    && order
                        .outputs
                        .iter()
                        .any(|o| o.material_ticker == material_ticker)
            }) {
                continue;
            }
            // dbg!(&prod);
            let mut total_daily_costs = 0.0;

            let inv = self
                .get_storage_for_user(username, &prod.planet_id)
                .await?
                .map(|s| s.items)
                .unwrap_or_default();

            let building: &building_db::StaticBuildingInfo =
                get_building_db().get(prod.building_type.as_str()).unwrap();
            // dbg!(building);
            let building_cost = self
                .calc_building_cost(building.ticker, planet_cxid)
                .await?;
            // assume we repair our buildings after 90 days
            let repair_cost = building_cost - (building_cost * 0.5).floor();
            let daily_repair_cost = repair_cost / 90.0;

            let wf = self
                .get_planet_workforce_for_user(username, &prod.planet_id)
                .await
                .unwrap();

            // find an order for this material
            let order = prod
                .orders
                .iter()
                .find(|order| {
                    order.duration.is_some()
                        && order
                        .outputs
                        .iter()
                        .any(|o| o.material_ticker == material_ticker)
                    // && order.started.is_none()
                })
                .context("Failed to find order")?;

            total_daily_costs += daily_repair_cost;
            // production scale -- multiple by this to compute how much stuff is produced per day

            trace!(daily_repair_cost);

            let day_scale = 86400.0 / order.duration.unwrap().as_secs() as f32;
            for input in &order.inputs {
                let daily_buy_amt = input.material_amount as f32 * day_scale;
                let cx_info = self
                    .get_exchange_info(&format!("{}.{planet_cxid}", input.material_ticker))
                    .await
                    .unwrap();

                let market_costs =
                    if let Some(total) = cx_info.instant_buy(daily_buy_amt.ceil() as u32) {
                        total.total_value
                    } else if let Some(x) = cx_info.price {
                        x * daily_buy_amt.ceil()
                    } else if let Some(x) = cx_info.get_any_price() {
                        x * daily_buy_amt.ceil()
                    } else {
                        0.0
                    };

                // println!(
                //     "Market costs for {} units of {}: {}",
                //     daily_buy_amt.ceil(),
                //     input.material_ticker,
                //     market_costs
                // );

                match cogm
                    .and_then(|m| m.get(&input.material_ticker))
                    .map(|cost| *cost * daily_buy_amt.ceil())
                {
                    Some(x) if x < market_costs => {
                        // println!(
                        //     "Making {} for {} is cheaper than buying at market for {}",
                        //     input.material_ticker, x, market_costs
                        // );
                        total_daily_costs += x
                    }
                    x => total_daily_costs += market_costs,
                }

                // println!(
                //     "{}:  qty={} total={}",
                //     input.material_ticker, daily_buy_amt, total.total_value
                // );
            }

            let daily_output_amt = order.outputs[0].material_amount as f32 * day_scale;
            // println!(
            //     "Production {} per day ({daily_output_amt})",
            //     materials::MaterialWithColor::new(order.outputs[0].material_ticker.as_str())
            //         .with_amount(daily_output_amt.floor() as i32)
            // );

            async fn add_needs(
                client: &FIOClient,
                inv: &HashMap<String, Item>,
                num_workers: u32,
                planet_cxid: &str,
                details: &WorkforceDetails,
            ) -> f32 {
                let mut total = 0.0;
                for need in &details.needs {
                    // only include this needed consumable if we have it in our inventory, or if it's essential
                    if inv.contains_key(&need.ticker) || need.essential {
                        // how much per day do we need?
                        let daily = need.units_per_one_hundred * (num_workers as f32 / 100.0);
                        let cx_info = client
                            .get_exchange_info(&format!("{}.{planet_cxid}", need.ticker))
                            .await
                            .unwrap();
                        total += cx_info.ask.or_else(|| cx_info.get_any_price()).unwrap() * daily;
                    }
                }
                total
            }

            let mut workforce_costs = 0.0;

            if building.pioneers > 0 {
                workforce_costs += add_needs(
                    self,
                    &inv,
                    building.pioneers,
                    planet_cxid,
                    wf.details.get(types::PlanetWorkforce::PIONEER).unwrap(),
                )
                .await;
            }
            if building.settlers > 0 {
                workforce_costs += add_needs(
                    self,
                    &inv,
                    building.settlers,
                    planet_cxid,
                    wf.details.get(types::PlanetWorkforce::SETTLER).unwrap(),
                )
                .await;
            }
            if building.technicians > 0 {
                workforce_costs += add_needs(
                    self,
                    &inv,
                    building.technicians,
                    planet_cxid,
                    wf.details.get(types::PlanetWorkforce::TECHNICIAN).unwrap(),
                )
                .await;
            }
            if building.engineers > 0 {
                workforce_costs += add_needs(
                    self,
                    &inv,
                    building.engineers,
                    planet_cxid,
                    wf.details.get(types::PlanetWorkforce::ENGINEER).unwrap(),
                )
                .await;
            }
            if building.scientists > 0 {
                workforce_costs += add_needs(
                    self,
                    &inv,
                    building.scientists,
                    planet_cxid,
                    wf.details.get(types::PlanetWorkforce::SCIENTIST).unwrap(),
                )
                .await;
            }

            let new_workforce_cost_calc = self
                .calc_workforce_costs(username, planet, building_ticker, true, true)
                .await?;
            if new_workforce_cost_calc.abs_sub(workforce_costs) > 1.0 {
                warn!(
                    old = workforce_costs,
                    new = new_workforce_cost_calc,
                    "Differing workforce cost calculations"
                );
            }

            total_daily_costs += workforce_costs;

            return Ok(Some(total_daily_costs / daily_output_amt));
        }

        Ok(None)
    }

    pub async fn calc_workforce_costs(
        &self,
        username: &str,
        planet_id: &str,
        building_ticker: &str,
        lux1: bool,
        lux2: bool,
    ) -> anyhow::Result<f32> {
        let building = get_building_db()
            .get(building_ticker)
            .context("No such building")?;

        let planet = self.get_planet(planet_id).await?;
        let planet_cxid = planet.get_cx_mid().unwrap_or("CI1");

        let mut total_daily_costs = 0.0;

        let wf = self
            .get_planet_workforce_for_user(username, planet_id)
            .await
            .unwrap();

        async fn add_needs(
            client: &FIOClient,
            num_workers: u32,
            details: &WorkforceDetails,
            planet_cxid: &str,
            lux1: bool,
            lux2: bool,
        ) -> f32 {
            let mut total = 0.0;
            for need in &details.needs {
                // TODO handle lux1 and lux2
                // TODO handle needs provided by our own buildings
                if need.essential
                    || (lux1 && ["COF", "KOM", "ALE", "GIN", "WIN"].contains(&need.ticker.as_str()))
                    || (lux2 && ["PWO", "REP", "SC", "VG", "NST"].contains(&need.ticker.as_str()))
                {
                    // how much per day do we need?
                    let daily = need.units_per_one_hundred * (num_workers as f32 / 100.0);
                    let cx_info = client
                        .get_exchange_info(&format!("{}.{planet_cxid}", need.ticker))
                        .await
                        .unwrap();
                    total += cx_info.ask.or_else(|| cx_info.get_any_price()).unwrap() * daily;
                }
            }
            total
        }

        if building.pioneers > 0 {
            total_daily_costs += add_needs(
                self,
                building.pioneers,
                wf.details.get(types::PlanetWorkforce::PIONEER).unwrap(),
                planet_cxid,
                lux1,
                lux2,
            )
            .await;
        }
        if building.settlers > 0 {
            total_daily_costs += add_needs(
                self,
                building.settlers,
                wf.details.get(types::PlanetWorkforce::SETTLER).unwrap(),
                planet_cxid,
                lux1,
                lux2,
            )
            .await;
        }
        if building.technicians > 0 {
            total_daily_costs += add_needs(
                self,
                building.technicians,
                wf.details.get(types::PlanetWorkforce::TECHNICIAN).unwrap(),
                planet_cxid,
                lux1,
                lux2,
            )
            .await;
        }
        if building.engineers > 0 {
            total_daily_costs += add_needs(
                self,
                building.engineers,
                wf.details.get(types::PlanetWorkforce::ENGINEER).unwrap(),
                planet_cxid,
                lux1,
                lux2,
            )
            .await;
        }

        if building.scientists > 0 {
            total_daily_costs += add_needs(
                self,
                building.scientists,
                wf.details.get(types::PlanetWorkforce::SCIENTIST).unwrap(),
                planet_cxid,
                lux1,
                lux2,
            )
            .await;
        }

        Ok(total_daily_costs)
    }
}

#[cfg(any(test, feature = "live_tests"))]
mod live_tests {
    use crate::{materials::MaterialWithColor, types::PlanetWorkforce};

    use super::*;

    fn get_test_client() -> FIOClient {
        let mut client = FIOClient::new_from_env().unwrap();
        client.local_cache_dir = Some(".fio_cache".into());
        client
    }

    #[tokio::test]
    #[ignore]
    async fn get_test_data_from_live() -> anyhow::Result<()> {
        let client = get_test_client();

        let data: serde_json::Value = client
            .request("/storage/eminence32")
            .await
            .unwrap()
            .unwrap();
        serde_json::to_writer_pretty(
            std::fs::File::create("test_data/storage_eminence32.json")?,
            &data,
        )?;

        let data: serde_json::Value = client.request("/exchange/SF.CI1").await.unwrap().unwrap();
        serde_json::to_writer_pretty(
            std::fs::File::create("test_data/exchange_SF_CI1.json")?,
            &data,
        )?;

        let data: serde_json::Value = client.request("/exchange/COT.CI1").await.unwrap().unwrap();
        serde_json::to_writer_pretty(
            std::fs::File::create("test_data/exchange_COT_CI1.json")?,
            &data,
        )?;

        let data: serde_json::Value = client
            .request("/material/allmaterials")
            .await
            .unwrap()
            .unwrap();
        serde_json::to_writer_pretty(
            std::fs::File::create("test_data/material_allmaterials.json")?,
            &data,
        )?;

        Ok(())
    }

    #[tokio::test]
    async fn test_login() {
        let client =
            FIOClient::new_with_password("eminence32".into(), "FYJOL9pNVyUzba14AAzg".into())
                .await
                .unwrap();

        assert_eq!("EMINENCE32", client.is_auth().await.unwrap());

        let client = FIOClient::new_from_env().unwrap();
        assert_eq!("EMINENCE32", client.is_auth().await.unwrap());
    }

    #[tokio::test]
    async fn test_planet() {
        let client = get_test_client();
        let pli = client.get_planet("CB-282d").await.unwrap();
        dbg!(pli);
    }

    #[tokio::test]
    async fn test_planet_for_user() {
        let client = get_test_client();
        // let pli = client.get_planets_for_user("eminence32").await.unwrap();
        client
            .get_planetsite_for_user("eminence32", "1e39248e468e9a6bb12938ba97b58bcf")
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_storage() {
        let client = get_test_client();
        let _storage = client.get_all_storage_for_user("eminence32").await.unwrap();

        let storage_planets = client
            .get_storage_planets_for_user("eminence32")
            .await
            .unwrap();
        assert!(storage_planets.iter().any(|p| p.name == "Katoa"));

        for planet in &storage_planets {
            let storage = client
                .get_storage_for_user("eminence32", &planet.natural_id)
                .await
                .unwrap();
            dbg!(storage);
        }
    }

    #[tokio::test]
    async fn test_workforce() {
        let client = get_test_client();
        let wf = client
            .get_planet_workforce_for_user("eminence32", "UV-351a")
            .await
            .unwrap();
        dbg!(wf);
    }

    #[tokio::test]
    async fn test_localmarket() {
        let client = get_test_client();
        let lm = client.get_planet_localmarket("UV-351a").await.unwrap();
        dbg!(lm);
    }

    #[tokio::test]
    async fn test_exchange() {
        let client = get_test_client();
        let cx = client.get_exchange_info("SF.CI1").await.unwrap();
        dbg!(cx);
    }

    #[tokio::test]
    async fn test_production() {
        let client = get_test_client();
        let prods = client
            .get_planet_production("EMINENCE32", "4d8bcb1963537462b8042799a24ebaac")
            .await
            .unwrap();
        // level load across all production lines (negative values indicate daily need)
        let mut total_daily_production = HashMap::new();
        for prod in prods {
            // if prod.building_type != "prefabPlant1" { continue }
            dbg!(&prod);
            let daily = prod.daily_production();
            for (mat, amt) in daily.outputs {
                *total_daily_production.entry(mat).or_insert(0.0) += amt;
            }
            for (mat, amt) in daily.inputs {
                *total_daily_production.entry(mat).or_insert(0.0) -= amt;
            }
        }
        dbg!(total_daily_production);
    }

    #[tokio::test]
    async fn test_cogm() {
        let client = get_test_client();
        let prods = client
            .get_planet_production("EMINENCE32", "Pyrgos")
            .await
            .unwrap();

        for prod in prods {
            if prod.planet_name != "Pyrgos" {
                continue;
            }
            if prod.building_type != "farm" {
                continue;
            }

            let building = get_building_db().get(prod.building_type.as_str()).unwrap();
            dbg!(building);
            let building_cost = client.calc_building_cost(building.ticker, "TODO").await.unwrap();
            println!("Building cost: {}", building_cost);
            // assume we repair our buildings after 90 days
            let repair_cost = building_cost - (building_cost * 0.5).floor();
            let daily_repair_cost = repair_cost / 90.0;
            println!("Daily repair cost: {}", daily_repair_cost);

            let wf = client
                .get_planet_workforce_for_user("EMINENCE32", &prod.planet_id)
                .await
                .unwrap();
            let inv = client
                .get_storage_for_user("EMINENCE32", &prod.planet_id)
                .await
                .unwrap()
                .unwrap();

            for order in &prod.orders {
                if !order.outputs.iter().any(|out| out.material_ticker == "BEA")
                    || order.started.is_some()
                {
                    continue;
                }
                dbg!(order);
                let mut total_daily_costs = daily_repair_cost;

                // production scale -- multiple by this to compute how much stuff is produced per day
                let day_scale = 86400.0 / order.duration.as_secs() as f32;
                for input in &order.inputs {
                    let daily_buy_amt = input.material_amount as f32 * day_scale;
                    let cx_info = client
                        .get_exchange_info(&format!("{}.CI1", input.material_ticker))
                        .await
                        .unwrap();
                    let total = cx_info.instant_buy(daily_buy_amt.ceil() as u32).unwrap();
                    total_daily_costs += total.total_value;
                    println!(
                        "{}:  qty={} total={}",
                        input.material_ticker, daily_buy_amt, total.total_value
                    );
                }
                let daily_output_amt = order.outputs[0].material_amount as f32 * day_scale;
                println!(
                    "Production {} per day ({daily_output_amt})",
                    MaterialWithColor::new(order.outputs[0].material_ticker.as_str())
                        .with_amount(daily_output_amt.floor() as i32)
                );

                let details = wf.details.get(PlanetWorkforce::PIONEER).unwrap();
                dbg!(details);
                for need in &details.needs {
                    // only include this needed consumable if we have it in our inventory
                    if inv.items.contains_key(&need.ticker) {
                        // how much per day do we need?
                        let daily = need.units_per_one_hundred * (building.pioneers as f32 / 100.0);
                        let cx_info = client
                            .get_exchange_info(&format!("{}.CI1", need.ticker))
                            .await
                            .unwrap();
                        let total =
                            cx_info.ask.or_else(|| cx_info.get_any_price()).unwrap() * daily; //cx_info.instant_buy(daily.ceil() as u32).unwrap();
                        println!(
                            "Need {} per day({daily}), costing {}",
                            MaterialWithColor::new(&need.ticker).with_amount(daily.ceil() as i32),
                            total
                        );
                        total_daily_costs += total;
                    }
                }

                println!("Total daily costs: {total_daily_costs}");
                println!("Cost per unit: {}", total_daily_costs / daily_output_amt);
            }
        }

        // manual calculation:
        /*
        100 pioneers need:
            * 0.5 COF per day  -> 750 / 2 = 375
            * 4 DW per day -> 308
            * 0.5 OVE per day -> 97.2 / 2 = 48.6
            * 0.2 PWO per day -> 274 / 5 = 54.8
            * 4 RAT per day -> 428

            total: 375 + 308 + 48.6 + 54.8 + 428 = 1214.4


        AMM production:

        50 pioneers needed:
          *

        */
    }

    #[tokio::test]
    async fn test_cogm2() {
        let client = get_test_client();
        let mut map = HashMap::new();

        let cogm = client
            .calc_cost_of_goods_manufactured("EMINENCE32", "Umbra", "CHP", "FLX", None)
            .await
            .unwrap();

        println!("Making FLX at Umbra costs: {:?}", cogm);
        map.insert("FLX".to_string(), cogm.unwrap());

        // let cogm = client
        //     .calc_cost_of_goods_manufactured("EMINENCE32", "Katoa", "BMP", "PE", None)
        //     .await
        //     .unwrap();

        // println!("Making PE at Katoa costs: {:?}", cogm);
        // map.insert("PE".to_string(), cogm.unwrap());

        // let cogm = client
        //     .calc_cost_of_goods_manufactured("EMINENCE32", "Gibson", "PP1", "BDE", Some(&map))
        //     .await
        //     .unwrap();

        // println!("Making BDE at Gibon/PP1 costs: {:?}", cogm);
        // let cogm = client
        //     .calc_cost_of_goods_manufactured("EMINENCE32", "Gibson", "PP2", "BDE", Some(&map))
        //     .await
        //     .unwrap();

        // println!("Making BDE at Gibon/PP2 costs: {:?}", cogm);
    }
}

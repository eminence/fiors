use std::{
    collections::HashMap,
    fs::File,
    path::{Path, PathBuf},
    sync::atomic::AtomicU64,
    time::Duration,
};

use anyhow::{bail, Context};
use building_db::StaticBuildingInfo;
use chrono::Utc;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

mod material_db;
pub use material_db::get_material_db;

mod building_db;
pub use building_db::get_building_db;

use crate::types::{Item, WorkforceDetails};

pub mod materials;
pub mod types;

pub struct FIOClient {
    url_root: &'static str,
    auth_token: String,
    expiry: Option<chrono::DateTime<Utc>>,
    client: reqwest::Client,
    pub local_cache_dir: Option<PathBuf>,
    retry_delay: AtomicU64,
}

impl FIOClient {
    pub async fn new_with_password(username: String, password: String) -> anyhow::Result<Self> {
        // first post a login and extract the authtoken and expiry

        let post_data = serde_json::json!({"UserName": username, "Password": password});
        let client = reqwest::Client::new();
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
        })
    }

    pub fn new_with_key(auth_token: String) -> Self {
        let client = reqwest::Client::new();
        let url_root = "https://rest.fnar.net";
        Self {
            url_root,
            auth_token,
            expiry: None,
            client,
            local_cache_dir: None,
            retry_delay: AtomicU64::new(500),
        }
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

    async fn request<T: DeserializeOwned + Serialize>(
        &self,
        url: &str,
    ) -> anyhow::Result<Option<T>> {
        let get_cache = |path: &Path| {
            path.join(url.trim_matches('/').replace("/", "_").replace('.', "_"))
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
                .map(|d| d.as_secs() < 900)
                .unwrap_or(false);

            if let Ok(mut file) = File::options().create(true).append(true).open("client.log") {
                use std::io::Write;
                writeln!(file, "Requesting: {} md={md}", url)?;
            }

            if md {
                if let Ok(loaded_from_cache) = serde_json::from_reader(cache) {
                    // println!("Returning from cache");
                    return Ok(Some(loaded_from_cache));
                }
            }
        }
        if let Ok(mut file) = File::options().create(true).append(true).open("client.log") {
            use std::io::Write;
            writeln!(file, "Requesting: {url}")?;
        }

        while self.should_retry() {
            let resp = self
                .client
                .get(format!("{}/{url}", self.url_root))
                .header("Authorization", &self.auth_token)
                .header("accept", "application/json")
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
                    serde_json::to_writer_pretty(cache, &data)?;
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

    pub async fn get_planet(&self, planet: &str) -> anyhow::Result<types::Planet> {
        let resp: Option<serde_json::Value> = self.request(&format!("/planet/{planet}")).await?;

        if let Some(planet) = resp {
            Ok(types::Planet::from_json(planet)?)
        } else {
            bail!("Planet does not exist")
        }
    }

    pub async fn get_planets_for_user(&self, user: &str) -> anyhow::Result<()> {
        let _resp: Option<serde_json::Value> =
            self.request(&format!("/sites/planets/{user}")).await?;
        todo!();
    }

    pub async fn get_planet_for_user(&self, user: &str, planet_id: &str) -> anyhow::Result<()> {
        let _resp: Option<serde_json::Value> =
            self.request(&format!("/sites/{user}/{planet_id}")).await?;
        todo!();
    }

    pub async fn get_all_storage_for_user(
        &self,
        user: &str,
    ) -> anyhow::Result<Vec<types::Storage>> {
        let resp: Option<serde_json::Value> = self.request(&format!("/storage/{user}")).await?;

        let mut v = Vec::new();
        if let Some(Value::Array(list)) = resp {
            for obj in list.into_iter() {
                let sto = types::Storage::from_json(obj).unwrap();

                v.push(sto);
            }
        }

        Ok(v)
    }

    /// Store can be a StorageId, a PlanetId, PlanetNaturalId, or PlanetName
    pub async fn get_storage_for_user(
        &self,
        user: &str,
        store: &str,
    ) -> anyhow::Result<Option<types::Storage>> {
        let resp: Option<serde_json::Value> =
            self.request(&format!("/storage/{user}/{store}")).await?;

        if let Some(sto) = resp {
            Ok(Some(types::Storage::from_json(sto)?))
        } else {
            Ok(None)
        }
    }

    /// Returns a list of planet IDs (AB-123x) where the given user has storage
    pub async fn get_storage_planets_for_user(
        &self,
        user: &str,
    ) -> anyhow::Result<Vec<types::Planet>> {
        let resp: Option<Vec<String>> = self.request(&format!("/storage/planets/{user}")).await?;

        let mut v = Vec::new();
        if let Some(planets) = resp {
            for planet in planets {
                let planet = self.get_planet(&planet).await?;
                v.push(planet)
            }
        }

        Ok(v)
    }

    pub async fn get_planet_workforce_for_user(
        &self,
        user: &str,
        planet: &str,
    ) -> anyhow::Result<types::PlanetWorkforce> {
        let resp: Option<serde_json::Value> =
            self.request(&format!("/workforce/{user}/{planet}")).await?;

        Ok(types::PlanetWorkforce::from_json(
            resp.context("No workforce on planet")?,
        )?)
    }

    pub async fn get_planet_localmarket(&self, planet: &str) -> anyhow::Result<types::LocalMarket> {
        let resp: Option<serde_json::Value> = self
            .request(&format!("/localmarket/planet/{planet}"))
            .await?;

        if let Some(data) = resp {
            Ok(types::LocalMarket::from_json(data)?)
        } else {
            Ok(Default::default())
        }
    }

    pub async fn get_exchange_info(&self, ticker: &str) -> anyhow::Result<types::Ticker> {
        let resp: Option<serde_json::Value> = self.request(&format!("/exchange/{ticker}")).await?;

        if let Some(data) = resp {
            Ok(types::Ticker::from_json(data)?)
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

    pub async fn get_planet_production(
        &self,
        username: &str,
        planet: &str,
    ) -> anyhow::Result<Vec<types::ProductionLine>> {
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

        Ok(v)
    }

    pub async fn calc_building_cost(&self, building_ticker: &str) -> anyhow::Result<f32> {
        let building = get_building_db().get(building_ticker).unwrap();
        let mut total_cost = 0.0;
        for (ticker, amount) in building.building_cost.iter() {
            let cx_info = self.get_exchange_info(&format!("{}.CI1", ticker)).await?;
            let total = cx_info
                .instant_buy(*amount as u32)
                .map(|o| o.total_value)
                .unwrap_or(cx_info.price);
            total_cost += total;
        }

        Ok(total_cost)
    }

    pub async fn calc_cost_of_goods_manufactured(
        &self,
        username: &str,
        planet: &str,
        building_ticker: &str,
        material_ticker: &str,
    ) -> anyhow::Result<f32> {
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

            let building = get_building_db().get(prod.building_type.as_str()).unwrap();
            // dbg!(building);
            let building_cost = self.calc_building_cost(building.ticker).await?;
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
                    order
                        .outputs
                        .iter()
                        .any(|o| o.material_ticker == material_ticker)
                    // && order.started.is_none()
                })
                .context("Failed to find order")?;

            total_daily_costs += daily_repair_cost;
            // production scale -- multiple by this to compute how much stuff is produced per day

            let day_scale = (86400.0 / order.duration.as_secs() as f32);
            for input in &order.inputs {
                let daily_buy_amt = input.material_amount as f32 * day_scale;
                let cx_info = self
                    .get_exchange_info(&format!("{}.CI1", input.material_ticker))
                    .await
                    .unwrap();
                let total = cx_info.instant_buy(daily_buy_amt.ceil() as u32).unwrap();
                total_daily_costs += total.total_value;
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
                details: &WorkforceDetails,
            ) -> f32 {
                let mut total = 0.0;
                for need in &details.needs {
                    // only include this needed consumable if we have it in our inventory, or if it's essential
                    if inv.contains_key(&need.ticker) || need.essential {
                        // how much per day do we need?
                        let daily = need.units_per_one_hundred * (num_workers as f32 / 100.0);
                        let cx_info = client
                            .get_exchange_info(&format!("{}.CI1", need.ticker))
                            .await
                            .unwrap();
                        total += cx_info.ask.unwrap_or(cx_info.price) * daily;
                    }
                }
                total
            }

            if building.pioneers > 0 {
                total_daily_costs += add_needs(
                    self,
                    &inv,
                    building.pioneers,
                    wf.details.get(types::PlanetWorkforce::PIONEER).unwrap(),
                )
                .await;
            }
            if building.settlers > 0 {
                total_daily_costs += add_needs(
                    self,
                    &inv,
                    building.settlers,
                    wf.details.get(types::PlanetWorkforce::SETTLER).unwrap(),
                )
                .await;
            }
            if building.technicians > 0 {
                total_daily_costs += add_needs(
                    self,
                    &inv,
                    building.technicians,
                    wf.details.get(types::PlanetWorkforce::TECHNICIAN).unwrap(),
                )
                .await;
            }
            if building.engineers > 0 {
                total_daily_costs += add_needs(
                    self,
                    &inv,
                    building.engineers,
                    wf.details.get(types::PlanetWorkforce::ENGINEER).unwrap(),
                )
                .await;
            }
            if building.scientists > 0 {
                total_daily_costs += add_needs(
                    self,
                    &inv,
                    building.scientists,
                    wf.details.get(types::PlanetWorkforce::SCIENTIST).unwrap(),
                )
                .await;
            }

            return Ok(total_daily_costs / daily_output_amt);
        }

        Ok(0.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::{materials::MaterialWithColor, types::PlanetWorkforce};

    use super::*;

    fn get_test_client() -> FIOClient {
        let mut client = FIOClient::new_with_key("9dd5160d-acc8-493d-b222-d5f96273f677".into());
        client.local_cache_dir = Some(".fio_cache".into());
        client
    }

    #[tokio::test]
    #[ignore]
    async fn get_test_data_from_live() -> anyhow::Result<()> {
        let client = get_test_client();

        let data: serde_json::Value = client
            .request(&format!("/storage/eminence32"))
            .await
            .unwrap()
            .unwrap();
        serde_json::to_writer_pretty(
            std::fs::File::create("test_data/storage_eminence32.json")?,
            &data,
        )?;

        let data: serde_json::Value = client
            .request(&format!("/exchange/SF.CI1"))
            .await
            .unwrap()
            .unwrap();
        serde_json::to_writer_pretty(
            std::fs::File::create("test_data/exchange_SF_CI1.json")?,
            &data,
        )?;

        let data: serde_json::Value = client
            .request(&format!("/exchange/COT.CI1"))
            .await
            .unwrap()
            .unwrap();
        serde_json::to_writer_pretty(
            std::fs::File::create("test_data/exchange_COT_CI1.json")?,
            &data,
        )?;

        let data: serde_json::Value = client
            .request(&format!("/material/allmaterials"))
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
    #[cfg(live_tests)]
    async fn test_login() {
        let client =
            FIOClient::new_with_password("eminence32".into(), "FYJOL9pNVyUzba14AAzg".into())
                .await
                .unwrap();

        assert!(client.is_auth().await.unwrap());

        let client = FIOClient::new_with_key("9dd5160d-acc8-493d-b222-d5f96273f677".into());
        assert!(client.is_auth().await.unwrap());
    }

    #[tokio::test]
    #[cfg(live_tests)]
    async fn test_planet() {
        let client = get_test_client();
        let pli = client.get_planet("JS-952c").await.unwrap();
        dbg!(pli);
    }

    #[tokio::test]
    // #[cfg(live_tests)]
    async fn test_planet_for_user() {
        let client = get_test_client();
        // let pli = client.get_planets_for_user("eminence32").await.unwrap();
        let pli = client
            .get_planet_for_user("eminence32", "1e39248e468e9a6bb12938ba97b58bcf")
            .await
            .unwrap();
        dbg!(pli);
    }

    #[tokio::test]
    // #[cfg(live_tests)]
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

            break;
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
            .get_planet_production("EMINENCE32", "Katoa")
            .await
            .unwrap();
        // level load across all production lines (negative values indicate daily need)
        let mut total_daily_production = HashMap::new();
        for prod in prods {
            // if prod.building_type != "prefabPlant1" { continue }
            // dbg!(&prod);
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
            .get_planet_production("EMINENCE32", "Katoa")
            .await
            .unwrap();

        for prod in prods {
            if prod.planet_name != "Katoa" {
                continue;
            }
            if prod.building_type != "refinery" {
                continue;
            }

            let building = get_building_db().get(prod.building_type.as_str()).unwrap();
            dbg!(building);
            let building_cost = client.calc_building_cost(building.ticker).await.unwrap();
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
                if !order.outputs.iter().any(|out| out.material_ticker == "SF")
                    || order.started.is_some()
                {
                    continue;
                }
                dbg!(order);
                let mut total_daily_costs = daily_repair_cost;

                // production scale -- multiple by this to compute how much stuff is produced per day
                let day_scale = (86400.0 / order.duration.as_secs() as f32);
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
                        let total = cx_info.ask.unwrap_or(cx_info.price) * daily; //cx_info.instant_buy(daily.ceil() as u32).unwrap();
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

        let cogm = client
            .calc_cost_of_goods_manufactured("EMINENCE32", "Katoa", "WPL", "NL")
            .await
            .unwrap();

        dbg!(cogm);
    }
}

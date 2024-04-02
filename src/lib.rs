use std::{
    collections::HashMap,
    fs::File,
    path::{Path, PathBuf},
    time::Duration,
};

use anyhow::{bail, Context};
use chrono::Utc;
use reqwest::Client;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;

pub mod types;

pub struct FIOClient {
    url_root: &'static str,
    auth_token: String,
    expiry: Option<chrono::DateTime<Utc>>,
    client: reqwest::Client,
    pub local_cache_dir: Option<PathBuf>,
}

#[derive(Debug, Copy, Clone)]
enum RetryBackoff {
    /// Never retry
    Never,
    /// Retry after some number of milliseconds
    RetryAfter(u64),
}

impl Default for RetryBackoff {
    fn default() -> Self {
        Self::RetryAfter(100)
    }
}

impl RetryBackoff {
    fn next(self) -> Self {
        match self {
            RetryBackoff::Never => Self::Never,
            RetryBackoff::RetryAfter(ms) if ms >= 3000 => Self::Never,
            RetryBackoff::RetryAfter(ms) => Self::RetryAfter(ms * 2),
        }
    }
    fn should_retry(self) -> bool {
        !matches!(self, Self::Never)
    }
    async fn sleep(self) {
        match self {
            RetryBackoff::Never => (),
            RetryBackoff::RetryAfter(ms) => {
                println!("Retrying after delay...");
                tokio::time::sleep(Duration::from_millis(ms)).await;
            }
        }
    }
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
        }
    }

    /// Useful for tests
    #[cfg(test)]
    async fn get_and_return_text(
        &self,
        url: &str,
        mut retry: RetryBackoff,
    ) -> anyhow::Result<String> {
        while retry.should_retry() {
            let resp = self
                .client
                .get(format!("{}/{url}", self.url_root))
                .header("Authorization", &self.auth_token)
                .header("accept", "application/json")
                .send()
                .await?;

            let status = resp.status();
            if status.as_u16() == 429 && retry.should_retry() {
                retry.sleep().await;
                retry = retry.next();
                continue;
            }

            if status.as_u16() == 204 {
                return Ok("".to_string());
            } else if status.is_success() {
                let data = resp.text().await?;
                return Ok(data);
            } else {
                bail!("Request not successful: {:?}", status);
            }
        }

        bail!("Failed after too many retries")
    }

    async fn request<T: DeserializeOwned + Serialize>(
        &self,
        url: &str,
        mut retry: RetryBackoff,
    ) -> anyhow::Result<Option<T>> {
        let get_cache = |path: &Path| {
            path.join(url.trim_matches('/').replace("/", "_"))
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
                .map(|d| d.as_secs() < 360)
                .unwrap_or(false);

            if md {
                if let Ok(loaded_from_cache) = serde_json::from_reader(cache) {
                    // println!("Returning from cache");
                    return Ok(Some(loaded_from_cache));
                }
            }
        }

        while retry.should_retry() {
            let resp = self
                .client
                .get(format!("{}/{url}", self.url_root))
                .header("Authorization", &self.auth_token)
                .header("accept", "application/json")
                .send()
                .await?;

            let status = resp.status();
            if status.as_u16() == 429 && retry.should_retry() {
                retry.sleep().await;
                retry = retry.next();
                continue;
            }

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

    pub async fn is_auth(&self) -> anyhow::Result<bool> {
        let resp = self
            .client
            .get(format!("{}/auth", self.url_root))
            .header("Authorization", &self.auth_token)
            .send()
            .await?;

        let status = resp.status();
        // dbg!(status);
        // dbg!(&resp.text().await?);
        Ok(status.is_success())
    }

    pub async fn get_planet(&self, planet: &str) -> anyhow::Result<types::Planet> {
        let resp: Option<serde_json::Value> = self
            .request(&format!("/planet/{planet}"), RetryBackoff::default())
            .await?;

        if let Some(planet) = resp {
            Ok(types::Planet::from_json(planet)?)
        } else {
            bail!("Planet does not exist")
        }
    }

    pub async fn get_planets_for_user(&self, user: &str) -> anyhow::Result<()> {
        let resp: Option<serde_json::Value> = self
            .request(&format!("/sites/planets/{user}"), Default::default())
            .await?;
        todo!();
    }

    pub async fn get_planet_for_user(&self, user: &str, planet_id: &str) -> anyhow::Result<()> {
        let resp: Option<serde_json::Value> = self
            .request(&format!("/sites/{user}/{planet_id}"), Default::default())
            .await?;
        todo!();
    }

    pub async fn get_all_storage_for_user(
        &self,
        user: &str,
    ) -> anyhow::Result<Vec<types::Storage>> {
        let resp: Option<serde_json::Value> = self
            .request(&format!("/storage/{user}"), RetryBackoff::default())
            .await?;

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
        let resp: Option<serde_json::Value> = self
            .request(&format!("/storage/{user}/{store}"), RetryBackoff::default())
            .await?;

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
        let resp: Option<Vec<String>> = self
            .request(&format!("/storage/planets/{user}"), RetryBackoff::default())
            .await?;

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
        let resp: Option<serde_json::Value> = self
            .request(
                &format!("/workforce/{user}/{planet}"),
                RetryBackoff::default(),
            )
            .await?;

        Ok(types::PlanetWorkforce::from_json(
            resp.context("No workforce on planet")?,
        )?)
    }

    pub async fn get_planet_localmarket(&self, planet: &str) -> anyhow::Result<types::LocalMarket> {
        let resp: Option<serde_json::Value> = self
            .request(
                &format!("/localmarket/planet/{planet}"),
                RetryBackoff::default(),
            )
            .await?;

        if let Some(data) = resp {
            Ok(types::LocalMarket::from_json(data)?)
        } else {
            Ok(Default::default())
        }
    }

    pub async fn get_exchange_info(&self, ticker: &str) -> anyhow::Result<types::Ticker> {
        let resp: Option<serde_json::Value> = self
            .request(&format!("/exchange/{ticker}"), RetryBackoff::default())
            .await?;

        if let Some(data) = resp {
            Ok(types::Ticker::from_json(data)?)
        } else {
            bail!("No exchange info found for this ticker")
        }
    }
}

#[cfg(test)]
mod tests {
    use reqwest::get;

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
            .request(&format!("/storage/eminence32"), Default::default())
            .await
            .unwrap()
            .unwrap();
        serde_json::to_writer_pretty(
            std::fs::File::create("test_data/storage_eminence32.json")?,
            &data,
        )?;

        let data: serde_json::Value = client
            .request(&format!("/exchange/SF.CI1"), Default::default())
            .await
            .unwrap()
            .unwrap();
        serde_json::to_writer_pretty(
            std::fs::File::create("test_data/exchange_SF.CI1.json")?,
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
}

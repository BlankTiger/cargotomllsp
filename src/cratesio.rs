use anyhow::Result;
use once_cell::sync::Lazy;
use reqwest::header::HeaderMap;
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
    sync::{Arc, Mutex, OnceLock},
};
use tracing::info;

// static CRATES_URL: &str = "https://crates.io/api/v1/crates";
static CRATE_URL: &str = "https://crates.io/api/v1/crates/{crate_name}";
static CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
    let mut default_headers = HeaderMap::new();
    default_headers.insert("User-Agent", "cargotomllsp".parse().unwrap());
    reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(2))
        .default_headers(default_headers)
        .build()
        .unwrap()
});

type Crates = HashMap<String, CrateInfo>;

pub struct CrateStore(Crates);

impl Deref for CrateStore {
    type Target = Crates;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CrateStore {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub static CRATE_STORE: OnceLock<Arc<Mutex<CrateStore>>> = OnceLock::new();

pub fn init_crate_store() {
    _ = CRATE_STORE.set(Arc::new(Mutex::new(CrateStore(HashMap::new()))));
}

#[derive(Debug, Clone)]
pub struct CrateInfo {
    pub name: String,
    pub version: String,
    pub features: Vec<String>,
}

pub fn get_stored_crate_info(name: &str) -> Option<CrateInfo> {
    return CRATE_STORE
        .get()
        .expect("text store not initialized")
        .lock()
        .expect("text store mutex poisoned")
        .get(name)
        .cloned();
}

pub fn write_crate_info_to_store(name: &str, info: CrateInfo) {
    CRATE_STORE
        .get()
        .expect("text store not initialized")
        .lock()
        .expect("text store mutex poisoned")
        .insert(name.to_string(), info);
}

impl TryFrom<serde_json::Value> for CrateInfo {
    type Error = anyhow::Error;
    fn try_from(value: serde_json::Value) -> Result<Self> {
        Ok(Self {
            name: value["crate"]["name"].to_string(),
            version: value["crate"]["newest_version"].to_string(),
            features: value["versions"][0]["features"]
                .as_object()
                .unwrap()
                .keys()
                .map(|x| x.to_string())
                .collect(),
        })
    }
}

pub async fn get_crate_info(name: &str) -> Result<CrateInfo> {
    if let Some(crate_info) = get_stored_crate_info(name) {
        info!("Got crate data from store: {:?}", crate_info);
        return Ok(crate_info);
    }
    info!("Getting crate data from crates.io for: {}", name);
    let crate_data = CLIENT
        .get(CRATE_URL.replace("{crate_name}", name))
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;
    let crate_data: CrateInfo = crate_data.try_into()?;
    write_crate_info_to_store(name, crate_data.clone());
    info!("Got crate data: {:?}", crate_data);
    Ok(crate_data)
}

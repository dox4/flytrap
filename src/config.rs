use anyhow::Result;
use std::fs;

use once_cell::sync::OnceCell;
use serde::{de::Error, Deserialize, Serialize};

static GLOBAL_CONFIG: OnceCell<Config> = OnceCell::new();

pub(crate) fn global_config() -> &'static Config {
    GLOBAL_CONFIG
        .get()
        .expect("get global configuration failed.")
}

pub(crate) fn init_config(file: &Option<String>) -> Result<()> {
    let conf = if let Some(file) = file {
        let data = fs::read_to_string(file)?;
        toml::from_str(&data)?
    } else {
        eprintln!("use default configuration.");
        Default::default()
    };
    GLOBAL_CONFIG
        .set(conf)
        .expect("set global configuration failed.");
    Ok(())
}

// ==========================
// config models
// =========================

#[derive(Debug, Serialize, Deserialize, Default)]
pub(crate) struct Config {
    pub(crate) base: BaseConfig,
    pub(crate) log: LogConfig,
    pub(crate) db: DbConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct BaseConfig {
    pub(crate) host: String,
    pub(crate) port: u16,
}

impl Default for BaseConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 2345u16,
        }
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct LogConfig {
    #[serde(deserialize_with = "deserialize_log_level")]
    #[serde(serialize_with = "serialize_log_level")]
    pub(crate) level: tracing::Level,
    pub(crate) file: String,
}

fn serialize_log_level<S>(level: &tracing::Level, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let s = level.as_str().to_lowercase();
    serializer.serialize_str(&s)
}

fn deserialize_log_level<'de, D>(deserializer: D) -> Result<tracing::Level, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.to_lowercase().as_str() {
        "trace" => Ok(tracing::Level::TRACE),
        "debug" => Ok(tracing::Level::DEBUG),
        "info" => Ok(tracing::Level::INFO),
        "warn" => Ok(tracing::Level::WARN),
        "error" => Ok(tracing::Level::ERROR),
        unknown => Err(D::Error::custom(format!("unknown log level: {}", unknown))),
    }
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: tracing::Level::DEBUG,
            file: env!("CARGO_PKG_NAME").to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct DbConfig {
    pub(crate) sockaddr: String,
    pub(crate) database: String,
    pub(crate) user: String,
    pub(crate) password: String,
}

impl DbConfig {
    pub(crate) fn db_url(&self) -> String {
        format!(
            "mysql://{}:{}@{}/{}?charset=utf8mb4&collation=utf8mb4_unicode_ci",
            self.user, self.password, self.sockaddr, self.database
        )
    }
}

impl Default for DbConfig {
    fn default() -> Self {
        Self {
            sockaddr: "127.0.0.1:3306".to_string(),
            database: env!("CARGO_PKG_NAME").to_string(),
            user: env!("CARGO_PKG_NAME").to_string(),
            password: "<Password>".to_string(),
        }
    }
}

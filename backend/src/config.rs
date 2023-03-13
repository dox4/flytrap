use anyhow::Result;
use std::fs;

use once_cell::sync::OnceCell;
use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub base: BaseConfig,
    #[serde(default)]
    pub log: LogConfig,
    #[serde(default)]
    pub db: DbConfig,
}

#[derive(Debug, Deserialize)]
pub struct BaseConfig {
    #[serde(default = "BaseConfig::default_host")]
    pub host: String,
    #[serde(default = "BaseConfig::default_port")]
    pub port: u16,
}

impl Default for BaseConfig {
    fn default() -> Self {
        Self {
            host: Self::default_host(),
            port: Self::default_port(),
        }
    }
}
impl BaseConfig {
    fn default_host() -> String {
        "0.0.0.0".to_string()
    }
    fn default_port() -> u16 {
        2302u16
    }
}

#[derive(Debug, Deserialize)]
pub struct LogConfig {
    #[serde(default = "LogConfig::default_level")]
    pub level: String,
    #[serde(default = "LogConfig::default_file")]
    pub file: String,
    #[serde(default = "LogConfig::default_structured")]
    pub structured: bool,
}

impl LogConfig {
    fn default_level() -> String {
        "debug".to_string()
    }

    fn default_file() -> String {
        "/var/log/flytrap/flytrap.log".to_string()
    }

    fn default_structured() -> bool {
        false
    }

    pub fn log_level(&self) -> tracing::Level {
        match self.level.to_lowercase().as_str() {
            "trace" => tracing::Level::TRACE,
            "debug" => tracing::Level::DEBUG,
            "info" => tracing::Level::INFO,
            "warn" => tracing::Level::WARN,
            "error" => tracing::Level::ERROR,
            _ => unreachable!("invalid log level."),
        }
    }
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: Self::default_level(),
            file: Self::default_file(),
            structured: Self::default_structured(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct DbConfig {
    #[serde(default = "DbConfig::default_db_domain")]
    pub db_domain: String,
    #[serde(default = "DbConfig::default_db_name")]
    pub db_name: String,
    #[serde(default = "DbConfig::default_db_user")]
    pub db_user: String,
    #[serde(default = "DbConfig::default_db_password")]
    pub db_password: String,
}

impl DbConfig {
    fn default_db_domain() -> String {
        "127.0.0.1:3306".to_string()
    }

    fn default_db_name() -> String {
        "flytrap".to_string()
    }

    fn default_db_user() -> String {
        "flytrap".to_string()
    }

    fn default_db_password() -> String {
        "flytrap".to_string()
    }

    pub(crate) fn db_url(&self) -> String {
        format!(
            "mysql://{}:{}@{}/{}",
            self.db_user, self.db_password, self.db_domain, self.db_name
        )
    }
}

impl Default for DbConfig {
    fn default() -> Self {
        Self {
            db_domain: Self::default_db_domain(),
            db_name: Self::default_db_name(),
            db_user: Self::default_db_user(),
            db_password: Self::default_db_password(),
        }
    }
}

static GLOBAL_CONFIG: OnceCell<Config> = OnceCell::new();

pub fn global_config() -> &'static Config {
    GLOBAL_CONFIG
        .get()
        .expect("get global configuration failed.")
}

pub fn init_config(file: Option<String>) -> Result<()> {
    let conf = if let Some(file) = file {
        let data = fs::read_to_string(file)?;
        toml::from_str(&data)?
    } else {
        Default::default()
    };
    GLOBAL_CONFIG.set(conf).expect("set global configuration failed.");
    Ok(())
}

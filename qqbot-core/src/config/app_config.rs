use std::time::Duration;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub logging_level: log::LevelFilter,
    pub cmd_suffix: String,
    pub database: DatabaseConfig,
    pub cache:CacheConfig,
    pub admins:Vec<i64>,
}

#[derive(Debug,Deserialize)]
pub struct CacheConfig{
    pub cache_capacity: u64,
    #[serde(with = "humantime_serde")]
    pub cache_lifetime: Duration,
    #[serde(with = "humantime_serde")]
    pub cache_idletime: Duration,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    #[serde(with = "humantime_serde")]
    pub connect_timeout: Duration,
    #[serde(with = "humantime_serde")]
    pub acquire_timeout: Duration,
    #[serde(with = "humantime_serde")]
    pub idle_timeout: Duration,
    #[serde(with = "humantime_serde")]
    #[serde(default)]
    pub max_lifetime: Duration,
    pub max_connections: u32,
    pub sqlx_logging: bool,
    // pub schema:Option<String>
}

use std::time::Duration;
pub mod permission;
pub mod service;
use config::APPCONFIG;
use moka::future::Cache;
use once_cell::sync::Lazy;

use serde::{Deserialize, Serialize};

pub mod config;
pub mod models;
pub mod repo;
pub mod cmd;
pub mod reply_strategy;

type UserId=i64;
#[derive(Serialize,Deserialize,Debug,Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum StrategeType {
    CmdStrategy
} 
impl std::default::Default for StrategeType{
    fn default() -> Self {
        StrategeType::CmdStrategy
    }
}
#[derive(Serialize,Deserialize,Debug,Clone, Copy)]
pub struct UserData{
    #[serde(default)]
    pub stratege:StrategeType
}
impl std::default::Default for UserData {
    fn default() -> Self {
        Self { stratege: Default::default() }
    }
}
pub static BOT_CACHE:Lazy<Cache<UserId,UserData>>=Lazy::new(||{
    Cache::builder()
    .max_capacity(APPCONFIG.cache.cache_capacity)
    .time_to_live(APPCONFIG.cache.cache_lifetime)
    .time_to_idle(APPCONFIG.cache.cache_idletime)
    .build()
});
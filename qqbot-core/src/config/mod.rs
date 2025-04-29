pub mod app_config;
use core::fmt;
use std::sync::Arc;

pub use app_config::*; // 导出 AppConfig 结构体

use config::{Config, Environment, File};
// 使用 once_cell 进行同步惰性初始化
use once_cell::sync::Lazy as SyncLazy; // 重命名以区分
// 使用 async_once_cell 进行异步惰性初始化
use async_once_cell::{Lazy, OnceCell as AsyncOnceCell}; // 重命名以区分 (或直接用 Lazy)

// 导入 SeaORM 相关类型
use sea_orm::{
    ConnectOptions, Database, DatabaseConnection, DbErr as SeaOrmError // 使用 SeaORM 的错误类型
};
// 保留 sqlx 类型，如果你仍然需要 init_db_pool 函数 (但它不用于 DB_GLOBAL)
use sea_orm::sqlx::{Error as SqlxError, MySql, Pool, mysql::MySqlPoolOptions};
use std::{future::Future, pin::Pin, time::Duration}; // 导入所需标准库类型

#[derive(Debug)]
pub struct ConfigError(String);
impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "config error: {}", self.0)
    }
}
impl std::error::Error for ConfigError {}

// 初始化 AppConfig (同步)
pub fn init_config(env: &str) -> Result<AppConfig, ConfigError> {
    let builder = Config::builder()
        .add_source(File::with_name(&format!("../config.{}", env)).required(false))
        .add_source(File::with_name(&format!("./config.{}", env)).required(false))
        .add_source(File::with_name(&format!("../../config.{}", env)).required(false))
        .add_source(Environment::with_prefix("QQBOT").separator("__"));

    // 考虑处理 build() 的 Result 而不是 unwrap
    let settings = match builder.build() {
        Ok(s) => s,
        Err(e) => return Err(ConfigError(format!("Failed to build config: {}", e))),
    };

    settings
        .try_deserialize::<AppConfig>()
        .map_err(|err| ConfigError(format!("Failed to deserialize config: {}", err)))
}

// 使用 once_cell::sync::Lazy (同步) 初始化配置
// 使用重命名的 SyncLazy
pub static APPCONFIG: SyncLazy<AppConfig> = SyncLazy::new(|| {
    // 考虑处理 init_config 的 Result 而不是 unwrap
    init_config("dev").expect("Failed to initialize AppConfig")
});

pub static DB_GLOBAL: AsyncOnceCell<Arc<DatabaseConnection>> = AsyncOnceCell::new();


pub async fn get_db() -> Arc<DatabaseConnection> {
    DB_GLOBAL
        .get_or_init(async {
            println!("Attempting to initialize SQLx database pool...");
            let db_conf = &APPCONFIG.database; // APPCONFIG 会在这里首次访问时初始化
            let database_url = &db_conf.url;
            let mut opt = ConnectOptions::new(database_url);
            
                opt.max_connections(db_conf.max_connections)
                .acquire_timeout(db_conf.acquire_timeout)
                .idle_timeout(db_conf.idle_timeout)
                .max_lifetime(db_conf.max_lifetime);

            let db = Database::connect(opt).await.unwrap();
            println!("SQLx Database pool initialized successfully!");
            Arc::new(db)
        })
        .await
        .clone()
}

// --- 测试部分 ---
#[test]
fn find_config_test() -> Result<(), ConfigError> {
    // 这个测试只访问 APPCONFIG，会触发它的同步初始化（如果尚未发生）
    println!("Testing APPCONFIG access...");
    println!("Database config from APPCONFIG: {:#?}", APPCONFIG.database);
    println!("APPCONFIG access successful.");
    Ok(())
}

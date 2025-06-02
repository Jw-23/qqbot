pub mod app_config;
use std::sync::Arc;

pub use app_config::*; // 导出 AppConfig 结构体
use crate::error::{AppError, AppResult};

use config::{Config, Environment, File};
// 使用 once_cell 进行同步惰性初始化
use once_cell::sync::Lazy as SyncLazy; // 重命名以区分
// 使用 async_once_cell 进行异步惰性初始化
use async_once_cell::{ OnceCell as AsyncOnceCell}; // 重命名以区分 (或直接用 Lazy)

// 导入 SeaORM 相关类型
use sea_orm::{
    ConnectOptions, Database, DatabaseConnection // 使用 SeaORM 的错误类型
};
 // 导入所需标准库类型

// 初始化 AppConfig (同步)
pub fn init_config(env: &str) -> AppResult<AppConfig> {
    let builder = Config::builder()
        .add_source(File::with_name(&format!("../config.{}", env)).required(false))
        .add_source(File::with_name(&format!("./config.{}", env)).required(false))
        .add_source(File::with_name(&format!("../../config.{}", env)).required(false))
        .add_source(Environment::with_prefix("QQBOT").separator("__"));

    let settings = builder.build()
        .map_err(|e| AppError::config(format!("Failed to build config: {}", e)))?;

    settings
        .try_deserialize::<AppConfig>()
        .map_err(|err| AppError::config(format!("Failed to deserialize config: {}", err)))
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

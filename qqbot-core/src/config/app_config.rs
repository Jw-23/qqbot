use std::time::Duration;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub logging_level: log::LevelFilter,
    pub cmd_suffix: String,
    pub database: DatabaseConfig,
    pub cache: CacheConfig,
    pub admins: Vec<i64>,
    pub llm: LlmConfig,
}

#[derive(Debug, Deserialize)]
pub struct CacheConfig {
    pub cache_capacity: u64,
    #[serde(with = "humantime_serde")]
    pub cache_lifetime: Duration,
    #[serde(with = "humantime_serde")]
    pub cache_idletime: Duration,
    // 对话缓存相关配置
    pub conversation_capacity: Option<u64>,
    pub max_conversation_history: Option<usize>,
    pub conversation_timeout_minutes: Option<i64>,
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

#[derive(Debug, Deserialize)]
pub struct LlmConfig {
    pub api_key: String,
    pub base_url: String,
    pub model: String,
    pub system_prompt: String,
    pub temperature: f32,
    pub max_tokens: u32,
    pub top_p: f32,
    pub timeout_seconds: u64,
    // 群聊自动捕获消息的配置
    #[serde(default = "default_auto_capture_group")]
    pub auto_capture_group_messages: bool,
}

fn default_auto_capture_group() -> bool {
    false
}

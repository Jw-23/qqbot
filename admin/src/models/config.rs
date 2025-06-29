use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigDto {
    pub logging_level: String,
    pub cmd_suffix: String,
    pub admins: Vec<String>,
    pub cache: CacheConfig,
    pub database: DatabaseConfig,
    pub llm: LlmConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CacheConfig {
    pub cache_lifetime: String,
    pub cache_capacity: u32,
    pub cache_idletime: String,
    pub conversation_capacity: u32,
    pub max_conversation_history: u32,
    pub conversation_timeout_minutes: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub connect_timeout: String,
    pub acquire_timeout: String,
    pub idle_timeout: String,
    pub max_lifetime: String,
    pub sqlx_logging: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LlmConfig {
    pub api_key: String,
    pub base_url: String,
    pub model: String,
    pub system_prompt: String,
    pub temperature: f64,
    pub max_tokens: u32,
    pub top_p: f64,
    pub timeout_seconds: u32,
    pub auto_capture_group_messages: bool,
}

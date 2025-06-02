use std::time::Duration;
pub mod permission;
pub mod service;
use chrono::{DateTime, Utc};
use config::APPCONFIG;
use moka::future::Cache;
use once_cell::sync::Lazy;
use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

pub mod cmd;
pub mod config;
pub mod conversation;
pub mod error;
pub mod models;
pub mod reply_strategy;
pub mod repo; // 添加错误处理模块

// 重新导出常用类型
pub use error::{AppError, AppResult};

type UserId = i64;
type GroupId = i64;

// 对话消息结构
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConversationMessage {
    pub role: String, // "user" 或 "assistant"
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub user_id: Option<UserId>,  // 在群聊中记录发言者ID，私聊中为None
    pub username: Option<String>, // 在群聊中记录发言者昵称，便于上下文理解
}

// 对话会话结构
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConversationSession {
    pub messages: VecDeque<ConversationMessage>,
    pub last_activity: DateTime<Utc>,
    pub max_history: usize, // 最大保留消息数
}

impl ConversationSession {
    pub fn new(max_history: usize) -> Self {
        Self {
            messages: VecDeque::new(),
            last_activity: Utc::now(),
            max_history,
        }
    }

    pub fn is_expired(&self, timeout_minutes: i64) -> bool {
        let now = Utc::now();
        let duration = now.signed_duration_since(self.last_activity);
        duration.num_minutes() >= timeout_minutes
    }

    pub fn get_recent_messages(&self, limit: usize) -> Vec<ConversationMessage> {
        self.messages
            .iter()
            .rev()
            .take(limit)
            .rev()
            .cloned()
            .collect()
    }

    pub fn add_message(&mut self, role: String, content: String) {
        let message = ConversationMessage {
            role,
            content,
            timestamp: Utc::now(),
            user_id: None, // 这里可以根据需要设置
            username: None,
        };

        self.messages.push_back(message);
        self.last_activity = Utc::now();

        // 保持历史记录数量在限制内
        while self.messages.len() > self.max_history {
            self.messages.pop_front();
        }
    }
}

// 会话标识符，支持私聊和群聊
#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq)]
pub enum SessionId {
    Private(UserId), // 私聊：每个用户独立对话
    Group(GroupId),  // 群聊：整个群共享对话历史
}

impl SessionId {
    pub fn get_user_id(&self) -> Option<UserId> {
        match self {
            SessionId::Private(user_id) => Some(*user_id),
            SessionId::Group(_) => None, // 群聊没有单一用户ID
        }
    }

    pub fn get_group_id(&self) -> Option<GroupId> {
        match self {
            SessionId::Private(_) => None,
            SessionId::Group(group_id) => Some(*group_id),
        }
    }

    pub fn is_private(&self) -> bool {
        matches!(self, SessionId::Private(_))
    }

    pub fn is_group(&self) -> bool {
        matches!(self, SessionId::Group(_))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum StrategeType {
    CmdStrategy,
    LlmStrategy,
}
impl std::default::Default for StrategeType {
    fn default() -> Self {
        StrategeType::CmdStrategy
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserData {
    #[serde(default)]
    pub stratege: StrategeType,
    #[serde(default)]
    pub model: String,
}
impl std::default::Default for UserData {
    fn default() -> Self {
        Self {
            stratege: Default::default(),
            model: String::from(""),
        }
    }
}
pub static BOT_CACHE: Lazy<Cache<UserId, UserData>> = Lazy::new(|| {
    Cache::builder()
        .max_capacity(APPCONFIG.cache.cache_capacity)
        .time_to_live(APPCONFIG.cache.cache_lifetime)
        .time_to_idle(APPCONFIG.cache.cache_idletime)
        .build()
});

// 对话历史缓存 - 10分钟过期
pub static CONVERSATION_CACHE: Lazy<Cache<SessionId, ConversationSession>> = Lazy::new(|| {
    Cache::builder()
        .max_capacity(APPCONFIG.cache.conversation_capacity.unwrap_or(1000))
        .time_to_idle(Duration::from_secs(600)) // 10分钟无活动则过期
        .build()
});

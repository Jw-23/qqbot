use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MessageSender {
    client: Client,
    bot_api_url: String,
}

#[derive(Debug, Serialize)]
struct SendMessageRequest {
    user_id: Option<i64>,
    group_id: Option<i64>,
    message: String,
}

#[derive(Debug, Deserialize)]
struct SendMessageResponse {
    status: String,
    message_id: Option<i64>,
}

impl MessageSender {
    pub fn new(bot_api_url: String) -> Self {
        Self {
            client: Client::new(),
            bot_api_url,
        }
    }

    /// 发送私聊消息
    pub async fn send_private_message(&self, user_id: i64, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        let request = SendMessageRequest {
            user_id: Some(user_id),
            group_id: None,
            message: message.to_string(),
        };

        let response = self.client
            .post(&format!("{}/send_private_msg", self.bot_api_url))
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(format!("发送消息失败: {}", response.status()).into())
        }
    }

    /// 发送群消息
    pub async fn send_group_message(&self, group_id: i64, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        let request = SendMessageRequest {
            user_id: None,
            group_id: Some(group_id),
            message: message.to_string(),
        };

        let response = self.client
            .post(&format!("{}/send_group_msg", self.bot_api_url))
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(format!("发送消息失败: {}", response.status()).into())
        }
    }

    /// 批量发送私聊消息
    pub async fn send_bulk_private_messages(&self, user_ids: &[i64], message: &str) -> (usize, Vec<String>) {
        let mut success_count = 0;
        let mut errors = Vec::new();

        for &user_id in user_ids {
            match self.send_private_message(user_id, message).await {
                Ok(_) => success_count += 1,
                Err(e) => errors.push(format!("用户{}: {}", user_id, e)),
            }
        }

        (success_count, errors)
    }
}

/// 消息发送服务的配置
#[derive(Debug, Clone)]
pub struct MessageSenderConfig {
    pub bot_api_url: String,
}

impl Default for MessageSenderConfig {
    fn default() -> Self {
        Self {
            // 这里假设QQ机器人提供HTTP API服务
            // 实际项目中应该从配置文件读取
            bot_api_url: "http://localhost:8080/api".to_string(),
        }
    }
}

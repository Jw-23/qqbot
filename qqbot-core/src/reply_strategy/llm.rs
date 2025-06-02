use super::{MessageContent, MessageContext, RelyStrategy, ReplyError, Env};
use crate::{config::APPCONFIG, GroupId, SessionId, UserId};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub temperature: f32,
    pub max_tokens: Option<u32>,
    pub top_p: Option<f32>,
    pub stream: bool,
}

#[derive(Debug, Deserialize)]
pub struct ChatChoice {
    pub message: ChatMessage,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub choices: Vec<ChatChoice>,
    pub usage: Option<Usage>,
}

#[derive(Debug, Deserialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Clone)]
pub struct LlmReplyStrategy {
    client: Client,
    api_key: String,
    base_url: String,
    model: String,
}

impl LlmReplyStrategy {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(APPCONFIG.llm.timeout_seconds))
                .build()
                .expect("Failed to create HTTP client"),
            api_key: APPCONFIG.llm.api_key.clone(),
            base_url: APPCONFIG.llm.base_url.clone(),
            model: APPCONFIG.llm.model.clone(),
        }
    }

    async fn get_conversation_history(&self, ctx: &MessageContext, current_message: &str) -> Vec<ChatMessage> {
        let mut messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: APPCONFIG.llm.system_prompt.clone(),
            }
        ];

        // 根据消息环境创建会话ID
        let session_id = match &ctx.env {
            Env::Private => SessionId::Private(ctx.sender_id as UserId),
            Env::Group { group_id, .. } => SessionId::Group(*group_id as GroupId),
        };

        // 获取历史对话
        let history = if let Some(session) = crate::CONVERSATION_CACHE.get(&session_id).await {
            let timeout = APPCONFIG.cache.conversation_timeout_minutes.unwrap_or(10);
            if !session.is_expired(timeout) {
                session.get_recent_messages(10)
            } else {
                vec![]
            }
        } else {
            vec![]
        };
        
        // 将历史对话转换为ChatMessage格式
        for conv_msg in history {
            messages.push(ChatMessage {
                role: conv_msg.role.clone(),
                content: conv_msg.content,
            });
        }

        // 添加当前用户消息
        messages.push(ChatMessage {
            role: "user".to_string(),
            content: current_message.to_string(),
        });

        messages
    }

    async fn call_llm_api(&self, messages: Vec<ChatMessage>) -> Result<String, ReplyError> {
        let request = ChatRequest {
            model: self.model.clone(),
            messages,
            temperature: APPCONFIG.llm.temperature,
            max_tokens: Some(APPCONFIG.llm.max_tokens),
            top_p: Some(APPCONFIG.llm.top_p),
            stream: false,
        };

        let url = format!("{}/chat/completions", self.base_url);
        
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| ReplyError(format!("LLM API request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(ReplyError(format!(
                "LLM API returned error {}: {}",
                status, error_text
            )));
        }

        let chat_response: ChatResponse = response
            .json()
            .await
            .map_err(|e| ReplyError(format!("Failed to parse LLM response: {}", e)))?;

        if let Some(choice) = chat_response.choices.first() {
            Ok(choice.message.content.clone())
        } else {
            Err(ReplyError("No response from LLM".to_string()))
        }
    }
}

impl RelyStrategy for LlmReplyStrategy {
    async fn reply(&self, ctx: &MessageContext) -> Result<MessageContent, ReplyError> {
        match &ctx.message {
            MessageContent::Text(text) => {
                // 根据消息环境创建会话ID
                let session_id = match &ctx.env {
                    Env::Private => SessionId::Private(ctx.sender_id as UserId),
                    Env::Group { group_id, .. } => SessionId::Group(*group_id as GroupId),
                };

                // 先记录用户消息到对话历史
                let mut session = if let Some(existing_session) = crate::CONVERSATION_CACHE.get(&session_id).await {
                    existing_session
                } else {
                    let max_history = APPCONFIG.cache.max_conversation_history.unwrap_or(20);
                    crate::ConversationSession::new(max_history)
                };

                // 添加用户消息
                let user_message = crate::ConversationMessage {
                    role: "user".to_string(),
                    content: text.clone(),
                    timestamp: chrono::Utc::now(),
                    user_id: Some(ctx.sender_id as UserId),
                    username: None, // 这里可以从上下文获取用户名
                };
                session.messages.push_back(user_message);
                session.last_activity = chrono::Utc::now();
                
                // 保持历史记录数量在限制内
                if session.messages.len() > session.max_history {
                    session.messages.pop_front();
                }
                // 获取对话历史并调用LLM
                let messages = self.get_conversation_history(ctx, text).await;
                let response = self.call_llm_api(messages).await?;
                
                // 记录助手回复到对话历史
                let assistant_message = crate::ConversationMessage {
                    role: "assistant".to_string(),
                    content: response.clone(),
                    timestamp: chrono::Utc::now(),
                    user_id: None, // 助手消息没有用户ID
                    username: Some("Assistant".to_string()),
                };
                session.messages.push_back(assistant_message);
                session.last_activity = chrono::Utc::now();
                
                // 保持历史记录数量在限制内
                if session.messages.len() > session.max_history {
                    session.messages.pop_front();
                }

                // 保存会话到缓存
                crate::CONVERSATION_CACHE.insert(session_id, session).await;
                
                Ok(MessageContent::Text(response))
            }
            MessageContent::Image(_) => {
                Err(ReplyError("Image messages are not supported yet".to_string()))
            }
            MessageContent::File(_) => {
                Err(ReplyError("File messages are not supported yet".to_string()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // 测试被注释掉了，所以不需要导入
    #[tokio::test]
    async fn test_llm_reply() {
        // 这里需要有效的API密钥才能运行
        // let strategy = LlmReplyStrategy::new();
        // let ctx = MessageContext {
        //     env: Env::Private,
        //     sender_id: 123456,
        //     self_id: 987654,
        //     message: MessageContent::Text("你好".to_string()),
        //     history: vec![],
        // };
        // let result = strategy.reply(&ctx).await;
        // assert!(result.is_ok());
    }
}

use super::{Env, MessageContent, MessageContext, RelyStrategy, ReplyError};
use crate::{GroupId, SessionId, UserId, config::APPCONFIG, service::user_config_service::UserConfigService, service::group_config_service::GroupConfigService};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use sea_orm::Database;

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
        }
    }

    async fn get_conversation_history(
        &self,
        ctx: &MessageContext,
        session_id: SessionId,
        custom_prompt: Option<String>,
    ) -> Vec<ChatMessage> {
        let system_prompt = custom_prompt.unwrap_or_else(|| APPCONFIG.llm.system_prompt.clone());
        let mut messages = vec![ChatMessage {
            role: "system".to_string(),
            content: system_prompt,
        }];

        // 获取历史对话 - 使用 ConversationManager 简化逻辑
        let history = crate::conversation::ConversationManager::get_conversation_history(
            session_id, 10
        ).await;

        // 将历史对话转换为ChatMessage格式，在群聊中包含用户信息
        for conv_msg in history {
            let content = match &ctx.env {
                Env::Group { .. } => {
                    // 在群聊中，为每条消息添加用户标识
                    if conv_msg.role == "user" {
                        if let Some(user_id) = conv_msg.user_id {
                            let default_username = format!("用户{}", user_id);
                            let username = conv_msg.username.as_deref()
                                .unwrap_or(&default_username);
                            format!("[{}]: {}", username, conv_msg.content)
                        } else {
                            conv_msg.content.clone()
                        }
                    } else {
                        conv_msg.content.clone()
                    }
                }
                Env::Private => conv_msg.content.clone(),
            };

            messages.push(ChatMessage {
                role: conv_msg.role.clone(),
                content,
            });
        }

        messages
    }

    async fn get_custom_prompt(&self, ctx: &MessageContext) -> Result<Option<String>, ReplyError> {
        // 连接数据库
        let database_url = &APPCONFIG.database.url;
        let db = Database::connect(database_url).await
            .map_err(|e| ReplyError(format!("数据库连接失败: {}", e)))?;
        
        match &ctx.env {
            Env::Group { group_id } => {
                // 群聊环境：优先使用群组配置，如果没有则使用用户配置
                let group_config_service = GroupConfigService::new(db.clone());
                match group_config_service.get_group_data(*group_id).await {
                    Ok(group_data) => {
                        if group_data.custom_prompt.is_some() {
                            Ok(group_data.custom_prompt)
                        } else {
                            // 群组没有自定义提示词，使用用户配置
                            let user_config_service = UserConfigService::new(db);
                            match user_config_service.get_user_data(ctx.sender_id).await {
                                Ok(user_data) => Ok(user_data.custom_prompt),
                                Err(_) => Ok(None),
                            }
                        }
                    }
                    Err(_) => {
                        // 群组配置获取失败，使用用户配置
                        let user_config_service = UserConfigService::new(db);
                        match user_config_service.get_user_data(ctx.sender_id).await {
                            Ok(user_data) => Ok(user_data.custom_prompt),
                            Err(_) => Ok(None),
                        }
                    }
                }
            }
            Env::Private => {
                // 私聊环境：使用用户配置
                let user_config_service = UserConfigService::new(db);
                match user_config_service.get_user_data(ctx.sender_id).await {
                    Ok(user_data) => Ok(user_data.custom_prompt),
                    Err(_) => Ok(None),
                }
            }
        }
    }

    async fn get_model_name(&self, ctx: &MessageContext) -> Result<String, ReplyError> {
        // 连接数据库
        let database_url = &APPCONFIG.database.url;
        let db = Database::connect(database_url).await
            .map_err(|e| ReplyError(format!("数据库连接失败: {}", e)))?;
        
        match &ctx.env {
            Env::Group { group_id } => {
                // 群聊环境：优先使用群组配置，如果没有则使用用户配置，最后使用默认配置
                let group_config_service = GroupConfigService::new(db.clone());
                match group_config_service.get_group_data(*group_id).await {
                    Ok(group_data) => {
                        if !group_data.model.is_empty() {
                            Ok(group_data.model)
                        } else {
                            // 群组没有设置模型，使用用户配置
                            let user_config_service = UserConfigService::new(db);
                            match user_config_service.get_user_data(ctx.sender_id).await {
                                Ok(user_data) => {
                                    if !user_data.model.is_empty() {
                                        Ok(user_data.model)
                                    } else {
                                        Ok(APPCONFIG.llm.model.clone())
                                    }
                                }
                                Err(_) => Ok(APPCONFIG.llm.model.clone()),
                            }
                        }
                    }
                    Err(_) => {
                        // 群组配置获取失败，使用用户配置
                        let user_config_service = UserConfigService::new(db);
                        match user_config_service.get_user_data(ctx.sender_id).await {
                            Ok(user_data) => {
                                if !user_data.model.is_empty() {
                                    Ok(user_data.model)
                                } else {
                                    Ok(APPCONFIG.llm.model.clone())
                                }
                            }
                            Err(_) => Ok(APPCONFIG.llm.model.clone()),
                        }
                    }
                }
            }
            Env::Private => {
                // 私聊环境：使用用户配置，如果没有则使用默认配置
                let user_config_service = UserConfigService::new(db);
                match user_config_service.get_user_data(ctx.sender_id).await {
                    Ok(user_data) => {
                        if !user_data.model.is_empty() {
                            Ok(user_data.model)
                        } else {
                            Ok(APPCONFIG.llm.model.clone())
                        }
                    }
                    Err(_) => Ok(APPCONFIG.llm.model.clone()),
                }
            }
        }
    }

    async fn call_llm_api(&self, messages: Vec<ChatMessage>, model: String) -> Result<String, ReplyError> {
        let request = ChatRequest {
            model,
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

                // 获取自定义提示词（群组优先或用户配置）
                let custom_prompt = match self.get_custom_prompt(ctx).await {
                    Ok(prompt) => prompt,
                    Err(_) => None, // 如果获取失败，使用默认提示词
                };

                // 获取模型名称（群组优先或用户配置）
                let model_name = match self.get_model_name(ctx).await {
                    Ok(model) => model,
                    Err(_) => APPCONFIG.llm.model.clone(), // 如果获取失败，使用默认模型
                };

                // 先记录用户消息到对话历史，使用统一的用户名格式
                let username = match &ctx.env {
                    Env::Group { .. } => {
                        ctx.sender_name.clone()
                            .unwrap_or_else(|| format!("用户{}", ctx.sender_id))
                    }
                    Env::Private => format!("用户{}", ctx.sender_id),
                };

                crate::conversation::ConversationManager::add_user_message_with_info(
                    session_id.clone(),
                    text.clone(),
                    ctx.sender_id as UserId,
                    match &ctx.env {
                        Env::Group { .. } => Some(username),
                        Env::Private => None,
                    },
                ).await;

                // 获取对话历史（包含刚刚添加的当前消息）
                let messages = self.get_conversation_history(ctx, session_id.clone(), custom_prompt).await;
                let response = self.call_llm_api(messages, model_name).await?;

                // 记录助手回复到对话历史
                crate::conversation::ConversationManager::add_assistant_message(
                    session_id,
                    response.clone(),
                ).await;

                Ok(MessageContent::Text(response))
            }
            MessageContent::Image(_) => Err(ReplyError(
                "Image messages are not supported yet".to_string(),
            )),
            MessageContent::File(_) => Err(ReplyError(
                "File messages are not supported yet".to_string(),
            )),
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

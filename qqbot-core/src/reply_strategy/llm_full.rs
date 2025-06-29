use super::{Env, MessageContent, MessageContext, RelyStrategy, ReplyError, FileAttachment};
use crate::{GroupId, SessionId, UserId, config::APPCONFIG, service::user_config_service::UserConfigService, service::group_config_service::GroupConfigService};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use sea_orm::Database;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_data: Option<ImageData>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImageData {
    pub file: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_package_id: Option<String>,
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
            image_data: None,
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
                image_data: None,
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

    async fn handle_image_message(
        &self,
        ctx: &MessageContext,
        attachment: &FileAttachment,
    ) -> Result<MessageContent, ReplyError> {
        // 根据消息环境创建会话ID
        let session_id = match &ctx.env {
            Env::Private => SessionId::Private(ctx.sender_id as UserId),
            Env::Group { group_id, .. } => SessionId::Group(*group_id as GroupId),
        };

        // 创建图片数据，从attachment中提取OneBot11协议所需的字段
        let image_data = self.extract_image_data_from_attachment(attachment)?;
        
        // 获取自定义提示词（群组优先或用户配置）
        let custom_prompt = match self.get_custom_prompt(ctx).await {
            Ok(prompt) => prompt,
            Err(_) => None, // 如果获取失败，使用默认提示词
        };

        // 记录图片消息到对话历史
        let username = match &ctx.env {
            Env::Group { .. } => {
                ctx.sender_name.clone()
                    .unwrap_or_else(|| format!("用户{}", ctx.sender_id))
            }
            Env::Private => format!("用户{}", ctx.sender_id),
        };

        let image_description = format!("[图片: {}]", attachment.name);
        crate::conversation::ConversationManager::add_user_message_with_info(
            session_id.clone(),
            image_description.clone(),
            ctx.sender_id as UserId,
            match &ctx.env {
                Env::Group { .. } => Some(username),
                Env::Private => None,
            },
        ).await;

        // 构建包含图片信息的消息
        let mut messages = self.get_conversation_history(ctx, session_id.clone(), custom_prompt).await;
        
        // 为最后一条用户消息添加图片数据
        if let Some(last_message) = messages.last_mut() {
            if last_message.role == "user" {
                last_message.image_data = Some(image_data.clone());
                // 如果图片有描述信息，也包含在内容中
                if let Some(summary) = &image_data.summary {
                    last_message.content = format!("{} (图片描述: {})", last_message.content, summary);
                }
            }
        }

        let response = self.call_llm_api(messages).await?;

        // 记录助手回复到对话历史
        crate::conversation::ConversationManager::add_assistant_message(
            session_id,
            response.clone(),
        ).await;

        Ok(MessageContent::Text(response))
    }

    fn extract_image_data_from_attachment(&self, attachment: &FileAttachment) -> Result<ImageData, ReplyError> {
        // 从文件名中提取信息，如果文件名包含OneBot11协议的字段信息
        // 这里假设attachment.name可能包含一些元数据，或者从mime_type推断
        
        // 基本的图片数据结构
        let mut image_data = ImageData {
            file: attachment.name.clone(),
            url: None,
            summary: None,
            sub_type: None,
            file_size: Some(attachment.content.len() as u64),
            key: None,
            emoji_id: None,
            emoji_package_id: None,
        };

        // 尝试从mime_type判断图片类型
        if let Some(mime_type) = &attachment.mime_type {
            match mime_type.as_str() {
                "image/jpeg" | "image/jpg" => image_data.sub_type = Some(0),
                "image/png" => image_data.sub_type = Some(1),
                "image/gif" => image_data.sub_type = Some(2),
                "image/webp" => image_data.sub_type = Some(3),
                _ => image_data.sub_type = Some(99), // 未知类型
            }
        }

        // 如果是表情相关的文件名，尝试提取表情信息
        if attachment.name.contains("emoji") || attachment.name.contains("sticker") {
            // 尝试从文件名中提取表情ID等信息
            // 这里可以根据实际的文件名格式来解析
            if let Some(emoji_info) = self.parse_emoji_info(&attachment.name) {
                image_data.emoji_id = emoji_info.emoji_id;
                image_data.emoji_package_id = emoji_info.emoji_package_id;
                image_data.key = emoji_info.key;
            }
        }

        Ok(image_data)
    }

    fn parse_emoji_info(&self, _filename: &str) -> Option<EmojiInfo> {
        // 简化的表情信息解析实现
        // 这里可以根据实际的文件名格式来解析表情ID等信息
        None // 暂时返回None，可以后续根据需要实现
    }
}

// 表情信息结构
#[derive(Debug)]
struct EmojiInfo {
    pub emoji_id: Option<String>,
    pub emoji_package_id: Option<String>,
    pub key: Option<String>,
}

// 实现LLM回复策略
impl RelyStrategy for LlmReplyStrategy {
    async fn reply(&self, ctx: &MessageContext) -> Result<MessageContent, ReplyError> {
        match &ctx.message {
            MessageContent::Text(text) => {
                // 处理纯文本消息
                self.handle_text_message(ctx, text).await
            },
            MessageContent::Mixed(segments) => {
                // 处理混合消息（图文并茂）
                self.handle_mixed_message(ctx, segments).await
            },
            MessageContent::Image(attachment) => {
                // 处理纯图片消息（向后兼容）
                self.handle_image_message(ctx, attachment).await
            },
            MessageContent::File(_) => {
                Err(ReplyError("File messages are not supported yet".to_string()))
            }
        }
    }
}

impl LlmReplyStrategy {
    async fn handle_text_message(&self, ctx: &MessageContext, text: &str) -> Result<MessageContent, ReplyError> {
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

        // 记录用户消息到对话历史
        let username = match &ctx.env {
            Env::Group { .. } => {
                ctx.sender_name.clone()
                    .unwrap_or_else(|| format!("用户{}", ctx.sender_id))
            }
            Env::Private => format!("用户{}", ctx.sender_id),
        };

        crate::conversation::ConversationManager::add_user_message_with_info(
            session_id.clone(),
            text.to_string(),
            ctx.sender_id as UserId,
            match &ctx.env {
                Env::Group { .. } => Some(username),
                Env::Private => None,
            },
        ).await;

        // 构建消息
        let messages = self.get_conversation_history(ctx, session_id.clone(), custom_prompt).await;
        let response = self.call_llm_api(messages).await?;

        // 记录助手回复到对话历史
        crate::conversation::ConversationManager::add_assistant_message(
            session_id,
            response.clone(),
        ).await;

        Ok(MessageContent::Text(response))
    }

    async fn handle_mixed_message(&self, ctx: &MessageContext, segments: &[super::MessageSegment]) -> Result<MessageContent, ReplyError> {
        use super::MessageSegment;
        
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

        // 构建描述信息，包含文本和图片信息
        let mut content_parts = Vec::new();
        let mut has_image = false;
        let mut image_data_list = Vec::new();

        for segment in segments {
            match segment {
                MessageSegment::Text { text } => {
                    if !text.trim().is_empty() {
                        content_parts.push(text.clone());
                    }
                },
                MessageSegment::Image { image_info } => {
                    has_image = true;
                    content_parts.push(format!("[图片: {}]", image_info.file));
                    
                    // 转换ImageInfo为ImageData
                    let image_data = ImageData {
                        file: image_info.file.clone(),
                        url: image_info.url.clone(),
                        summary: image_info.summary.clone(),
                        sub_type: image_info.sub_type,
                        file_size: image_info.file_size,
                        key: image_info.key.clone(),
                        emoji_id: image_info.emoji_id.clone(),
                        emoji_package_id: image_info.emoji_package_id.clone(),
                    };
                    image_data_list.push(image_data);
                },
                MessageSegment::At { qq } => {
                    content_parts.push(format!("[@{}]", qq));
                },
                MessageSegment::Face { id } => {
                    content_parts.push(format!("[表情{}]", id));
                },
            }
        }

        let content_description = content_parts.join(" ");

        // 记录消息到对话历史
        let username = match &ctx.env {
            Env::Group { .. } => {
                ctx.sender_name.clone()
                    .unwrap_or_else(|| format!("用户{}", ctx.sender_id))
            }
            Env::Private => format!("用户{}", ctx.sender_id),
        };

        crate::conversation::ConversationManager::add_user_message_with_info(
            session_id.clone(),
            content_description.clone(),
            ctx.sender_id as UserId,
            match &ctx.env {
                Env::Group { .. } => Some(username),
                Env::Private => None,
            },
        ).await;

        // 构建包含图片信息的消息
        let mut messages = self.get_conversation_history(ctx, session_id.clone(), custom_prompt).await;
        
        // 如果有图片，为最后一条用户消息添加第一张图片的数据
        if has_image && !image_data_list.is_empty() {
            if let Some(last_message) = messages.last_mut() {
                if last_message.role == "user" {
                    last_message.image_data = Some(image_data_list[0].clone());
                    
                    // 如果图片有描述信息，也包含在内容中
                    if let Some(summary) = &image_data_list[0].summary {
                        last_message.content = format!("{} (图片描述: {})", last_message.content, summary);
                    }
                }
            }
        }

        let response = self.call_llm_api(messages).await?;

        // 记录助手回复到对话历史
        crate::conversation::ConversationManager::add_assistant_message(
            session_id,
            response.clone(),
        ).await;

        Ok(MessageContent::Text(response))
    }
}
use super::{Env, MessageContent, MessageContext, RelyStrategy, ReplyError, MessageSegment, ImageInfo};
use crate::{GroupId, SessionId, UserId, config::APPCONFIG};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// LLM API 的图片数据结构，对应 OneBot11 协议
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

/// LLM API 聊天消息结构
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: ChatContent,
}

/// 聊天消息内容，支持文本和图片
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ChatContent {
    Text(String),
    Array(Vec<ContentPart>),
}

/// 消息内容部分，支持文本和图片
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ContentPart {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image_url")]
    ImageUrl { image_url: ImageUrl },
}

/// 图片URL结构
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImageUrl {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>, // "low", "high", "auto"
}

/// LLM API 请求结构
#[derive(Debug, Serialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub temperature: f32,
    pub max_tokens: Option<u32>,
    pub top_p: Option<f32>,
    pub stream: bool,
}

/// LLM API 响应结构
#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub choices: Vec<ChatChoice>,
}

#[derive(Debug, Deserialize)]
pub struct ChatChoice {
    pub message: ChatMessage,
}

/// 简化的 LLM 回复策略，专注于图片处理
#[derive(Clone)]
pub struct SimpleLlmReplyStrategy {
    client: Client,
    api_key: String,
    base_url: String,
    model: String,
}

impl SimpleLlmReplyStrategy {
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

    /// 将 ImageInfo 转换为 ImageData
    fn convert_image_info_to_data(&self, image_info: &ImageInfo) -> ImageData {
        ImageData {
            file: image_info.file.clone(),
            url: image_info.url.clone(),
            summary: image_info.summary.clone(),
            sub_type: image_info.sub_type,
            file_size: image_info.file_size,
            key: image_info.key.clone(),
            emoji_id: image_info.emoji_id.clone(),
            emoji_package_id: image_info.emoji_package_id.clone(),
        }
    }

    /// 从消息内容提取文本和图片
    fn extract_content_and_images(&self, message: &MessageContent) -> (String, Vec<ImageData>) {
        match message {
            MessageContent::Text(text) => (text.clone(), Vec::new()),
            MessageContent::Mixed(segments) => {
                let mut text_parts = Vec::new();
                let mut images = Vec::new();
                
                for segment in segments {
                    match segment {
                        MessageSegment::Text { text } => {
                            if !text.trim().is_empty() {
                                text_parts.push(text.clone());
                            }
                        },
                        MessageSegment::Image { image_info } => {
                            text_parts.push(format!("[图片: {}]", image_info.file));
                            images.push(self.convert_image_info_to_data(image_info));
                        },
                        MessageSegment::At { qq } => {
                            text_parts.push(format!("[@{}]", qq));
                        },
                        MessageSegment::Face { id } => {
                            text_parts.push(format!("[表情{}]", id));
                        },
                    }
                }
                
                (text_parts.join(" "), images)
            },
            _ => (String::new(), Vec::new()),
        }
    }

    /// 调用 LLM API（支持视觉模型）
    async fn call_llm_api(&self, content: String, image: Option<ImageData>) -> Result<String, ReplyError> {
        // 构建消息内容
        let user_content = if let Some(image_data) = image {
            // 如果有图片，构建包含图片的消息
            if let Some(image_url) = &image_data.url {
                // 检查是否是HTTP URL，如果是则直接使用，否则尝试下载并转换为base64
                let final_image_url = if image_url.starts_with("http://") || image_url.starts_with("https://") {
                    image_url.clone()
                } else {
                    // 如果URL不是标准HTTP URL，尝试下载并转换为base64
                    match self.download_and_encode_image(image_url).await {
                        Ok(base64_url) => base64_url,
                        Err(_) => {
                            // 下载失败，回退到文本描述
                            return Ok(format!("{} [图片: {}, 无法加载]", content, image_data.file));
                        }
                    }
                };
                
                // 使用图片URL构建视觉消息
                ChatContent::Array(vec![
                    ContentPart::Text { text: content },
                    ContentPart::ImageUrl { 
                        image_url: ImageUrl { 
                            url: final_image_url,
                            detail: Some("high".to_string()) // 高清晰度识别
                        }
                    },
                ])
            } else {
                // 如果没有URL，仅使用文本描述
                let description = format!("{} [图片文件: {}]", content, image_data.file);
                ChatContent::Text(description)
            }
        } else {
            // 纯文本消息
            ChatContent::Text(content)
        };
        
        let messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: ChatContent::Text(APPCONFIG.llm.system_prompt.clone()),
            },
            ChatMessage {
                role: "user".to_string(),
                content: user_content,
            },
        ];

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
            // 提取响应内容
            match &choice.message.content {
                ChatContent::Text(text) => Ok(text.clone()),
                ChatContent::Array(parts) => {
                    // 如果是数组格式，提取所有文本部分
                    let text_parts: Vec<String> = parts
                        .iter()
                        .filter_map(|part| match part {
                            ContentPart::Text { text } => Some(text.clone()),
                            _ => None,
                        })
                        .collect();
                    Ok(text_parts.join(" "))
                }
            }
        } else {
            Err(ReplyError("No response from LLM".to_string()))
        }
    }

    /// 下载图片并转换为Base64编码
    async fn download_and_encode_image(&self, image_url: &str) -> Result<String, ReplyError> {
        let response = self
            .client
            .get(image_url)
            .send()
            .await
            .map_err(|e| ReplyError(format!("Failed to download image: {}", e)))?;

        if !response.status().is_success() {
            return Err(ReplyError(format!(
                "Failed to download image, status: {}",
                response.status()
            )));
        }

        let image_bytes = response
            .bytes()
            .await
            .map_err(|e| ReplyError(format!("Failed to read image bytes: {}", e)))?;

        // 编码为Base64
        use base64::{Engine as _, engine::general_purpose};
        let base64_image = general_purpose::STANDARD.encode(&image_bytes);
        
        // 尝试检测图片格式
        let content_type = if image_url.to_lowercase().ends_with(".png") {
            "image/png"
        } else if image_url.to_lowercase().ends_with(".jpg") || image_url.to_lowercase().ends_with(".jpeg") {
            "image/jpeg"
        } else if image_url.to_lowercase().ends_with(".gif") {
            "image/gif"
        } else if image_url.to_lowercase().ends_with(".webp") {
            "image/webp"
        } else {
            "image/jpeg" // 默认
        };

        Ok(format!("data:{};base64,{}", content_type, base64_image))
    }

    /// 记录消息到对话历史
    async fn log_message(&self, ctx: &MessageContext, content: &str) {
        let session_id = match &ctx.env {
            Env::Private => SessionId::Private(ctx.sender_id as UserId),
            Env::Group { group_id } => SessionId::Group(*group_id as GroupId),
        };

        let username = match &ctx.env {
            Env::Group { .. } => ctx.sender_name.clone()
                .unwrap_or_else(|| format!("用户{}", ctx.sender_id)),
            Env::Private => format!("用户{}", ctx.sender_id),
        };

        crate::conversation::ConversationManager::add_user_message_with_info(
            session_id.clone(),
            content.to_string(),
            ctx.sender_id as UserId,
            match &ctx.env {
                Env::Group { .. } => Some(username),
                Env::Private => None,
            },
        ).await;
    }

    /// 记录回复到对话历史
    async fn log_reply(&self, ctx: &MessageContext, response: &str) {
        let session_id = match &ctx.env {
            Env::Private => SessionId::Private(ctx.sender_id as UserId),
            Env::Group { group_id } => SessionId::Group(*group_id as GroupId),
        };

        crate::conversation::ConversationManager::add_assistant_message(
            session_id,
            response.to_string(),
        ).await;
    }
}

impl RelyStrategy for SimpleLlmReplyStrategy {
    async fn reply(&self, ctx: &MessageContext) -> Result<MessageContent, ReplyError> {
        // 提取消息内容和图片
        let (content, images) = self.extract_content_and_images(&ctx.message);
        
        // 如果没有内容，返回错误
        if content.trim().is_empty() && images.is_empty() {
            return Err(ReplyError("空消息内容".to_string()));
        }

        // 记录用户消息
        self.log_message(ctx, &content).await;

        // 处理图片（如果有的话，使用第一张图片）
        let image_data = images.first().cloned();
        
        // 调用 LLM API
        let response = self.call_llm_api(content, image_data).await?;

        // 记录回复
        self.log_reply(ctx, &response).await;

        Ok(MessageContent::Text(response))
    }
}

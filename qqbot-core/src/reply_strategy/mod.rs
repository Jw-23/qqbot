use crate::error::AppError;
use core::fmt;

pub mod cmd;
pub mod llm;
pub mod llm_full;
pub mod reply_manager;
#[derive(Debug, Clone)]
pub enum Env {
    Group { group_id: i64 },
    Private,
}
impl fmt::Display for Env {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Env::Group { .. } => write!(f, "group"),
            Env::Private => write!(f, "private"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FileAttachment {
    pub name: String,
    pub content: Vec<u8>,
    pub mime_type: Option<String>,
}

// OneBot11 图片信息结构
#[derive(Debug, Clone)]
pub struct ImageInfo {
    pub file: String,
    pub url: Option<String>,
    pub summary: Option<String>,
    pub sub_type: Option<u32>,
    pub file_size: Option<u64>,
    pub key: Option<String>,
    pub emoji_id: Option<String>,
    pub emoji_package_id: Option<String>,
}

// 消息段类型，对应OneBot11协议的消息段
#[derive(Debug, Clone)]
pub enum MessageSegment {
    Text { text: String },
    Image { image_info: ImageInfo },
    At { qq: String },
    Face { id: u32 },
    // 可以根据需要添加更多消息段类型
}

#[derive(Debug, Clone)]
pub enum MessageContent {
    Text(String),                           // 纯文本消息（向后兼容）
    Image(FileAttachment),                  // 纯图片消息（向后兼容）
    File(FileAttachment),                   // 文件消息（向后兼容）
    Mixed(Vec<MessageSegment>),             // 混合消息（图文并茂）
}

impl MessageContent {
    pub fn has_text(&self) -> bool {
        match self {
            MessageContent::Text(text) => !text.trim().is_empty(),
            MessageContent::Mixed(segments) => {
                segments.iter().any(|seg| matches!(seg, MessageSegment::Text { .. }))
            },
            _ => false,
        }
    }
    
    pub fn get_text(&self) -> String {
        match self {
            MessageContent::Text(text) => text.clone(),
            MessageContent::Mixed(segments) => {
                segments.iter()
                    .filter_map(|seg| match seg {
                        MessageSegment::Text { text } => Some(text.as_str()),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
                    .join(" ")
            },
            _ => String::new(),
        }
    }
    
    pub fn has_image(&self) -> bool {
        match self {
            MessageContent::Image(_) => true,
            MessageContent::Mixed(segments) => {
                segments.iter().any(|seg| matches!(seg, MessageSegment::Image { .. }))
            },
            _ => false,
        }
    }
    
    pub fn get_images(&self) -> Vec<&ImageInfo> {
        match self {
            MessageContent::Mixed(segments) => {
                segments.iter()
                    .filter_map(|seg| match seg {
                        MessageSegment::Image { image_info } => Some(image_info),
                        _ => None,
                    })
                    .collect()
            },
            _ => Vec::new(),
        }
    }
}
#[derive(Debug)]
pub struct MessageContext {
    pub env: Env,
    pub sender_id: i64,
    pub self_id: i64,
    pub group_admin: bool,
    pub message: MessageContent,
    pub history: Vec<MessageContent>,
    pub sender_name: Option<String>, // 发送者的昵称或用户名，在群聊中特别有用
}
#[derive(Debug)]
pub struct ReplyError(pub String);
impl fmt::Display for ReplyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ReplyError {}

// 提供从 ReplyError 到 AppError 的转换
impl From<ReplyError> for AppError {
    fn from(err: ReplyError) -> Self {
        AppError::reply(err.0)
    }
}

// 提供从 AppError 到 ReplyError 的转换以兼容现有代码
impl From<AppError> for ReplyError {
    fn from(err: AppError) -> Self {
        ReplyError(err.to_string())
    }
}

pub trait RelyStrategy {
    fn reply(
        &self,
        ctx: &MessageContext,
    ) -> impl std::future::Future<Output = Result<MessageContent, ReplyError>> + Send;
}

#[tokio::test]
async fn reply_message_test() -> Result<(), Box<dyn std::error::Error>> {
    use crate::config::get_db;
    use cmd::CommandReplyStrategy;

    get_db().await;
    let mc = MessageContent::Text(vec!["/query".into(), "grade"].join(" "));
    let message_context = MessageContext {
        env: Env::Private,
        sender_id: 87654321,
        self_id: 9999,
        message: mc,
        group_admin: false,
        history: vec![],
        sender_name: None,
    };
    let cmd_strategy = CommandReplyStrategy::new();
    let result = cmd_strategy.reply(&message_context).await;
    let output = match result {
        Ok(MessageContent::Text(res)) => res,
        Err(err) => err.to_string(),
        _ => "invalid reply".into(),
    };
    println!("result:\n{}", output);
    Ok(())
}

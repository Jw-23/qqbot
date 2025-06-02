use crate::error::AppError;
use core::fmt;

pub mod cmd;
pub mod llm;
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

#[derive(Debug, Clone)]
pub enum MessageContent {
    Text(String),
    Image(FileAttachment),
    File(FileAttachment),
}
#[derive(Debug)]
pub struct MessageContext {
    pub env: Env,
    pub sender_id: i64,
    pub self_id: i64,
    pub group_admin: bool,
    pub message: MessageContent,
    pub history: Vec<MessageContent>,
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

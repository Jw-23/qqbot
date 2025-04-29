use core::fmt;
use std::fmt::write;

use clap::builder::Str;
use cmd::CommandReplyStrategy;
use once_cell::sync::Lazy;
use sea_orm::sqlx::types::Text;

use crate::{cmd::CmdRegistry, config::get_db};

pub mod cmd;
pub use cmd::*;
#[derive(Debug)]
pub enum Env {
    Group,
    Private,
}
impl fmt::Display for Env{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Env::Group=>write!(f,"group"),
            Env::Private=>write!(f, "private")
        }
    }
}
#[derive(Debug)]
pub enum MessageContent {
    Text(String),
    Image(Vec<u8>),
    File(Vec<u8>),
}
#[derive(Debug)]
pub struct MessageContext {
    pub env: Env,
    pub sender_id: i64,
    pub self_id: i64,
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

pub trait RelyStrategy {
    fn reply(&self, ctx: &MessageContext) -> impl std::future::Future<Output = Result<MessageContent, ReplyError>> + Send;
}

#[tokio::test]
async fn reply_message_test() -> Result<(), Box<dyn std::error::Error>> {
    get_db().await;
    let mc = MessageContent::Text(vec!["/query".into(), "grade"].join(" "));
    let message_context = MessageContext {
        env: Env::Private,
        sender_id: 87654321,
        self_id: 9999,
        message: mc,
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

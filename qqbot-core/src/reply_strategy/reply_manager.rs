use super::{MessageContent, MessageContext, RelyStrategy, ReplyError};
use crate::{BOT_CACHE, StrategeType};
use super::cmd::CommandReplyStrategy;
use super::llm::LlmReplyStrategy;

#[derive(Clone)]
pub struct ReplyManager {
    cmd_strategy: CommandReplyStrategy,
    llm_strategy: LlmReplyStrategy,
}

impl ReplyManager {
    pub fn new() -> Self {
        Self {
            cmd_strategy: CommandReplyStrategy::new(),
            llm_strategy: LlmReplyStrategy::new(),
        }
    }

    pub async fn reply(&self, ctx: &MessageContext) -> Result<MessageContent, ReplyError> {
        // 获取用户的策略设置
        let user_data = BOT_CACHE.get(&ctx.sender_id).await.unwrap_or_default();
        
        match user_data.stratege {
            StrategeType::CmdStrategy => {
                self.cmd_strategy.reply(ctx).await
            }
            StrategeType::LlmStrategy => {
                // 对于LLM策略，如果消息不是以命令前缀开头，则使用LLM回复
                if let MessageContent::Text(text) = &ctx.message {
                    if text.starts_with(&crate::config::APPCONFIG.cmd_suffix) {
                        // 仍然是命令，使用命令策略处理
                        self.cmd_strategy.reply(ctx).await
                    } else {
                        // 普通聊天消息，使用LLM策略
                        self.llm_strategy.reply(ctx).await
                    }
                } else {
                    // 非文本消息，尝试LLM策略
                    self.llm_strategy.reply(ctx).await
                }
            }
        }
    }
}

impl Default for ReplyManager {
    fn default() -> Self {
        Self::new()
    }
}

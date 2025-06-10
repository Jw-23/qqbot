use super::cmd::CommandReplyStrategy;
use super::llm::SimpleLlmReplyStrategy;
use super::{MessageContent, MessageContext, RelyStrategy, ReplyError, Env};
use crate::{BOT_CACHE, StrategeType};
use crate::service::group_config_service::GROUP_CACHE;

#[derive(Clone)]
pub struct ReplyManager {
    cmd_strategy: CommandReplyStrategy,
    llm_strategy: SimpleLlmReplyStrategy,
}

impl ReplyManager {
    pub fn new() -> Self {
        Self {
            cmd_strategy: CommandReplyStrategy::new(),
            llm_strategy: SimpleLlmReplyStrategy::new(),
        }
    }

    pub async fn reply(&self, ctx: &MessageContext) -> Result<MessageContent, ReplyError> {
        // 根据环境获取有效配置（群组优先或用户配置）
        let strategy = match &ctx.env {
            Env::Group { group_id } => {
                // 群聊环境：优先使用群组配置，如果没有则使用用户配置
                if let Some(group_data) = GROUP_CACHE.get(group_id).await {
                    group_data.stratege
                } else {
                    // 群组没有配置，使用用户配置
                    BOT_CACHE.get(&ctx.sender_id).await.unwrap_or_default().stratege
                }
            }
            Env::Private => {
                // 私聊环境：使用用户配置
                BOT_CACHE.get(&ctx.sender_id).await.unwrap_or_default().stratege
            }
        };

        match strategy {
            StrategeType::CmdStrategy => self.cmd_strategy.reply(ctx).await,
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

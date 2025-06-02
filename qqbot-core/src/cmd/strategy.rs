use crate::{
    BOT_CACHE, StrategeType, UserData,
    cmd::{CmdHandler, CmdResult, CommonArgs, HandlerBuilder},
    config::APPCONFIG,
    error::AppError,
};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "strategy")]
#[command(about = "切换回复策略")]
pub struct Strategy {
    #[command(flatten)]
    pub common: CommonArgs,

    #[arg(help = "策略类型: cmd | llm")]
    strategy: String,
    #[arg(required = false, help = "模型", default_value_t=String::from(""))]
    model: String,
}

impl HandlerBuilder for Strategy {
    fn build() -> CmdHandler {
        Box::new(move |args: Vec<String>| {
            Box::pin(async move {
                let strategy = Strategy::try_parse_from(args)
                    .map_err(|err| AppError::command(err.to_string()))?;

                // 检查是否为管理员或者用户自己
                if !strategy.common.group_admin && strategy.common.env == String::from("group") {
                    return Err(AppError::command("群聊中只有管理员能使用".to_string()));
                }

                let strategy_type = match strategy.strategy.to_lowercase().as_str() {
                    "cmd" => StrategeType::CmdStrategy,
                    "llm" => StrategeType::LlmStrategy,
                    _ => {
                        return Err(AppError::command(
                            "无效的策略类型，支持的策略: cmd, llm".to_string(),
                        ));
                    }
                };
                let default_model = &APPCONFIG.llm.model;
                // 更新用户策略
                let user_data = UserData {
                    stratege: strategy_type,
                    model: if strategy.model.is_empty() {
                        default_model.clone()
                    } else {
                        strategy.model
                    },
                };

                BOT_CACHE.insert(strategy.common.sender, user_data).await;

                let strategy_name = match strategy_type {
                    StrategeType::CmdStrategy => "命令模式",
                    StrategeType::LlmStrategy => "大模型聊天模式",
                };

                Ok(CmdResult {
                    output: format!("已成功切换到{}！", strategy_name),
                })
            })
        })
    }
}

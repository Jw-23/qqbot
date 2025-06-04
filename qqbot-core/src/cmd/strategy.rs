use crate::{
    StrategeType,
    cmd::{CmdHandler, CmdResult, CommonArgs, HandlerBuilder},
    config::APPCONFIG,
    error::AppError,
    service::user_config_service::UserConfigService,
};
use clap::{Parser, Subcommand};
use sea_orm::Database;

#[derive(Parser, Debug)]
#[command(name = "strategy")]
#[command(about = "切换回复策略")]
pub struct Strategy {
    #[command(flatten)]
    pub common: CommonArgs,

    #[command(subcommand)]
    command: StrategyCommand,
}

#[derive(Subcommand, Debug)]
pub enum StrategyCommand {
    /// 切换到命令模式
    #[command(name = "cmd")]
    Cmd,
    /// 切换到大模型聊天模式
    #[command(name = "llm")]
    Llm {
        /// 指定使用的模型名称
        #[arg(short, long, help = "模型名称")]
        model: Option<String>,
        /// 设置自定义提示词
        #[arg(short, long, help = "自定义提示词")]
        prompt: Option<String>,
        /// 重置为默认提示词
        #[arg(long, help = "重置为默认提示词")]
        reset_prompt: bool,
    },
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

                // 初始化数据库连接
                let database_url = &APPCONFIG.database.url;
                let db = Database::connect(database_url).await
                    .map_err(|e| AppError::Database(e))?;
                
                let user_config_service = UserConfigService::new(db);
                let user_id = strategy.common.sender;

                // 获取当前用户配置
                let mut user_data = user_config_service.get_user_data(user_id).await
                    .unwrap_or_default();

                match strategy.command {
                    StrategyCommand::Cmd => {
                        // 切换到命令模式
                        user_data.stratege = StrategeType::CmdStrategy;
                        
                        // 保存到数据库
                        user_config_service.save_user_data(user_id, &user_data).await
                            .map_err(|e| AppError::command(format!("保存用户配置失败: {}", e)))?;

                        Ok(CmdResult {
                            output: "✅ 已成功切换到命令模式！".to_string(),
                        })
                    }
                    StrategyCommand::Llm { model, prompt, reset_prompt } => {
                        // 切换到大模型聊天模式
                        user_data.stratege = StrategeType::LlmStrategy;
                        
                        // 处理模型设置
                        if let Some(model_name) = model {
                            if !model_name.trim().is_empty() {
                                user_data.model = model_name;
                            }
                        } else if user_data.model.is_empty() {
                            user_data.model = APPCONFIG.llm.model.clone();
                        }

                        // 处理提示词设置
                        let mut messages = vec!["✅ 已成功切换到大模型聊天模式！".to_string()];
                        
                        if reset_prompt {
                            user_data.custom_prompt = None;
                            messages.push("🔄 已重置为默认系统提示词".to_string());
                        } else if let Some(custom_prompt) = prompt {
                            if custom_prompt.trim().is_empty() {
                                return Err(AppError::command("提示词内容不能为空".to_string()));
                            }
                            
                            if custom_prompt.len() > 2000 {
                                return Err(AppError::command("提示词长度不能超过2000个字符".to_string()));
                            }
                            
                            user_data.custom_prompt = Some(custom_prompt.clone());
                            messages.push(format!("📝 自定义提示词已设置:\n{}", custom_prompt));
                        }

                        // 保存到数据库
                        user_config_service.save_user_data(user_id, &user_data).await
                            .map_err(|e| AppError::command(format!("保存用户配置失败: {}", e)))?;

                        // 显示当前配置信息
                        messages.push(format!("🤖 当前模型: {}", user_data.model));
                        
                        match &user_data.custom_prompt {
                            Some(custom) => {
                                messages.push(format!("📝 当前提示词: 自定义\n{}", custom));
                            }
                            None => {
                                messages.push("📝 当前提示词: 系统默认".to_string());
                            }
                        }

                        Ok(CmdResult {
                            output: messages.join("\n\n"),
                        })
                    }
                }
            })
        })
    }
}

use crate::{
    StrategeType,
    cmd::{CmdHandler, CmdResult, CommonArgs, HandlerBuilder},
    config::APPCONFIG,
    error::AppError,
    service::user_config_service::UserConfigService,
    service::group_config_service::GroupConfigService,
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
    /// 查询当前配置
    #[command(name = "query")]
    Query,
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
                
                // 根据环境类型决定操作用户配置还是群组配置
                if strategy.common.env == String::from("group") {
                    // 群聊环境：操作群组配置
                    let group_config_service = GroupConfigService::new(db);
                    let group_id = strategy.common.group_id;
                    
                    if group_id == 0 {
                        return Err(AppError::command("群组ID无效".to_string()));
                    }
                    
                    let mut group_data = group_config_service.get_group_data(group_id).await
                        .unwrap_or_default();

                    match strategy.command {
                        StrategyCommand::Cmd => {
                            // 切换群组到命令模式
                            group_data.stratege = StrategeType::CmdStrategy;
                            
                            // 保存到数据库
                            group_config_service.save_group_data(group_id, &group_data).await
                                .map_err(|e| AppError::command(format!("保存群组配置失败: {}", e)))?;

                            Ok(CmdResult {
                                output: "✅ 已成功切换群组到命令模式！".to_string(),
                            })
                        }
                        StrategyCommand::Llm { model, prompt, reset_prompt } => {
                            // 切换群组到大模型聊天模式
                            group_data.stratege = StrategeType::LlmStrategy;
                            
                            // 处理模型设置
                            if let Some(model_name) = model {
                                if !model_name.trim().is_empty() {
                                    group_data.model = model_name;
                                }
                            } else if group_data.model.is_empty() {
                                group_data.model = APPCONFIG.llm.model.clone();
                            }

                            // 处理提示词设置
                            let mut messages = vec!["✅ 已成功切换群组到大模型聊天模式！".to_string()];
                            
                            if reset_prompt {
                                group_data.custom_prompt = None;
                                messages.push("🔄 已重置群组为默认系统提示词".to_string());
                            } else if let Some(custom_prompt) = prompt {
                                if custom_prompt.trim().is_empty() {
                                    return Err(AppError::command("提示词内容不能为空".to_string()));
                                }
                                
                                if custom_prompt.len() > 2000 {
                                    return Err(AppError::command("提示词长度不能超过2000个字符".to_string()));
                                }
                                
                                group_data.custom_prompt = Some(custom_prompt.clone());
                                messages.push(format!("📝 群组自定义提示词已设置:\n{}", custom_prompt));
                            }

                            // 保存到数据库
                            group_config_service.save_group_data(group_id, &group_data).await
                                .map_err(|e| AppError::command(format!("保存群组配置失败: {}", e)))?;

                            // 显示当前配置信息
                            messages.push(format!("🤖 群组当前模型: {}", group_data.model));
                            
                            match &group_data.custom_prompt {
                                Some(custom) => {
                                    messages.push(format!("📝 群组当前提示词: 自定义\n{}", custom));
                                }
                                None => {
                                    messages.push("📝 群组当前提示词: 系统默认".to_string());
                                }
                            }

                            Ok(CmdResult {
                                output: messages.join("\n\n"),
                            })
                        }
                        StrategyCommand::Query => {
                            // 查询群组当前配置
                            let mut messages = vec!["📊 群组当前配置:".to_string()];
                            
                            // 显示策略类型
                            let strategy_name = match group_data.stratege {
                                StrategeType::CmdStrategy => "命令模式",
                                StrategeType::LlmStrategy => "大模型聊天模式",
                            };
                            messages.push(format!("🔧 回复策略: {}", strategy_name));
                            
                            // 如果是 LLM 模式，显示模型和提示词信息
                            if matches!(group_data.stratege, StrategeType::LlmStrategy) {
                                messages.push(format!("🤖 使用模型: {}", group_data.model));
                                
                                match &group_data.custom_prompt {
                                    Some(custom) => {
                                        messages.push(format!("📝 提示词: 自定义\n{}", custom));
                                    }
                                    None => {
                                        messages.push("📝 提示词: 系统默认".to_string());
                                    }
                                }
                            }
                            
                            Ok(CmdResult {
                                output: messages.join("\n\n"),
                            })
                        }
                    }
                } else {
                    // 私聊环境：操作用户配置
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
                        StrategyCommand::Query => {
                            // 查询用户当前配置
                            let mut messages = vec!["📊 您当前的配置:".to_string()];
                            
                            // 显示策略类型
                            let strategy_name = match user_data.stratege {
                                StrategeType::CmdStrategy => "命令模式",
                                StrategeType::LlmStrategy => "大模型聊天模式",
                            };
                            messages.push(format!("🔧 回复策略: {}", strategy_name));
                            
                            // 如果是 LLM 模式，显示模型和提示词信息
                            if matches!(user_data.stratege, StrategeType::LlmStrategy) {
                                messages.push(format!("🤖 使用模型: {}", user_data.model));
                                
                                match &user_data.custom_prompt {
                                    Some(custom) => {
                                        messages.push(format!("📝 提示词: 自定义\n{}", custom));
                                    }
                                    None => {
                                        messages.push("📝 提示词: 系统默认".to_string());
                                    }
                                }
                            }
                            
                            Ok(CmdResult {
                                output: messages.join("\n\n"),
                            })
                        }
                    }
                }
            })
        })
    }
}

use crate::{
    BOT_CACHE, UserData,
    cmd::{CmdHandler, CmdResult, CommonArgs, HandlerBuilder},
    config::APPCONFIG,
    error::AppError,
};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "prompt")]
#[command(about = "设置或查看当前用户的自定义提示词")]
pub struct Prompt {
    #[command(flatten)]
    pub common: CommonArgs,

    #[arg(help = "设置提示词内容，留空则查看当前提示词")]
    content: Option<String>,
    
    #[arg(long, help = "重置为默认提示词", default_value_t=false)]
    reset: bool,
}

impl HandlerBuilder for Prompt {
    fn build() -> CmdHandler {
        Box::new(move |args: Vec<String>| {
            Box::pin(async move {
                let prompt = Prompt::try_parse_from(args)
                    .map_err(|err| AppError::command(err.to_string()))?;

                // 检查权限：群聊中只有管理员能使用，私聊中用户自己可以使用
                if !prompt.common.group_admin && prompt.common.env == String::from("group") {
                    return Err(AppError::command("群聊中只有管理员能使用此命令".to_string()));
                }

                // 获取当前用户数据
                let mut user_data = BOT_CACHE.get(&prompt.common.sender).await.unwrap_or_default();

                if prompt.reset {
                    // 重置提示词
                    user_data.custom_prompt = None;
                    BOT_CACHE.insert(prompt.common.sender, user_data).await;
                    
                    Ok(CmdResult {
                        output: "✅ 已重置为默认系统提示词".to_string(),
                    })
                } else if let Some(content) = prompt.content {
                    // 设置新的提示词
                    if content.trim().is_empty() {
                        return Err(AppError::command("提示词内容不能为空".to_string()));
                    }
                    
                    if content.len() > 2000 {
                        return Err(AppError::command("提示词长度不能超过2000个字符".to_string()));
                    }
                    
                    user_data.custom_prompt = Some(content.clone());
                    BOT_CACHE.insert(prompt.common.sender, user_data).await;
                    
                    Ok(CmdResult {
                        output: format!("✅ 提示词设置成功！\n\n📝 当前提示词:\n{}", content),
                    })
                } else {
                    // 查看当前提示词
                    match user_data.custom_prompt {
                        Some(custom_prompt) => {
                            Ok(CmdResult {
                                output: format!("📝 当前自定义提示词:\n{}\n\n💡 使用 /prompt --reset 可重置为默认提示词", custom_prompt),
                            })
                        }
                        None => {
                            Ok(CmdResult {
                                output: format!("📝 当前使用默认系统提示词:\n{}\n\n💡 使用 /prompt <内容> 可设置自定义提示词", APPCONFIG.llm.system_prompt),
                            })
                        }
                    }
                }
            })
        })
    }
}

use clap::Parser;
use crate::{
    cmd::{CmdResult, CommonArgs, HandlerBuilder},
    error::AppError,
};
use std::{future::Future, pin::Pin};

#[derive(Parser, Debug, Clone)]
#[command(name = "push")]
#[command(about = "推送消息到群成员（私聊中使用，需要群管理员权限）")]
pub struct Push {
    #[command(flatten)]
    pub common: CommonArgs,

    #[arg(short = 'g', long, help = "目标群号")]
    pub group_id: i64,

    #[arg(short = 'm', long, help = "消息内容")]
    pub message: String,

    #[arg(short = 'l', long, help = "目标成员QQ号列表", num_args = 1..)]
    pub members: Vec<i64>,
}

impl HandlerBuilder for Push {
    fn build() -> crate::cmd::CmdHandler {
        Box::new(|args: Vec<String>| {
            Box::pin(async move {
                let push = Push::try_parse_from(std::iter::once("push".to_string()).chain(args))
                    .map_err(|e| AppError::command(e.to_string()))?;

                // 只能在私聊中使用
                if push.common.env() != "private" {
                    return Err(AppError::command("❌ 此命令只能在私聊中使用".to_string()));
                }

                // 验证参数
                if push.group_id <= 0 {
                    return Err(AppError::command("❌ 请指定有效的群号".to_string()));
                }

                if push.message.trim().is_empty() {
                    return Err(AppError::command("❌ 消息内容不能为空".to_string()));
                }

                if push.members.is_empty() {
                    return Err(AppError::command("❌ 请指定至少一个目标成员QQ号".to_string()));
                }

                // 返回说明信息，实际的消息发送由push插件处理
                Ok(CmdResult {
                    output: format!(
                        "📝 Push命令已记录，但实际的消息发送需要通过push插件处理。\n\n参数信息：\n• 群号：{}\n• 消息：\"{}\"\n• 目标成员：{:?}\n\n💡 请使用插件格式: /push -g {} -m \"{}\" -l {}",
                        push.group_id,
                        push.message,
                        push.members,
                        push.group_id,
                        push.message,
                        push.members.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" ")
                    ),
                })
            }) as Pin<Box<dyn Future<Output = Result<CmdResult, AppError>> + Send>>
        }) as crate::cmd::CmdHandler
    }
}

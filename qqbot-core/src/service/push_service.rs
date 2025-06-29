use crate::{
    config::get_db,
    models::student::{self, Entity as Student},
    error::{AppError, AppResult},
};
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PushRequest {
    pub sender_id: i64,        // 发送者QQ号
    pub group_id: i64,         // 目标群号
    pub message: String,       // 消息内容
    pub target_members: Vec<i64>, // 目标成员QQ号列表
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PushResult {
    pub success_count: usize,
    pub failed_count: usize,
    pub total_count: usize,
    pub failed_members: Vec<String>,
    pub message: String,
}

pub struct PushService;

impl PushService {
    /// 验证用户是否有权限向指定群发送消息
    pub async fn validate_permission(
        sender_id: i64,
        group_id: i64,
    ) -> AppResult<bool> {
        // 这里需要调用QQ API检查：
        // 1. 机器人是否在指定群内
        // 2. 发送者是否是该群的管理员
        // 由于无法直接调用QQ API，这里返回模拟结果
        
        // 模拟权限检查逻辑
        if group_id > 0 && sender_id > 0 {
            // 实际应用中需要调用QQ群信息API
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// 执行推送消息
    pub async fn push_messages(request: PushRequest) -> AppResult<PushResult> {
        // 1. 验证权限
        if !Self::validate_permission(request.sender_id, request.group_id).await? {
            return Err(AppError::command("❌ 您没有权限向此群发送消息".to_string()));
        }

        // 2. 验证消息内容
        if request.message.trim().is_empty() {
            return Err(AppError::command("❌ 消息内容不能为空".to_string()));
        }

        if request.target_members.is_empty() {
            return Err(AppError::command("❌ 目标成员列表不能为空".to_string()));
        }

        // 3. 执行消息发送（模拟）
        let mut success_count = 0;
        let mut failed_members = Vec::new();

        for &member_id in &request.target_members {
            // 实际应用中需要调用QQ API发送群临时私聊消息
            // 这里模拟发送结果
            if Self::send_temp_message(request.group_id, member_id, &request.message).await.is_ok() {
                success_count += 1;
            } else {
                failed_members.push(format!("QQ{}: 发送失败", member_id));
            }
        }

        let result = PushResult {
            success_count,
            failed_count: failed_members.len(),
            total_count: request.target_members.len(),
            failed_members: failed_members.clone(),
            message: format!(
                "推送完成：成功{}条，失败{}条",
                success_count,
                failed_members.len()
            ),
        };

        Ok(result)
    }

    /// 发送群临时私聊消息（模拟）
    async fn send_temp_message(
        group_id: i64,
        member_id: i64,
        message: &str,
    ) -> AppResult<()> {
        // 实际应用中需要调用类似以下的API：
        // bot.send_group_temp_msg(group_id, member_id, message).await?;
        
        // 模拟发送（90%成功率）
        use rand::Rng;
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.9) {
            Ok(())
        } else {
            Err(AppError::command("消息发送失败".to_string()))
        }
    }

    /// 根据学号查询QQ号
    pub async fn get_qq_by_student_ids(student_ids: Vec<String>) -> AppResult<Vec<i64>> {
        let db = get_db().await;
        
        let students = Student::find()
            .filter(student::Column::StudentId.is_in(student_ids))
            .all(db.as_ref())
            .await
            .map_err(|e| AppError::database(format!("查询学生信息失败: {}", e)))?;

        let qq_numbers: Vec<i64> = students
            .into_iter()
            .filter_map(|s| s.qq_number?.parse().ok())
            .collect();

        Ok(qq_numbers)
    }
}

// 如果需要在现有cmd系统中使用，可以创建一个简化的命令
use crate::cmd::{CmdHandler, CmdResult, HandlerBuilder};
use clap::Parser;
use std::{future::Future, pin::Pin};

#[derive(Parser, Debug, Clone)]
#[command(name = "push")]
#[command(about = "推送消息到群成员（私聊中使用，需要群管理员权限）")]
pub struct PushCommand {
    #[command(flatten)]
    pub common: crate::cmd::CommonArgs,

    #[arg(short = 'g', long, help = "目标群号")]
    pub group_id: i64,

    #[arg(short = 'm', long, help = "消息内容")]
    pub message: String,

    #[arg(short = 'l', long, help = "目标成员QQ号列表", num_args = 1..)]
    pub members: Vec<i64>,
}

impl HandlerBuilder for PushCommand {
    fn build() -> CmdHandler {
        Box::new(|args: Vec<String>| {
            Box::pin(async move {
                let push = PushCommand::try_parse_from(std::iter::once("push".to_string()).chain(args))
                    .map_err(|e| AppError::command(e.to_string()))?;

                // 只能在私聊中使用
                if push.common.env != "private" {
                    return Err(AppError::command("❌ 此命令只能在私聊中使用".to_string()));
                }

                let request = PushRequest {
                    sender_id: push.common.sender,
                    group_id: push.group_id,
                    message: push.message,
                    target_members: push.members,
                };

                match PushService::push_messages(request).await {
                    Ok(result) => Ok(CmdResult {
                        output: format!(
                            "📤 推送结果：\n{}\n\n📊 详细统计：\n• 成功：{}条\n• 失败：{}条\n• 总计：{}条{}",
                            result.message,
                            result.success_count,
                            result.failed_count,
                            result.total_count,
                            if !result.failed_members.is_empty() {
                                format!("\n\n❌ 失败详情：\n{}", result.failed_members.join("\n"))
                            } else {
                                String::new()
                            }
                        ),
                    }),
                    Err(e) => Err(e),
                }
            }) as Pin<Box<dyn Future<Output = Result<CmdResult, AppError>> + Send>>
        }) as CmdHandler
    }
}

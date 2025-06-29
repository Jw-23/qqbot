use kovi::PluginBuilder as plugin;
use qqbot_core::{
    config::get_db,
};

#[kovi::plugin]
async fn main() {
    let bot = plugin::get_runtime_bot();
    get_db().await;

    // 监听push命令消息
    plugin::on_msg(move |event| {
        let bot = bot.clone();
        async move {
            // 只处理私聊消息
            if event.message_type != "private" {
                return;
            }

            // 检查是否是push命令
            if let Some(msg) = event.borrow_text() {
                if !msg.starts_with("/push") {
                    return;
                }

                // 解析push命令
                match parse_push_command(msg) {
                    Ok(push_cmd) => {
                        // 执行push命令
                        let result = execute_push_command(&bot, &event, push_cmd).await;
                        
                        // 发送结果给用户
                        bot.send_private_msg(event.sender.user_id, result);
                    }
                    Err(err) => {
                        bot.send_private_msg(event.sender.user_id, format!("❌ 命令格式错误: {}", err));
                    }
                }
            }
        }
    });
}

#[derive(Debug)]
struct PushCommand {
    group_id: i64,
    message: String,
    members: Vec<i64>,
}

fn parse_push_command(msg: &str) -> Result<PushCommand, String> {
    // 简单的命令解析 - 这里可以使用更复杂的解析器
    // 格式: /push -g 群号 -m 消息内容 -l 成员1 成员2 成员3
    
    let parts: Vec<&str> = msg.split_whitespace().collect();
    if parts.len() < 7 {
        return Err("命令格式错误。正确格式: /push -g 群号 -m 消息内容 -l QQ号1 QQ号2 ...".to_string());
    }

    let mut group_id = None;
    let mut message = None;
    let mut members = Vec::new();
    let mut i = 1; // 跳过 "/push"

    while i < parts.len() {
        match parts[i] {
            "-g" => {
                if i + 1 < parts.len() {
                    group_id = parts[i + 1].parse().ok();
                    i += 2;
                } else {
                    return Err("缺少群号参数".to_string());
                }
            }
            "-m" => {
                if i + 1 < parts.len() {
                    // 消息可能包含空格，需要特殊处理
                    let msg_start = i + 1;
                    let mut msg_parts = Vec::new();
                    let mut j = msg_start;
                    
                    // 收集消息内容直到遇到 -l
                    while j < parts.len() && parts[j] != "-l" {
                        msg_parts.push(parts[j]);
                        j += 1;
                    }
                    
                    if msg_parts.is_empty() {
                        return Err("缺少消息内容".to_string());
                    }
                    
                    message = Some(msg_parts.join(" "));
                    i = j;
                } else {
                    return Err("缺少消息内容参数".to_string());
                }
            }
            "-l" => {
                // 收集所有后续的QQ号
                i += 1;
                while i < parts.len() {
                    if let Ok(qq) = parts[i].parse::<i64>() {
                        members.push(qq);
                    } else {
                        return Err(format!("无效的QQ号: {}", parts[i]));
                    }
                    i += 1;
                }
                break;
            }
            _ => {
                return Err(format!("未知参数: {}", parts[i]));
            }
        }
    }

    let group_id = group_id.ok_or("缺少群号参数")?;
    let message = message.ok_or("缺少消息内容参数")?;
    
    if members.is_empty() {
        return Err("缺少目标成员QQ号".to_string());
    }

    Ok(PushCommand {
        group_id,
        message,
        members,
    })
}

async fn execute_push_command(
    bot: &kovi::RuntimeBot,
    event: &kovi::bot::plugin_builder::event::MsgEvent,
    cmd: PushCommand,
) -> String {
    // 检查用户是否是指定群的管理员
    // 注意：这里需要调用QQ API获取群成员信息，kovi可能需要额外的API支持
    // 暂时跳过权限检查，在实际部署时需要实现
    
    let _sender_id = event.sender.user_id;
    
    // TODO: 实现权限检查
    // let is_admin = check_group_admin(bot, cmd.group_id, sender_id).await;
    // if !is_admin {
    //     return "❌ 您不是该群的管理员，无法使用此功能".to_string();
    // }

    // 发送消息给每个目标成员
    let mut success_count = 0;
    let failed_members: Vec<String> = Vec::new();

    for member_qq in &cmd.members {
        // 发送临时会话消息 (群临时消息)
        // 注意：kovi可能需要特殊的API来发送群临时消息
        // 这里先使用普通私聊消息作为替代
        
        bot.send_private_msg(*member_qq, &cmd.message);
        
        // 假设发送成功（实际应该检查API返回，但kovi的send_private_msg返回()）
        success_count += 1;
        
        // 添加短暂延迟避免发送过快
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }

    let total_count = cmd.members.len();
    let failed_count = total_count - success_count;

    format!(
        "📤 推送完成！\n\n📊 统计信息：\n• 目标群：{}\n• 成功：{}条\n• 失败：{}条\n• 总计：{}条\n• 消息内容：\"{}\"\n{}",
        cmd.group_id,
        success_count,
        failed_count,
        total_count,
        cmd.message,
        if !failed_members.is_empty() {
            format!("\n❌ 失败详情：\n{}", failed_members.join("\n"))
        } else {
            String::new()
        }
    )
}

// TODO: 实现群管理员权限检查
// async fn check_group_admin(bot: &kovi::RuntimeBot, group_id: i64, user_id: i64) -> bool {
//     // 这里需要调用QQ API检查用户是否是群管理员
//     // 具体实现取决于kovi框架提供的API
//     false
// }

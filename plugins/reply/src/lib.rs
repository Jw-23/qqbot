use kovi::PluginBuilder as plugin;
use qqbot_core::{
    BOT_CACHE, StrategeType, UserData,
    config::{APPCONFIG, get_db},
    reply_strategy::{Env, MessageContent, MessageContext, reply_manager::ReplyManager},
};

#[kovi::plugin]
async fn main() {
    let bot = plugin::get_runtime_bot();
    get_db().await;

    // 创建回复管理器
    let reply_manager = ReplyManager::new();

    plugin::on_msg(move |event| {
        let bot = bot.clone();
        let reply_manager = reply_manager.clone();
        async move {
            let sender = event.sender.user_id;

            // 获取或创建用户数据
            let data = if let Some(user_data) = BOT_CACHE.get(&sender).await {
                user_data
            } else {
                BOT_CACHE.insert(sender, UserData::default()).await;
                UserData::default()
            };

            // 处理消息
            if let Some(msg) = event.borrow_text() {
                // 检查是否应该响应这条消息
                let should_respond = match data.stratege {
                    StrategeType::CmdStrategy => {
                        // 命令模式只响应命令消息
                        msg.starts_with(&APPCONFIG.cmd_suffix)
                    }
                    StrategeType::LlmStrategy => {
                        // LLM模式响应所有文本消息
                        true
                    }
                };

                if should_respond {
                    let env = if event.message_type == "private" {
                        Env::Private
                    } else if event.message_type == "group" {
                        if let Some(group_id) = event.group_id {
                            Env::Group { group_id }
                        } else {
                            // 如果没有群号，按私聊处理
                            Env::Private
                        }
                    } else {
                        // 其他类型按私聊处理
                        Env::Private
                    };

                    let message_context = MessageContext {
                        env,
                        sender_id: event.sender.user_id,
                        self_id: event.self_id,
                        message: MessageContent::Text(msg.to_string()),
                        group_admin: event.sender.role == Some(String::from("admin"))
                            || event.sender.role == Some(String::from("owner")),
                        history: vec![], // 未来可以扩展为真实的对话历史
                        sender_name: event.sender.nickname.clone().or_else(|| Some(format!("用户{}", event.sender.user_id))),
                    };

                    // 使用统一的回复管理器处理消息
                    let reply_msg = match reply_manager.reply(&message_context).await {
                        Ok(MessageContent::Text(res)) => res,
                        Err(err) => {
                            // 根据错误类型提供友好的错误消息
                            if err.to_string().contains("API") {
                                "抱歉，AI服务暂时不可用，请稍后再试。".to_string()
                            } else if err.to_string().contains("Command") {
                                "命令执行失败，请检查命令格式。".to_string()
                            } else {
                                format!("处理失败: {}", err)
                            }
                        }
                        _ => "收到了不支持的回复类型".to_string(),
                    };
                    let reply_msg = String::from(reply_msg.trim());
                    // 发送回复
                    match event.message_type.as_str() {
                        "private" => {
                            bot.send_private_msg(event.sender.user_id, reply_msg);
                        }
                        "group" => {
                            if let Some(group_id) = event.group_id {
                                bot.send_group_msg(group_id, reply_msg);
                            }
                        }
                        _ => {
                            // 其他类型的消息，默认发送私聊
                            bot.send_private_msg(event.sender.user_id, reply_msg);
                        }
                    }
                }
            }
        }
    });
}

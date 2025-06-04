use kovi::PluginBuilder as plugin;
use qqbot_core::{
    BOT_CACHE, SessionId, StrategeType,
    config::{APPCONFIG, get_db},
    conversation::ConversationManager,
    reply_strategy::{Env, MessageContent, MessageContext, reply_manager::ReplyManager},
    service::group_config_service::GROUP_CACHE,
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

            // 根据消息环境获取有效配置（群组优先或用户配置）
            let effective_config = if event.message_type == "group" {
                // 群聊环境：优先使用群组配置，如果没有则使用用户配置
                if let Some(group_id) = event.group_id {
                    if let Some(group_data) = GROUP_CACHE.get(&group_id).await {
                        // 使用群组配置
                        (group_data.stratege, group_data.custom_prompt)
                    } else {
                        // 群组没有配置，使用用户配置
                        let user_data = BOT_CACHE.get(&sender).await.unwrap_or_default();
                        (user_data.stratege, user_data.custom_prompt)
                    }
                } else {
                    // 没有群组ID，使用用户配置
                    let user_data = BOT_CACHE.get(&sender).await.unwrap_or_default();
                    (user_data.stratege, user_data.custom_prompt)
                }
            } else {
                // 私聊环境：使用用户配置
                let user_data = BOT_CACHE.get(&sender).await.unwrap_or_default();
                (user_data.stratege, user_data.custom_prompt)
            };

            let (strategy, _custom_prompt) = effective_config;

            // 处理消息
            if let Some(msg) = event.borrow_text() {
                // 检查是否被@了（仅在群聊中有效）
                let is_mentioned = if event.message_type == "group" {
                    event.message.iter().any(|m| {
                        m.type_ == "at"
                            && m.data
                                .get("qq")
                                .and_then(|v| v.as_str())
                                .map(|qq| qq == event.self_id.to_string())
                                .unwrap_or(false)
                    })
                } else {
                    false // 私聊不需要@
                };

                // 新的智能响应逻辑
                let should_respond = if event.message_type == "private" {
                    // 私聊：根据策略决定
                    match strategy {
                        StrategeType::CmdStrategy => msg.starts_with(&APPCONFIG.cmd_suffix),
                        StrategeType::LlmStrategy => true,
                    }
                } else {
                    // 群聊：必须被@才考虑响应
                    if is_mentioned {
                        // 被@了，根据消息内容决定
                        if msg.starts_with(&APPCONFIG.cmd_suffix) {
                            // 以命令前缀开头：按策略处理
                            match strategy {
                                StrategeType::CmdStrategy => true,
                                StrategeType::LlmStrategy => true, // LLM模式下命令也处理
                            }
                        } else {
                            // 不以命令前缀开头：强制使用LLM
                            true
                        }
                    } else {
                        false // 没被@，不响应
                    }
                };

                // 消息捕获逻辑：在LLM策略下自动捕获消息到对话历史
                let should_capture = match strategy {
                    StrategeType::LlmStrategy => {
                        if event.message_type == "group"
                            && APPCONFIG.llm.auto_capture_group_messages
                        {
                            true
                        } else if event.message_type == "private" {
                            true
                        } else {
                            false
                        }
                    }
                    _ => false,
                };

                // 捕获消息到对话历史（只在不回复时捕获，避免重复）
                if should_capture && !should_respond {
                    let env = if event.message_type == "private" {
                        Env::Private
                    } else if event.message_type == "group" {
                        if let Some(group_id) = event.group_id {
                            Env::Group { group_id }
                        } else {
                            Env::Private
                        }
                    } else {
                        Env::Private
                    };

                    let session_id = match env {
                        Env::Private => SessionId::Private(event.sender.user_id),
                        Env::Group { group_id } => SessionId::Group(group_id),
                    };

                    let username = match env {
                        Env::Group { .. } => event
                            .sender
                            .card
                            .clone()
                            .or(event.sender.nickname.clone())
                            .unwrap_or_else(|| format!("用户{}", event.sender.user_id)),
                        Env::Private => event.sender.nickname.clone()
                            .unwrap_or_else(|| format!("用户{}", event.sender.user_id)),
                    };

                    ConversationManager::add_user_message_with_info(
                        session_id,
                        msg.to_string(),
                        event.sender.user_id,
                        match env {
                            Env::Group { .. } => Some(username),
                            Env::Private => None,
                        },
                    )
                    .await;
                }

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
                        sender_name: event
                            .sender
                            .card
                            .clone()
                            .or_else(|| event.sender.nickname.clone())
                            .or_else(|| Some(format!("用户{}", event.sender.user_id))),
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

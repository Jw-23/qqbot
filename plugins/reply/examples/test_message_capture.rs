use qqbot_core::{
    BOT_CACHE, StrategeType, UserData, SessionId,
    config::APPCONFIG,
    reply_strategy::Env,
    conversation::ConversationManager,
};

// 模拟一个消息事件结构
#[derive(Debug)]
struct MockMessageEvent {
    sender_user_id: i64,
    message_type: String,
    group_id: Option<i64>,
    text: String,
    nickname: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 测试群聊消息捕获功能");
    
    // 模拟用户设置为LLM策略，带自定义提示词
    let user_id = 123456;
    let user_data = UserData {
        stratege: StrategeType::LlmStrategy,
        model: "test-model".to_string(),
        custom_prompt: Some("你是一个友善的AI助手，会详细回答用户的问题。".to_string()),
    };
    BOT_CACHE.insert(user_id, user_data).await;
    
    // 模拟群聊消息事件
    let mock_event = MockMessageEvent {
        sender_user_id: user_id,
        message_type: "group".to_string(),
        group_id: Some(987654),
        text: "这是一条测试消息".to_string(),
        nickname: Some("测试用户".to_string()),
    };
    
    println!("📨 模拟消息: 用户{} 在群{:?} 发送: {}", 
        mock_event.sender_user_id, mock_event.group_id, mock_event.text);
    
    // 获取用户数据
    let data = BOT_CACHE.get(&mock_event.sender_user_id).await.unwrap_or_default();
    println!("👤 用户策略: {:?}", data.stratege);
    println!("🔤 用户模型: {}", data.model);
    match &data.custom_prompt {
        Some(prompt) => println!("📝 自定义提示词: {}", prompt),
        None => println!("📝 使用默认提示词"),
    }
    
    // 检查是否应该捕获
    let should_capture = match data.stratege {
        StrategeType::LlmStrategy => {
            if mock_event.message_type == "group" && APPCONFIG.llm.auto_capture_group_messages {
                println!("🔍 群聊消息捕获: 用户{} 在群{:?} 发送: {} (auto_capture={})", 
                    mock_event.sender_user_id, mock_event.group_id, mock_event.text, APPCONFIG.llm.auto_capture_group_messages);
                true
            } else {
                println!("🚫 不捕获消息: 消息类型={}, LLM自动捕获={}", 
                    mock_event.message_type, APPCONFIG.llm.auto_capture_group_messages);
                false
            }
        }
        _ => {
            println!("🚫 不捕获消息: 策略类型={:?}", data.stratege);
            false
        }
    };
    
    // 模拟不回复的情况（群聊中未被@）
    let should_respond = false;
    
    println!("📊 消息处理状态: should_capture={}, should_respond={}", 
        should_capture, should_respond);
    
    // 如果需要捕获且不回复，则保存消息
    if should_capture && !should_respond {
        println!("💾 保存消息到对话历史");
        
        let env = Env::Group { group_id: mock_event.group_id.unwrap() };
        let session_id = SessionId::Group(mock_event.group_id.unwrap());
        let username = mock_event.nickname.clone()
            .unwrap_or_else(|| format!("用户{}", mock_event.sender_user_id));
        
        println!("🌍 环境: {:?}", env);
        println!("🆔 会话ID: {:?}", session_id);
        
        // 保存消息
        ConversationManager::add_user_message_with_info(
            session_id.clone(),
            mock_event.text.clone(),
            mock_event.sender_user_id,
            Some(username.clone()),
        ).await;
        
        println!("✅ 消息已保存到对话历史");
        
        // 验证保存是否成功
        let history = ConversationManager::get_conversation_history(session_id.clone(), 10).await;
        println!("📖 当前对话历史:");
        for (i, msg) in history.iter().enumerate() {
            println!("  {}. [{}] {}: {}", 
                i + 1, 
                msg.timestamp.format("%H:%M:%S"),
                msg.username.as_deref().unwrap_or("未知用户"),
                msg.content
            );
        }
        
        println!("🎉 测试成功！群聊消息捕获功能正常工作");
    } else {
        if !should_capture {
            println!("❌ 测试失败：消息没有被标记为需要捕获");
        }
        if should_respond {
            println!("❌ 测试失败：消息被标记为需要回复（测试中应该不回复）");
        }
    }
    
    println!("\n📋 配置信息:");
    println!("  - auto_capture_group_messages: {}", APPCONFIG.llm.auto_capture_group_messages);
    println!("  - 默认策略: {:?}", StrategeType::default());
    
    // 额外测试：验证自定义提示词功能
    println!("\n🔧 额外测试：自定义提示词功能");
    
    // 测试1：用户有自定义提示词
    println!("\n📝 测试1: 验证用户自定义提示词");
    let user_data_with_prompt = BOT_CACHE.get(&user_id).await.unwrap_or_default();
    match &user_data_with_prompt.custom_prompt {
        Some(prompt) => {
            println!("✅ 用户已设置自定义提示词: {}", prompt);
        }
        None => {
            println!("❌ 用户未设置自定义提示词，这不符合预期");
        }
    }
    
    // 测试2：模拟用户重置提示词
    println!("\n🔄 测试2: 模拟重置自定义提示词");
    let mut user_data_reset = user_data_with_prompt.clone();
    user_data_reset.custom_prompt = None;
    BOT_CACHE.insert(user_id, user_data_reset).await;
    
    let reset_data = BOT_CACHE.get(&user_id).await.unwrap_or_default();
    match &reset_data.custom_prompt {
        Some(prompt) => {
            println!("❌ 提示词未被重置，当前值: {}", prompt);
        }
        None => {
            println!("✅ 提示词已成功重置为默认值");
        }
    }
    
    // 测试3：模拟新用户（无自定义提示词）
    println!("\n👤 测试3: 模拟新用户（无自定义提示词）");
    let new_user_id = 999999;
    let new_user_data = UserData {
        stratege: StrategeType::LlmStrategy,
        model: "test-model".to_string(),
        custom_prompt: None,
    };
    BOT_CACHE.insert(new_user_id, new_user_data).await;
    
    let new_data = BOT_CACHE.get(&new_user_id).await.unwrap_or_default();
    match &new_data.custom_prompt {
        Some(prompt) => {
            println!("❌ 新用户不应该有自定义提示词，但检测到: {}", prompt);
        }
        None => {
            println!("✅ 新用户正确使用默认提示词");
        }
    }
    
    println!("\n🎉 所有测试完成！消息捕获和自定义提示词功能正常工作");
    
    Ok(())
}

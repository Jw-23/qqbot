/// 测试连续对话功能的示例
/// 运行命令: cargo run --example test_conversation

use qqbot_core::{
    conversation::ConversationManager,
    SessionId, UserId, GroupId,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🤖 测试连续对话功能");
    
    // 创建一个私聊会话
    let user_id: UserId = 123456;
    let session_id = SessionId::Private(user_id);
    
    println!("\n📝 模拟连续对话:");
    
    // 第一轮对话
    println!("用户: 你好");
    ConversationManager::add_user_message_with_info(
        session_id.clone(),
        "你好".to_string(),
        user_id,
        Some("测试用户".to_string()),
    ).await;
    
    ConversationManager::add_assistant_message(
        session_id.clone(),
        "你好！有什么可以帮助你的吗？".to_string(),
    ).await;
    println!("助手: 你好！有什么可以帮助你的吗？");
    
    // 第二轮对话
    println!("用户: 我想了解天气");
    ConversationManager::add_user_message_with_info(
        session_id.clone(),
        "我想了解天气".to_string(),
        user_id,
        Some("测试用户".to_string()),
    ).await;
    
    ConversationManager::add_assistant_message(
        session_id.clone(),
        "请告诉我你想查询哪个城市的天气？".to_string(),
    ).await;
    println!("助手: 请告诉我你想查询哪个城市的天气？");
    
    // 第三轮对话
    println!("用户: 北京");
    ConversationManager::add_user_message_with_info(
        session_id.clone(),
        "北京".to_string(),
        user_id,
        Some("测试用户".to_string()),
    ).await;
    
    ConversationManager::add_assistant_message(
        session_id.clone(),
        "抱歉，我暂时无法获取实时天气信息，建议您查看天气应用。".to_string(),
    ).await;
    println!("助手: 抱歉，我暂时无法获取实时天气信息，建议您查看天气应用。");
    
    // 获取完整对话历史
    println!("\n📚 完整对话历史:");
    let history = ConversationManager::get_conversation_history(session_id.clone(), 10).await;
    
    for (i, msg) in history.iter().enumerate() {
        let role = if msg.role == "user" { "用户" } else { "助手" };
        let username = msg.username.as_deref().unwrap_or("unknown");
        println!("{}. [{}] {}: {}", 
            i + 1, 
            msg.timestamp.format("%H:%M:%S"),
            if msg.role == "user" { username } else { role },
            msg.content
        );
    }
    
    // 测试会话状态
    println!("\n📊 会话状态:");
    println!("活跃会话数量: {}", ConversationManager::get_active_session_count().await);
    
    if let Some(last_activity) = ConversationManager::get_last_activity(session_id.clone()).await {
        println!("最后活动时间: {}", last_activity.format("%Y-%m-%d %H:%M:%S"));
    }
    
    // 测试群聊功能
    println!("\n👥 测试群聊多用户对话:");
    let group_id: GroupId = 987654;
    let group_session_id = SessionId::Group(group_id);
    
    // 用户A发言
    let user_a = 111;
    ConversationManager::add_user_message_with_info(
        group_session_id.clone(),
        "大家好，我是新来的".to_string(),
        user_a,
        Some("张三".to_string()),
    ).await;
    println!("张三: 大家好，我是新来的");
    
    // 用户B发言
    let user_b = 222;
    ConversationManager::add_user_message_with_info(
        group_session_id.clone(),
        "欢迎欢迎！".to_string(),
        user_b,
        Some("李四".to_string()),
    ).await;
    println!("李四: 欢迎欢迎！");
    
    // 助手回复
    ConversationManager::add_assistant_message(
        group_session_id.clone(),
        "欢迎新朋友加入群聊！".to_string(),
    ).await;
    println!("助手: 欢迎新朋友加入群聊！");
    
    // 获取群聊历史
    println!("\n群聊对话历史:");
    let group_history = ConversationManager::get_conversation_history(group_session_id.clone(), 10).await;
    
    for (i, msg) in group_history.iter().enumerate() {
        let default_username = format!("用户{}", msg.user_id.unwrap_or(0));
        let speaker = if msg.role == "user" {
            msg.username.as_deref().unwrap_or(&default_username)
        } else {
            "助手"
        };
        println!("{}. {}: {}", i + 1, speaker, msg.content);
    }
    
    // 获取用户A的专属对话历史
    println!("\n张三的对话历史:");
    let user_a_history = ConversationManager::get_user_conversation_history(
        group_session_id, user_a, 10
    ).await;
    
    for (i, msg) in user_a_history.iter().enumerate() {
        let speaker = if msg.role == "user" {
            msg.username.as_deref().unwrap_or("张三")
        } else {
            "助手"
        };
        println!("{}. {}: {}", i + 1, speaker, msg.content);
    }
    
    println!("\n✅ 连续对话功能测试完成！");
    println!("总活跃会话数: {}", ConversationManager::get_active_session_count().await);
    
    Ok(())
}

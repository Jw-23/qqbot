use qqbot_core::{
    config::APPCONFIG,
    reply_strategy::{
        llm::SimpleLlmReplyStrategy,
        MessageContent, MessageContext, MessageSegment, ImageInfo, Env, RelyStrategy
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 测试图片视觉识别功能");
    
    // 确保APPCONFIG被初始化
    println!("📋 API Key: {}...", &APPCONFIG.llm.api_key[..10]);
    println!("📋 Base URL: {}", &APPCONFIG.llm.base_url);
    println!("📋 Model: {}", &APPCONFIG.llm.model);
    
    // 创建SimpleLlmReplyStrategy
    let strategy = SimpleLlmReplyStrategy::new();
    
    // 模拟一个包含图片的消息
    let image_info = ImageInfo {
        file: "test.jpg".to_string(),
        url: Some("https://example.com/test.jpg".to_string()), // 这是一个示例URL
        summary: Some("测试图片".to_string()),
        sub_type: Some(0),
        file_size: Some(1024),
        key: None,
        emoji_id: None,
        emoji_package_id: None,
    };
    
    let message_content = MessageContent::Mixed(vec![
        MessageSegment::Text { text: "这张图片里有什么？".to_string() },
        MessageSegment::Image { image_info },
    ]);
    
    let message_context = MessageContext {
        env: Env::Private,
        sender_id: 123456789,
        self_id: 987654321,
        message: message_content,
        group_admin: false,
        history: vec![],
        sender_name: Some("测试用户".to_string()),
    };
    
    // 调用回复策略
    println!("\n🚀 开始调用LLM API...");
    match strategy.reply(&message_context).await {
        Ok(MessageContent::Text(response)) => {
            println!("✅ 成功收到回复: {}", response);
        },
        Ok(content) => {
            println!("✅ 收到其他类型回复: {:?}", content);
        },
        Err(err) => {
            println!("❌ 调用失败: {}", err);
            // 打印详细的错误信息以便调试
            println!("错误类型: {:?}", err);
        }
    }
    
    Ok(())
}

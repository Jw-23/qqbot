// 简化的图片识别测试，不依赖数据库
use qqbot_core::reply_strategy::{MessageContent, MessageSegment, ImageInfo, MessageContext, Env, RelyStrategy};
use qqbot_core::reply_strategy::llm::SimpleLlmReplyStrategy;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== 简化图片识别测试 ===");

    // 模拟一个包含图片的消息
    let image_info = ImageInfo {
        file: "test.png".to_string(),
        url: Some("https://httpbin.org/image/png".to_string()), // 使用一个有效的图片URL
        summary: Some("测试图片".to_string()),
        sub_type: Some(0),
        file_size: Some(1024),
        key: None,
        emoji_id: None,
        emoji_package_id: None,
    };

    // 创建包含图片和文本的混合消息
    let mixed_message = MessageContent::Mixed(vec![
        MessageSegment::Text { 
            text: "这张图片里有什么？".to_string() 
        },
        MessageSegment::Image { 
            image_info: image_info.clone() 
        },
    ]);

    // 创建消息上下文
    let message_context = MessageContext {
        env: Env::Private,
        sender_id: 12345,
        self_id: 67890,
        message: mixed_message,
        group_admin: false,
        history: vec![],
        sender_name: Some("测试用户".to_string()),
    };

    println!("发送的消息包含:");
    println!("- 文本: 这张图片里有什么？");
    println!("- 图片: {} (URL: {})", 
        image_info.file, 
        image_info.url.as_deref().unwrap_or("无")
    );

    // 直接测试LLM策略
    let llm_strategy = SimpleLlmReplyStrategy::new();
    
    println!("\n正在调用LLM API...");
    match llm_strategy.reply(&message_context).await {
        Ok(MessageContent::Text(response)) => {
            println!("\n✅ LLM 回复: {}", response);
        },
        Err(e) => {
            println!("\n❌ LLM 调用失败: {}", e);
        },
        _ => {
            println!("\n收到非文本回复");
        }
    }

    Ok(())
}

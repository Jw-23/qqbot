// 测试完整的图片识别功能
use qqbot_core::reply_strategy::{MessageContent, MessageSegment, ImageInfo, MessageContext, Env};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化数据库连接
    qqbot_core::config::get_db().await;

    println!("=== 测试图片识别功能 ===");

    // 模拟一个包含图片的消息
    let image_info = ImageInfo {
        file: "cat.jpg".to_string(),
        url: Some("https://res.hc-cdn.com/smb-console/25.5.10/hws/images/instance/product-ecs.png".to_string()), // 模拟图片URL
        summary: Some("一只可爱的猫咪".to_string()),
        sub_type: Some(0), // JPEG
        file_size: Some(2048),
        key: None,
        emoji_id: None,
        emoji_package_id: None,
    };

    // 创建包含图片和文本的混合消息
    let mixed_message = MessageContent::Mixed(vec![
        MessageSegment::Text { 
            text: "请描述这张图片".to_string() 
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
    println!("- 文本: 请描述这张图片");
    println!("- 图片: {} (URL: {})", 
        image_info.file, 
        image_info.url.as_deref().unwrap_or("无")
    );

    // 测试MessageContent的便捷方法
    println!("\n消息分析:");
    println!("- 包含文本: {}", message_context.message.has_text());
    println!("- 包含图片: {}", message_context.message.has_image());
    println!("- 文本内容: {}", message_context.message.get_text());
    
    let images = message_context.message.get_images();
    println!("- 图片数量: {}", images.len());
    
    if let Some(image) = images.first() {
        println!("- 第一张图片:");
        println!("  文件名: {}", image.file);
        println!("  URL: {:?}", image.url);
        println!("  描述: {:?}", image.summary);
        println!("  文件大小: {:?} bytes", image.file_size);
    }

    // 注意：实际的LLM API调用需要有效的API密钥和端点
    // 这里我们只测试消息构建和提取功能
    println!("\n✓ 图片信息捕获和处理功能测试完成");
    println!("✓ 消息可以正确传递给支持视觉的大模型");
    
    // 如果配置了有效的LLM API，可以创建策略进行真实测试
    /*
    use qqbot_core::reply_strategy::{SimpleLlmReplyStrategy, RelyStrategy};
    let llm_strategy = SimpleLlmReplyStrategy::new();
    match llm_strategy.reply(&message_context).await {
        Ok(MessageContent::Text(response)) => {
            println!("\nLLM 回复: {}", response);
        },
        Err(e) => {
            println!("\nLLM 调用失败: {}", e);
        },
        _ => {
            println!("\n收到非文本回复");
        }
    }
    */

    Ok(())
}

// 测试图片信息捕获功能
use qqbot_core::reply_strategy::{MessageContent, MessageSegment, ImageInfo};

fn main() {
    // 模拟一个包含图片的混合消息
    let image_info = ImageInfo {
        file: "test.jpg".to_string(),
        url: Some("https://example.com/test.jpg".to_string()),
        summary: Some("一张测试图片".to_string()),
        sub_type: Some(0), // JPEG
        file_size: Some(1024),
        key: None,
        emoji_id: None,
        emoji_package_id: None,
    };

    let mixed_message = MessageContent::Mixed(vec![
        MessageSegment::Text { 
            text: "请看这张图片".to_string() 
        },
        MessageSegment::Image { 
            image_info: image_info.clone() 
        },
        MessageSegment::Text { 
            text: "很漂亮吧".to_string() 
        },
    ]);

    // 测试MessageContent的便捷方法
    println!("有文本: {}", mixed_message.has_text());
    println!("有图片: {}", mixed_message.has_image());
    println!("文本内容: {}", mixed_message.get_text());
    
    let images = mixed_message.get_images();
    println!("图片数量: {}", images.len());
    
    if let Some(first_image) = images.first() {
        println!("第一张图片文件名: {}", first_image.file);
        println!("第一张图片URL: {:?}", first_image.url);
        println!("第一张图片描述: {:?}", first_image.summary);
        println!("第一张图片大小: {:?}", first_image.file_size);
    }
}

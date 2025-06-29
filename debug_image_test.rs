use qqbot_core::reply_strategy::{MessageContent, MessageSegment, ImageInfo};

fn main() {
    // 模拟QQ图片消息的常见格式
    let test_cases = vec![
        // 典型的QQ图片文件名格式
        ImageInfo {
            file: "1234567890ABCDEF1234567890ABCDEF.jpg".to_string(),
            url: Some("https://gchat.qpic.cn/gchatpic_new/123456789/123456789-123456789-1234567890ABCDEF/0".to_string()),
            summary: None,
            sub_type: Some(0),
            file_size: Some(123456),
            key: None,
            emoji_id: None,
            emoji_package_id: None,
        },
        // 本地文件格式
        ImageInfo {
            file: "{ABCDEF12-3456-7890-ABCD-EF1234567890}.jpg".to_string(),
            url: None,
            summary: None,
            sub_type: Some(0),
            file_size: Some(78910),
            key: None,
            emoji_id: None,
            emoji_package_id: None,
        },
        // 可能的base64格式
        ImageInfo {
            file: "image.jpg".to_string(),
            url: Some("file:///path/to/image.jpg".to_string()),
            summary: None,
            sub_type: Some(0),
            file_size: Some(45678),
            key: None,
            emoji_id: None,
            emoji_package_id: None,
        },
    ];

    for (i, image_info) in test_cases.iter().enumerate() {
        println!("Test case {}: file={}, url={:?}", 
                 i + 1, 
                 image_info.file, 
                 image_info.url);
        
        if let Some(url) = &image_info.url {
            if url.starts_with("http://") || url.starts_with("https://") {
                println!("  → Valid HTTP URL");
            } else {
                println!("  → Non-HTTP URL, needs conversion to base64");
            }
        } else {
            println!("  → No URL provided");
        }
        println!();
    }
}

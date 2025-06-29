use qqbot_core::{
    cmd::{CMD_REGISTRY, Execute},
    config::get_db,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 简单测试push命令");
    
    // 初始化数据库连接
    get_db().await;
    
    // 测试push命令解析 - 使用字符串切片而不是临时值
    let args = vec![
        "--sender", "123456789",
        "--myself", "987654321", 
        "--env", "private",
        "-g", "111222333",
        "-m", "这是一条测试消息",
        "-l", "111111111", "222222222", "333333333"
    ];
    
    println!("📝 执行push命令");
    
    match CMD_REGISTRY.execute("push", &args).await {
        Ok(result) => {
            println!("✅ 命令执行成功:");
            println!("{}", result.output);
        }
        Err(e) => {
            println!("❌ 命令执行失败: {}", e);
        }
    }
    
    println!("\n✅ push命令测试完成！");
    Ok(())
}

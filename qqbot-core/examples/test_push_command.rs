use qqbot_core::{
    cmd::CMD_REGISTRY,
    config::get_db,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化数据库连接
    get_db().await;

    println!("=== 测试 Push 命令 ===");

    // 测试参数：模拟用户123456在私聊中发送push命令
    let user_id = 123456;
    let group_id = 987654;
    let members = vec![111111, 222222, 333333];
    let message = "这是一条测试推送消息";

    // 构建命令参数
    let args = vec![
        "--sender", &user_id.to_string(),
        "--myself", "987654",
        "--env", "private",
        "-g", &group_id.to_string(),
        "-m", message,
        "-l", "111111", "222222", "333333"
    ];

    println!("📤 执行推送命令...");
    println!("• 发送者：{}", user_id);
    println!("• 目标群：{}", group_id);
    println!("• 目标成员：{:?}", members);
    println!("• 消息内容：{}", message);

    // 执行命令
    match CMD_REGISTRY.execute("push", &args).await {
        Ok(result) => {
            println!("\n✅ 命令执行成功：");
            println!("{}", result.output);
        }
        Err(e) => {
            println!("\n❌ 命令执行失败：{}", e);
        }
    }

    // 测试错误情况：在群聊中使用
    println!("\n=== 测试错误情况：在群聊中使用 ===");
    let group_args = vec![
        "--sender", &user_id.to_string(),
        "--myself", "987654",
        "--group-id", &group_id.to_string(),
        "--env", "group",
        "-g", &group_id.to_string(),
        "-m", "测试消息",
        "-l", "111111"
    ];

    match CMD_REGISTRY.execute("push", &group_args).await {
        Ok(result) => {
            println!("意外成功：{}", result.output);
        }
        Err(e) => {
            println!("✅ 正确拒绝：{}", e);
        }
    }

    // 测试帮助信息
    println!("\n=== 测试帮助信息 ===");
    let help_args = vec!["--help"];
    match CMD_REGISTRY.execute("push", &help_args).await {
        Ok(result) => {
            println!("帮助信息：\n{}", result.output);
        }
        Err(e) => {
            println!("获取帮助失败：{}", e);
        }
    }

    Ok(())
}

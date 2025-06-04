use qqbot_core::{
    config::get_db,
    cmd::{Execute, CMD_REGISTRY},
    service::{group_config_service::GroupConfigService, user_config_service::UserConfigService},
    StrategeType,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 测试群组ID参数传递修复");
    println!("{}", "=".repeat(40));
    
    let db = get_db().await;
    let group_service = GroupConfigService::new(db.as_ref().clone());
    
    let test_group_id: i64 = 789012;
    let admin_user_id: i64 = 999999;
    
    // 清理之前的测试数据
    let _ = group_service.delete_group_config(test_group_id).await;
    
    let admin_id_str = admin_user_id.to_string();
    let group_id_str = test_group_id.to_string();
    
    println!("1️⃣ 测试群聊策略命令 - 设置自定义提示词");
    println!("{}", "-".repeat(30));
    
    // 管理员在群聊中设置策略，应该包含群组ID参数
    let admin_set_args = vec![
        "--sender", admin_id_str.as_str(),
        "--myself", "987654",
        "--group-id", group_id_str.as_str(),
        "--env", "group",
        "--group-admin",
        "llm",
        "--prompt", "你是一个测试AI助手，这是群组自定义提示词。"
    ];
    
    let result = CMD_REGISTRY.execute("strategy", &admin_set_args).await?;
    println!("✅ 策略设置结果:\n{}", result.output);
    
    // 验证群组配置已更新
    let group_data = group_service.get_group_data(test_group_id).await?;
    assert_eq!(group_data.stratege, StrategeType::LlmStrategy);
    assert!(group_data.custom_prompt.is_some());
    assert_eq!(group_data.custom_prompt.as_ref().unwrap(), "你是一个测试AI助手，这是群组自定义提示词。");
    println!("✅ 群组配置验证通过");
    
    println!("\n2️⃣ 测试查询群组配置");
    println!("{}", "-".repeat(30));
    
    let query_args = vec![
        "--sender", admin_id_str.as_str(),
        "--myself", "987654",
        "--group-id", group_id_str.as_str(),
        "--env", "group",
        "--group-admin",
        "query"
    ];
    
    let result = CMD_REGISTRY.execute("strategy", &query_args).await?;
    println!("✅ 查询结果:\n{}", result.output);
    
    // 清理测试数据
    let _ = group_service.delete_group_config(test_group_id).await;
    
    println!("\n🎉 群组ID参数传递修复验证成功！");
    
    Ok(())
}

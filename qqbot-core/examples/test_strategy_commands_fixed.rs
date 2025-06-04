use qqbot_core::{
    config::get_db,
    cmd::{Execute, CMD_REGISTRY},
    service::{group_config_service::GroupConfigService, user_config_service::UserConfigService},
    StrategeType,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("测试策略命令在群聊和私聊中的行为...");
    
    let db = get_db().await;
    let group_service = GroupConfigService::new(db.as_ref().clone());
    let user_service = UserConfigService::new(db.as_ref().clone());
    
    let test_user_id: i64 = 123456;
    let test_group_id: i64 = 789012;
    
    // 清理之前的测试数据
    let _ = group_service.delete_group_config(test_group_id).await;
    
    println!("1. 测试私聊中的策略命令...");
    
    // 创建字符串以避免生命周期问题
    let user_id_str = test_user_id.to_string();
    let group_id_str = test_group_id.to_string();
    
    // 模拟私聊环境下设置策略为命令模式
    let private_args = vec![
        "--sender", user_id_str.as_str(),
        "--myself", "987654",
        "--env", "private",
        "cmd"  // 使用命令模式
    ];
    
    // 执行私聊策略命令
    let result = CMD_REGISTRY.execute("strategy", &private_args).await?;
    println!("私聊策略设置结果: {}", result.output);
    
    // 验证用户配置是否更新
    let user_data = user_service.get_user_data(test_user_id).await?;
    assert_eq!(user_data.stratege, StrategeType::CmdStrategy);
    println!("✅ 私聊策略命令正常工作，用户配置已更新");
    
    println!("2. 测试群聊中的策略命令...");
    
    // 模拟群聊环境下设置策略为LLM模式（需要管理员权限）
    let group_args = vec![
        "--sender", user_id_str.as_str(),
        "--myself", "987654",
        "--group-id", group_id_str.as_str(),
        "--env", "group",
        "--group-admin",  // 设置为管理员（布尔开关）
        "llm"  // 使用LLM模式
    ];
    
    // 执行群聊策略命令
    let result = CMD_REGISTRY.execute("strategy", &group_args).await?;
    println!("群聊策略设置结果: {}", result.output);
    
    // 验证群组配置是否更新
    let group_data = group_service.get_group_data(test_group_id).await?;
    assert_eq!(group_data.stratege, StrategeType::LlmStrategy);
    println!("✅ 群聊策略命令正常工作，群组配置已更新");
    
    // 验证用户配置没有被影响
    let user_data_after = user_service.get_user_data(test_user_id).await?;
    assert_eq!(user_data_after.stratege, StrategeType::CmdStrategy); // 应该保持之前的设置
    println!("✅ 群聊策略命令不影响用户个人配置");
    
    println!("3. 测试查询命令...");
    
    // 查询私聊配置
    let private_query_args = vec![
        "--sender", user_id_str.as_str(),
        "--myself", "987654",
        "--env", "private",
        "query"  // 使用 query 子命令
    ];
    
    let query_result = CMD_REGISTRY.execute("strategy", &private_query_args).await;
    match query_result {
        Ok(result) => {
            println!("私聊配置查询结果: {}", result.output);
        }
        Err(e) => {
            println!("私聊配置查询失败: {}", e);
        }
    }
    
    // 查询群聊配置
    let group_query_args = vec![
        "--sender", user_id_str.as_str(),
        "--myself", "987654", 
        "--group-id", group_id_str.as_str(),
        "--env", "group",
        "--group-admin",  // 布尔开关
        "query"  // 使用 query 子命令
    ];
    
    let query_result = CMD_REGISTRY.execute("strategy", &group_query_args).await;
    match query_result {
        Ok(result) => {
            println!("群聊配置查询结果: {}", result.output);
        }
        Err(e) => {
            println!("群聊配置查询失败: {}", e);
        }
    }
    
    // 清理测试数据
    let _ = group_service.delete_group_config(test_group_id).await;
    
    println!("🎉 策略命令基本功能测试完成！");
    
    Ok(())
}

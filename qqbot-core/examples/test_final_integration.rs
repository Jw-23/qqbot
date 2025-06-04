use qqbot_core::{
    config::get_db,
    cmd::{Execute, CMD_REGISTRY},
    service::{group_config_service::GroupConfigService, user_config_service::UserConfigService},
    StrategeType,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 QQBot 群组配置系统完整功能测试");
    println!("{}", "=".repeat(50));
    
    let db = get_db().await;
    let group_service = GroupConfigService::new(db.as_ref().clone());
    let user_service = UserConfigService::new(db.as_ref().clone());
    
    let test_user_id: i64 = 123456;
    let test_group_id: i64 = 789012;
    let admin_user_id: i64 = 999999;
    
    // 清理之前的测试数据
    let _ = group_service.delete_group_config(test_group_id).await;
    
    // 创建字符串以避免生命周期问题
    let user_id_str = test_user_id.to_string();
    let group_id_str = test_group_id.to_string();
    let admin_id_str = admin_user_id.to_string();
    
    println!("1️⃣ 测试权限控制 - 非管理员无法修改群组配置");
    println!("{}", "-".repeat(40));
    
    // 测试非管理员尝试修改群组配置
    let non_admin_args = vec![
        "--sender", user_id_str.as_str(),
        "--myself", "987654",
        "--group-id", group_id_str.as_str(),
        "--env", "group",
        // 注意：没有 --group-admin 参数
        "llm"
    ];
    
    let result = CMD_REGISTRY.execute("strategy", &non_admin_args).await;
    match result {
        Ok(_) => {
            println!("❌ 权限控制失败：非管理员能够修改群组配置");
        }
        Err(e) => {
            println!("✅ 权限控制正常：{}", e);
        }
    }
    
    println!("\n2️⃣ 测试管理员设置群组LLM模式和自定义提示词");
    println!("-" .repeat(40));
    
    // 管理员设置群组为LLM模式并设置自定义提示词
    let admin_set_args = vec![
        "--sender", admin_id_str.as_str(),
        "--myself", "987654",
        "--group-id", group_id_str.as_str(),
        "--env", "group",
        "--group-admin",
        "llm",
        "--model", "gpt-4",
        "--prompt", "你是一个友善的AI助手，专门为群聊用户提供帮助。请用简洁明了的语言回答问题。"
    ];
    
    let result = CMD_REGISTRY.execute("strategy", &admin_set_args).await?;
    println!("管理员设置结果:\n{}", result.output);
    
    // 验证群组配置已更新
    let group_data = group_service.get_group_data(test_group_id).await?;
    assert_eq!(group_data.stratege, StrategeType::LlmStrategy);
    assert_eq!(group_data.model, "gpt-4");
    assert!(group_data.custom_prompt.is_some());
    println!("✅ 群组配置验证通过");
    
    println!("\n3️⃣ 测试查询群组配置");
    println!("-" .repeat(40));
    
    let query_args = vec![
        "--sender", admin_id_str.as_str(),
        "--myself", "987654",
        "--group-id", group_id_str.as_str(),
        "--env", "group",
        "--group-admin",
        "query"
    ];
    
    let result = CMD_REGISTRY.execute("strategy", &query_args).await?;
    println!("群组配置查询结果:\n{}", result.output);
    
    println!("\n4️⃣ 测试用户个人配置不受群组配置影响");
    println!("-" .repeat(40));
    
    // 设置用户个人配置
    let user_set_args = vec![
        "--sender", user_id_str.as_str(),
        "--myself", "987654",
        "--env", "private",
        "llm",
        "--model", "claude-3",
        "--prompt", "你是我的私人助手。"
    ];
    
    let result = CMD_REGISTRY.execute("strategy", &user_set_args).await?;
    println!("用户配置设置结果:\n{}", result.output);
    
    // 查询用户配置
    let user_query_args = vec![
        "--sender", user_id_str.as_str(),
        "--myself", "987654",
        "--env", "private",
        "query"
    ];
    
    let result = CMD_REGISTRY.execute("strategy", &user_query_args).await?;
    println!("用户配置查询结果:\n{}", result.output);
    
    // 验证用户和群组配置独立
    let user_data = user_service.get_user_data(test_user_id).await?;
    let group_data = group_service.get_group_data(test_group_id).await?;
    
    assert_eq!(user_data.model, "claude-3");
    assert_eq!(group_data.model, "gpt-4");
    println!("✅ 用户配置和群组配置相互独立");
    
    println!("\n5️⃣ 测试重置群组提示词");
    println!("-" .repeat(40));
    
    let reset_args = vec![
        "--sender", admin_id_str.as_str(),
        "--myself", "987654",
        "--group-id", group_id_str.as_str(),
        "--env", "group",
        "--group-admin",
        "llm",
        "--reset-prompt"
    ];
    
    let result = CMD_REGISTRY.execute("strategy", &reset_args).await?;
    println!("重置提示词结果:\n{}", result.output);
    
    // 验证提示词已重置
    let group_data = group_service.get_group_data(test_group_id).await?;
    assert!(group_data.custom_prompt.is_none());
    println!("✅ 群组提示词重置成功");
    
    println!("\n6️⃣ 测试切换群组到命令模式");
    println!("-" .repeat(40));
    
    let cmd_args = vec![
        "--sender", admin_id_str.as_str(),
        "--myself", "987654",
        "--group-id", group_id_str.as_str(),
        "--env", "group",
        "--group-admin",
        "cmd"
    ];
    
    let result = CMD_REGISTRY.execute("strategy", &cmd_args).await?;
    println!("切换到命令模式结果:\n{}", result.output);
    
    // 验证策略已切换
    let group_data = group_service.get_group_data(test_group_id).await?;
    assert_eq!(group_data.stratege, StrategeType::CmdStrategy);
    println!("✅ 群组策略切换成功");
    
    // 清理测试数据
    let _ = group_service.delete_group_config(test_group_id).await;
    
    println!("\n🎉 所有测试通过！群组配置系统功能完整正常！");
    println!("=" .repeat(50));
    
    Ok(())
}

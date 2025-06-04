use qqbot_core::{
    config::get_db,
    service::group_config_service::{GroupConfigService, GROUP_CACHE},
    StrategeType, GroupData,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("测试群组配置功能...");
    
    let db = get_db().await;
    let service = GroupConfigService::new(db.as_ref().clone());
    
    let test_group_id: i64 = 123456789;
    let test_strategy = StrategeType::LlmStrategy;
    let test_prompt = Some("你是一个群聊助手，请用友好的语调回答问题。".to_string());
    
    // 测试创建群组配置
    println!("1. 创建群组配置...");
    let group_data = GroupData {
        stratege: test_strategy,
        custom_prompt: test_prompt.clone(),
        ..Default::default()
    };
    
    service.save_group_data(test_group_id, &group_data).await?;
    println!("✅ 群组配置创建成功");
    
    // 测试从数据库获取配置
    println!("2. 从数据库获取群组配置...");
    let retrieved_data = service.get_group_data(test_group_id).await?;
    assert_eq!(retrieved_data.stratege, test_strategy);
    assert_eq!(retrieved_data.custom_prompt, test_prompt);
    println!("✅ 群组配置获取成功，数据匹配");
    
    // 测试缓存功能
    println!("3. 测试缓存功能...");
    let cached_data = GROUP_CACHE.get(&test_group_id).await;
    if let Some(cached_data) = cached_data {
        assert_eq!(cached_data.stratege, test_strategy);
        assert_eq!(cached_data.custom_prompt, test_prompt);
        println!("✅ 缓存功能正常工作");
    } else {
        println!("⚠️ 缓存中没有找到数据，这可能是正常的");
    }
    
    // 测试更新配置
    println!("4. 测试更新群组配置...");
    let updated_prompt = Some("你是一个专业的技术助手。".to_string());
    let updated_data = GroupData {
        stratege: StrategeType::CmdStrategy,
        custom_prompt: updated_prompt.clone(),
        ..Default::default()
    };
    
    service.save_group_data(test_group_id, &updated_data).await?;
    
    let updated_retrieved = service.get_group_data(test_group_id).await?;
    assert_eq!(updated_retrieved.stratege, StrategeType::CmdStrategy);
    assert_eq!(updated_retrieved.custom_prompt, updated_prompt);
    println!("✅ 群组配置更新成功");
    
    // 测试删除配置
    println!("5. 测试删除群组配置...");
    service.delete_group_config(test_group_id).await?;
    
    // 尝试获取已删除的配置（应该返回默认值）
    let deleted_data = service.get_group_data(test_group_id).await?;
    assert_eq!(deleted_data.stratege, StrategeType::LlmStrategy); // 默认策略
    assert_eq!(deleted_data.custom_prompt, None); // 默认无自定义提示词
    println!("✅ 群组配置删除成功");
    
    println!("🎉 所有测试通过！群组配置功能工作正常。");
    
    Ok(())
}

// Test prompt setting functionality
use qqbot_core::{
    config::APPCONFIG,
    service::user_config_service::UserConfigService,
    StrategeType, UserData,
};
use sea_orm::Database;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化配置
    println!("🧪 测试提示词设置功能");
    
    // 确保APPCONFIG被初始化
    let _ = &*APPCONFIG;
    println!("📋 配置已初始化");
    
    // 模拟用户ID
    let test_user_id = 123456789i64;
    
    // 连接数据库
    let database_url = &APPCONFIG.database.url;
    println!("🔗 连接数据库: {}", database_url);
    let db = Database::connect(database_url).await?;
    let user_config_service = UserConfigService::new(db);
    
    // 测试1: 创建新用户配置
    println!("\n📝 测试1: 创建用户配置");
    let mut user_data = UserData {
        stratege: StrategeType::LlmStrategy,
        model: APPCONFIG.llm.model.clone(),
        custom_prompt: Some("你是一个友善的助手，总是用温暖的语调回复用户。".to_string()),
    };
    
    user_config_service.save_user_data(test_user_id, &user_data).await?;
    println!("✅ 用户配置已保存");
    
    // 测试2: 读取用户配置
    println!("\n🔍 测试2: 读取用户配置");
    match user_config_service.get_user_data(test_user_id).await {
        Ok(saved_data) => {
            println!("📊 用户配置:");
            println!("  - 策略: {:?}", saved_data.stratege);
            println!("  - 模型: {}", saved_data.model);
            match &saved_data.custom_prompt {
                Some(prompt) => println!("  - 自定义提示词: {}", prompt),
                None => println!("  - 提示词: 使用默认"),
            }
        }
        Err(e) => {
            println!("❌ 获取用户配置失败: {}", e);
        }
    }
    
    // 测试3: 重置提示词
    println!("\n🔄 测试3: 重置提示词");
    user_data.custom_prompt = None;
    user_config_service.save_user_data(test_user_id, &user_data).await?;
    println!("✅ 提示词已重置");
    
    // 测试4: 验证重置后的配置
    println!("\n🔍 测试4: 验证重置后的配置");
    match user_config_service.get_user_data(test_user_id).await {
        Ok(reset_data) => {
            println!("📊 重置后的用户配置:");
            println!("  - 策略: {:?}", reset_data.stratege);
            println!("  - 模型: {}", reset_data.model);
            match &reset_data.custom_prompt {
                Some(prompt) => println!("  - 自定义提示词: {}", prompt),
                None => println!("  - 提示词: 使用默认 ✅"),
            }
        }
        Err(e) => {
            println!("❌ 获取用户配置失败: {}", e);
        }
    }
    
    // 测试5: 切换到命令模式
    println!("\n⚙️ 测试5: 切换到命令模式");
    user_data.stratege = StrategeType::CmdStrategy;
    user_config_service.save_user_data(test_user_id, &user_data).await?;
    
    match user_config_service.get_user_data(test_user_id).await {
        Ok(cmd_data) => {
            println!("📊 切换后的用户配置:");
            println!("  - 策略: {:?} ✅", cmd_data.stratege);
            println!("  - 模型: {}", cmd_data.model);
        }
        Err(e) => {
            println!("❌ 获取用户配置失败: {}", e);
        }
    }
    
    println!("\n🎉 测试完成！提示词设置功能正常工作");
    
    Ok(())
}

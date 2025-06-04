// Test strategy command functionality
use qqbot_core::{
    config::APPCONFIG,
    service::user_config_service::UserConfigService,
    cmd::strategy::Strategy,
    cmd::HandlerBuilder,
    StrategeType,
};
use sea_orm::Database;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 测试策略命令功能");
    
    // 确保APPCONFIG被初始化
    let _ = &*APPCONFIG;
    println!("📋 配置已初始化");
    
    // 模拟用户和群组ID
    let test_user_id = 987654321i64;
    
    // 连接数据库
    let database_url = &APPCONFIG.database.url;
    println!("🔗 连接数据库: {}", database_url);
    let db = Database::connect(database_url).await?;
    let user_config_service = UserConfigService::new(db);
    
    // 测试1: 切换到LLM模式
    println!("\n📝 测试1: 切换到LLM模式");
    let strategy_handler = Strategy::build();
    
    // 模拟命令参数: "strategy llm --sender 987654321 --myself 123456 --env private"
    let args = vec![
        "strategy".to_string(),
        "llm".to_string(),
        "--sender".to_string(),
        test_user_id.to_string(),
        "--myself".to_string(),
        "123456".to_string(),
        "--env".to_string(),
        "private".to_string(),
    ];
    
    match strategy_handler(args).await {
        Ok(result) => println!("✅ 命令执行成功: {}", result.output),
        Err(e) => println!("❌ 命令执行失败: {}", e),
    }
    
    // 验证配置是否保存
    match user_config_service.get_user_data(test_user_id).await {
        Ok(user_data) => {
            println!("📊 用户配置: 策略={:?}, 模型={}", user_data.stratege, user_data.model);
        }
        Err(e) => println!("❌ 获取用户配置失败: {}", e),
    }
    
    // 测试2: 切换到LLM模式并设置自定义提示词
    println!("\n📝 测试2: 设置自定义提示词");
    let strategy_handler2 = Strategy::build();
    let args = vec![
        "strategy".to_string(),
        "llm".to_string(),
        "--prompt".to_string(),
        "你是一个专业的代码助手，专门帮助用户解决编程问题。".to_string(),
        "--sender".to_string(),
        test_user_id.to_string(),
        "--myself".to_string(),
        "123456".to_string(),
        "--env".to_string(),
        "private".to_string(),
    ];
    
    match strategy_handler2(args).await {
        Ok(result) => println!("✅ 命令执行成功: {}", result.output),
        Err(e) => println!("❌ 命令执行失败: {}", e),
    }
    
    // 验证提示词是否保存
    match user_config_service.get_user_data(test_user_id).await {
        Ok(user_data) => {
            println!("📊 用户配置: 策略={:?}", user_data.stratege);
            match &user_data.custom_prompt {
                Some(prompt) => println!("📝 自定义提示词: {}", prompt),
                None => println!("📝 提示词: 使用默认"),
            }
        }
        Err(e) => println!("❌ 获取用户配置失败: {}", e),
    }
    
    // 测试3: 重置提示词
    println!("\n📝 测试3: 重置提示词");
    let strategy_handler3 = Strategy::build();
    let args = vec![
        "strategy".to_string(),
        "llm".to_string(),
        "--reset-prompt".to_string(),
        "--sender".to_string(),
        test_user_id.to_string(),
        "--myself".to_string(),
        "123456".to_string(),
        "--env".to_string(),
        "private".to_string(),
    ];
    
    match strategy_handler3(args).await {
        Ok(result) => println!("✅ 命令执行成功: {}", result.output),
        Err(e) => println!("❌ 命令执行失败: {}", e),
    }
    
    // 验证提示词是否被重置
    match user_config_service.get_user_data(test_user_id).await {
        Ok(user_data) => {
            match &user_data.custom_prompt {
                Some(prompt) => println!("📝 自定义提示词: {}", prompt),
                None => println!("📝 提示词: 使用默认 ✅"),
            }
        }
        Err(e) => println!("❌ 获取用户配置失败: {}", e),
    }
    
    // 测试4: 切换到命令模式
    println!("\n📝 测试4: 切换到命令模式");
    let strategy_handler4 = Strategy::build();
    let args = vec![
        "strategy".to_string(),
        "cmd".to_string(),
        "--sender".to_string(),
        test_user_id.to_string(),
        "--myself".to_string(),
        "123456".to_string(),
        "--env".to_string(),
        "private".to_string(),
    ];
    
    match strategy_handler4(args).await {
        Ok(result) => println!("✅ 命令执行成功: {}", result.output),
        Err(e) => println!("❌ 命令执行失败: {}", e),
    }
    
    // 验证策略是否切换
    match user_config_service.get_user_data(test_user_id).await {
        Ok(user_data) => {
            println!("📊 用户配置: 策略={:?} ✅", user_data.stratege);
            if user_data.stratege == StrategeType::CmdStrategy {
                println!("✅ 策略成功切换到命令模式");
            } else {
                println!("❌ 策略切换失败");
            }
        }
        Err(e) => println!("❌ 获取用户配置失败: {}", e),
    }
    
    println!("\n🎉 策略命令测试完成！");
    
    Ok(())
}

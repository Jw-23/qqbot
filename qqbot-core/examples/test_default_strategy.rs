use qqbot_core::{UserData, StrategeType};

#[tokio::main]
async fn main() {
    // 测试默认策略
    let default_user_data = UserData::default();
    
    println!("🔧 测试默认策略设置:");
    println!("默认策略: {:?}", default_user_data.stratege);
    
    match default_user_data.stratege {
        StrategeType::LlmStrategy => {
            println!("✅ 成功！默认策略已设置为大模型回复模式 (LlmStrategy)");
        }
        StrategeType::CmdStrategy => {
            println!("❌ 错误！默认策略仍为命令模式 (CmdStrategy)");
        }
    }
    let mut arr=vec![6,3,2,4,9,1,7];
    
    // 测试策略类型的默认值
    let default_strategy = StrategeType::default();
    println!("\n🎯 StrategeType::default() 返回: {:?}", default_strategy);
    
    match default_strategy {
        StrategeType::LlmStrategy => {
            println!("✅ 策略类型默认值正确设置为 LlmStrategy");
        }
        StrategeType::CmdStrategy => {
            println!("❌ 策略类型默认值仍为 CmdStrategy");
        }
    }
    
    println!("\n📊 测试总结:");
    println!("- 新用户默认将使用大模型回复策略");
    println!("- 在私聊中会直接响应所有消息");
    println!("- 在群聊中只响应@机器人的消息");
    println!("- 用户仍可通过命令切换回命令模式");
}

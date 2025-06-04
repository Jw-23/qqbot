use sea_orm::ActiveValue::Set;
use sea_orm::{entity::prelude::*, ActiveValue::NotSet};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sea_query::Expr;
use serde::{Deserialize, Serialize};
use crate::StrategeType;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "user_config")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: i64, // QQ用户ID作为主键
    pub strategy: String, // 策略类型 (cmd_strategy, llm_strategy)
    pub model: Option<String>, // LLM模型名称
    pub custom_prompt: Option<String>, // 自定义提示词
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: DateTimeWithTimeZone, // 创建时间
    #[sea_orm(
        default_expr = "Expr::current_timestamp()",
        on_update = "Expr::current_timestamp()"
    )]
    pub updated_at: DateTimeWithTimeZone, // 更新时间
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    /// 将数据库模型转换为业务模型
    pub fn to_user_data(&self, default_model: &str) -> crate::UserData {
        let strategy = match self.strategy.as_str() {
            "cmd_strategy" => StrategeType::CmdStrategy,
            "llm_strategy" => StrategeType::LlmStrategy,
            _ => StrategeType::default(),
        };

        crate::UserData {
            stratege: strategy,
            model: self.model.clone().unwrap_or_else(|| default_model.to_string()),
            custom_prompt: self.custom_prompt.clone(),
        }
    }

    /// 从业务模型创建数据库模型
    pub fn from_user_data(user_id: i64, user_data: &crate::UserData) -> ActiveModel {
        let strategy_str = match user_data.stratege {
            StrategeType::CmdStrategy => "cmd_strategy",
            StrategeType::LlmStrategy => "llm_strategy",
        };

        ActiveModel {
            user_id: Set(user_id),
            strategy: Set(strategy_str.to_string()),
            model: Set(if user_data.model.is_empty() { 
                None 
            } else { 
                Some(user_data.model.clone()) 
            }),
            custom_prompt: Set(user_data.custom_prompt.clone()),
            created_at: NotSet,
            updated_at: NotSet,
        }
    }
}

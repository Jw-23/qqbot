use sea_orm::ActiveValue::Set;
use sea_orm::{entity::prelude::*, ActiveValue::NotSet};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sea_query::Expr;
use serde::{Deserialize, Serialize};
use crate::StrategeType;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "group_config")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub group_id: i64, // QQ群ID作为主键
    pub strategy: String, // 策略类型 (cmd_strategy, llm_strategy)
    pub model: Option<String>, // LLM模型名称
    pub custom_prompt: Option<String>, // 群组自定义提示词
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
    pub fn to_group_data(&self, default_model: &str) -> crate::GroupData {
        let strategy = match self.strategy.as_str() {
            "cmd_strategy" => StrategeType::CmdStrategy,
            "llm_strategy" => StrategeType::LlmStrategy,
            _ => StrategeType::default(),
        };

        crate::GroupData {
            stratege: strategy,
            model: self.model.clone().unwrap_or_else(|| default_model.to_string()),
            custom_prompt: self.custom_prompt.clone(),
        }
    }

    /// 从业务模型创建数据库模型
    pub fn from_group_data(group_id: i64, group_data: &crate::GroupData) -> ActiveModel {
        let strategy_str = match group_data.stratege {
            StrategeType::CmdStrategy => "cmd_strategy",
            StrategeType::LlmStrategy => "llm_strategy",
        };

        ActiveModel {
            group_id: Set(group_id),
            strategy: Set(strategy_str.to_string()),
            model: Set(if group_data.model.is_empty() { 
                None 
            } else { 
                Some(group_data.model.clone()) 
            }),
            custom_prompt: Set(group_data.custom_prompt.clone()),
            created_at: NotSet,
            updated_at: NotSet,
        }
    }
}

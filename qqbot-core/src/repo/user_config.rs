use crate::models::user_config::{Entity as UserConfigEntity, Model as UserConfigModel};
use async_trait::async_trait;
use sea_orm::{DbConn, EntityTrait, DbErr, ActiveModelTrait};
use crate::{UserData, UserId};

#[async_trait]
pub trait UserConfigRepository {
    async fn find_by_user_id(&self, user_id: UserId) -> Result<Option<UserConfigModel>, DbErr>;
    async fn upsert_user_config(&self, user_id: UserId, user_data: &UserData) -> Result<(), DbErr>;
    async fn delete_user_config(&self, user_id: UserId) -> Result<(), DbErr>;
}

pub struct UserConfigRepo {
    db: DbConn,
}

impl UserConfigRepo {
    pub fn new(db: DbConn) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserConfigRepository for UserConfigRepo {
    async fn find_by_user_id(&self, user_id: UserId) -> Result<Option<UserConfigModel>, DbErr> {
        UserConfigEntity::find_by_id(user_id)
            .one(&self.db)
            .await
    }

    async fn upsert_user_config(&self, user_id: UserId, user_data: &UserData) -> Result<(), DbErr> {
        use sea_orm::Set;
        
        // 先尝试查找现有记录
        let existing = UserConfigEntity::find_by_id(user_id)
            .one(&self.db)
            .await?;
            
        match existing {
            Some(_) => {
                // 记录存在，执行更新
                let mut active_model: crate::models::user_config::ActiveModel = Default::default();
                active_model.user_id = Set(user_id);
                
                let strategy_str = match user_data.stratege {
                    crate::StrategeType::CmdStrategy => "cmd_strategy",
                    crate::StrategeType::LlmStrategy => "llm_strategy",
                };
                
                active_model.strategy = Set(strategy_str.to_string());
                active_model.model = Set(if user_data.model.is_empty() { 
                    None 
                } else { 
                    Some(user_data.model.clone()) 
                });
                active_model.custom_prompt = Set(user_data.custom_prompt.clone());
                
                active_model.update(&self.db).await?;
            }
            None => {
                // 记录不存在，执行插入
                let active_model = UserConfigModel::from_user_data(user_id, user_data);
                active_model.insert(&self.db).await?;
            }
        }
        
        Ok(())
    }

    async fn delete_user_config(&self, user_id: UserId) -> Result<(), DbErr> {
        UserConfigEntity::delete_by_id(user_id)
            .exec(&self.db)
            .await?;
        Ok(())
    }
}

use async_trait::async_trait;
use sea_orm::{DbConn, DbErr, EntityTrait, ActiveModelTrait};
use crate::{GroupData, GroupId};
use crate::models::group_config::{Entity as GroupConfigEntity, Model as GroupConfigModel};

#[async_trait]
pub trait GroupConfigRepository {
    async fn find_by_group_id(&self, group_id: GroupId) -> Result<Option<GroupConfigModel>, DbErr>;
    async fn upsert_group_config(&self, group_id: GroupId, group_data: &GroupData) -> Result<(), DbErr>;
    async fn delete_group_config(&self, group_id: GroupId) -> Result<(), DbErr>;
}

pub struct GroupConfigRepo {
    pub db: DbConn,
}

impl GroupConfigRepo {
    pub fn new(db: DbConn) -> Self {
        Self { db }
    }
}

#[async_trait]
impl GroupConfigRepository for GroupConfigRepo {
    async fn find_by_group_id(&self, group_id: GroupId) -> Result<Option<GroupConfigModel>, DbErr> {
        GroupConfigEntity::find_by_id(group_id)
            .one(&self.db)
            .await
    }

    async fn upsert_group_config(&self, group_id: GroupId, group_data: &GroupData) -> Result<(), DbErr> {
        use sea_orm::Set;
        
        // 先尝试查找现有记录
        let existing = GroupConfigEntity::find_by_id(group_id)
            .one(&self.db)
            .await?;
            
        match existing {
            Some(_) => {
                // 记录存在，执行更新
                let mut active_model: crate::models::group_config::ActiveModel = Default::default();
                active_model.group_id = Set(group_id);
                
                let strategy_str = match group_data.stratege {
                    crate::StrategeType::CmdStrategy => "cmd_strategy",
                    crate::StrategeType::LlmStrategy => "llm_strategy",
                };
                
                active_model.strategy = Set(strategy_str.to_string());
                active_model.model = Set(if group_data.model.is_empty() { 
                    None 
                } else { 
                    Some(group_data.model.clone()) 
                });
                active_model.custom_prompt = Set(group_data.custom_prompt.clone());
                
                active_model.update(&self.db).await?;
            }
            None => {
                // 记录不存在，执行插入
                let active_model = GroupConfigModel::from_group_data(group_id, group_data);
                active_model.insert(&self.db).await?;
            }
        }
        
        Ok(())
    }

    async fn delete_group_config(&self, group_id: GroupId) -> Result<(), DbErr> {
        GroupConfigEntity::delete_by_id(group_id)
            .exec(&self.db)
            .await?;
        Ok(())
    }
}

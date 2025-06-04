use crate::{
    repo::user_config::{UserConfigRepository, UserConfigRepo},
    UserData, UserId, BOT_CACHE,
    config::APPCONFIG,
    error::{AppError, AppResult},
};
use sea_orm::DbConn;
use std::sync::Arc;

pub struct UserConfigService {
    repo: Arc<dyn UserConfigRepository + Send + Sync>,
}

impl UserConfigService {
    pub fn new(db: DbConn) -> Self {
        Self {
            repo: Arc::new(UserConfigRepo::new(db)),
        }
    }

    /// 获取用户配置，优先从缓存读取，缓存未命中则从数据库读取
    pub async fn get_user_data(&self, user_id: UserId) -> AppResult<UserData> {
        // 先尝试从缓存获取
        if let Some(user_data) = BOT_CACHE.get(&user_id).await {
            return Ok(user_data);
        }

        // 缓存未命中，从数据库读取
        match self.repo.find_by_user_id(user_id).await {
            Ok(Some(config)) => {
                let user_data = config.to_user_data(&APPCONFIG.llm.model);
                // 同步到缓存
                BOT_CACHE.insert(user_id, user_data.clone()).await;
                Ok(user_data)
            }
            Ok(None) => {
                // 用户配置不存在，返回默认配置
                let user_data = UserData::default();
                Ok(user_data)
            }
            Err(e) => Err(AppError::Database(e)),
        }
    }

    /// 保存用户配置到数据库和缓存
    pub async fn save_user_data(&self, user_id: UserId, user_data: &UserData) -> AppResult<()> {
        // 保存到数据库
        self.repo.upsert_user_config(user_id, user_data)
            .await
            .map_err(|e| AppError::Database(e))?;

        // 同步到缓存
        BOT_CACHE.insert(user_id, user_data.clone()).await;

        Ok(())
    }

    /// 删除用户配置
    pub async fn delete_user_config(&self, user_id: UserId) -> AppResult<()> {
        // 从数据库删除
        self.repo.delete_user_config(user_id)
            .await
            .map_err(|e| AppError::Database(e))?;

        // 从缓存删除
        BOT_CACHE.remove(&user_id).await;

        Ok(())
    }
}

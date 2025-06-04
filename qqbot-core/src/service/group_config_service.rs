use crate::{
    AppResult, AppError, GroupData, GroupId,
    repo::group_config::{GroupConfigRepo, GroupConfigRepository},
    config::APPCONFIG,
};
use sea_orm::DbConn;
use moka::future::Cache;
use once_cell::sync::Lazy;

// 群组配置缓存
pub static GROUP_CACHE: Lazy<Cache<GroupId, GroupData>> = Lazy::new(|| {
    Cache::builder()
        .max_capacity(APPCONFIG.cache.cache_capacity / 10) // 群组数量通常比用户少
        .time_to_live(APPCONFIG.cache.cache_lifetime)
        .time_to_idle(APPCONFIG.cache.cache_idletime)
        .build()
});

pub struct GroupConfigService {
    repo: GroupConfigRepo,
}

impl GroupConfigService {
    pub fn new(db: DbConn) -> Self {
        Self {
            repo: GroupConfigRepo::new(db),
        }
    }

    /// 获取群组配置，优先从缓存读取，缓存未命中则从数据库读取
    pub async fn get_group_data(&self, group_id: GroupId) -> AppResult<GroupData> {
        // 先尝试从缓存获取
        if let Some(group_data) = GROUP_CACHE.get(&group_id).await {
            return Ok(group_data);
        }

        // 缓存未命中，从数据库读取
        match self.repo.find_by_group_id(group_id).await {
            Ok(Some(config)) => {
                let group_data = config.to_group_data(&APPCONFIG.llm.model);
                // 同步到缓存
                GROUP_CACHE.insert(group_id, group_data.clone()).await;
                Ok(group_data)
            }
            Ok(None) => {
                // 群组配置不存在，返回默认配置
                let group_data = GroupData::default();
                Ok(group_data)
            }
            Err(e) => Err(AppError::Database(e)),
        }
    }

    /// 保存群组配置到数据库和缓存
    pub async fn save_group_data(&self, group_id: GroupId, group_data: &GroupData) -> AppResult<()> {
        // 保存到数据库
        self.repo.upsert_group_config(group_id, group_data)
            .await
            .map_err(|e| AppError::Database(e))?;

        // 同步到缓存
        GROUP_CACHE.insert(group_id, group_data.clone()).await;

        Ok(())
    }

    /// 删除群组配置
    pub async fn delete_group_config(&self, group_id: GroupId) -> AppResult<()> {
        // 从数据库删除
        self.repo.delete_group_config(group_id)
            .await
            .map_err(|e| AppError::Database(e))?;

        // 从缓存删除
        GROUP_CACHE.remove(&group_id).await;

        Ok(())
    }
}

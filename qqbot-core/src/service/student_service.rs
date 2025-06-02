use std::sync::Arc;

use crate::{ 
    models::student::Model, 
    repo::student::{StudentRepo, StudentRepository},
    error::{AppError, AppResult}
};
use sea_orm::DatabaseConnection;

pub trait UserService {
    fn get(&self, id: i64) -> impl std::future::Future<Output = AppResult<Model>> + Send;
    fn find_by_qq(&self, qq: i64) -> impl std::future::Future<Output = AppResult<Model>> + Send;
    fn update_qq(&self, id: i64, qq: i64) -> impl std::future::Future<Output = AppResult<()>> + Send;
} 

pub struct StuServiceImpl{
    repo: StudentRepo
}

impl StuServiceImpl {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        StuServiceImpl { repo: StudentRepo::new(db) }
    }
}

impl UserService for StuServiceImpl {
    async fn get(&self, id: i64) -> AppResult<Model> {
        match self.repo.find_by_id(id).await? {
            Some(model) => Ok(model),
            None => Err(AppError::not_found(format!("学生ID: {}", id))),
        }
    }
    
    async fn find_by_qq(&self, qq: i64) -> AppResult<Model> {
        match self.repo.find_by_qq(qq).await? {
            Some(model) => Ok(model),
            None => Err(AppError::not_found(format!("QQ号: {}", qq))),
        }
    }
    
    async fn update_qq(&self, id: i64, qq: i64) -> AppResult<()> {
        self.repo.update_qq(id, qq).await?;
        Ok(())
    }
}
use std::sync::Arc;

use crate::{ models::{grade::Grade, student::{Model, Student}}, repo::{self, student::{StudentRepo, StudentRepository}}};
use crate::models::{student, grade};
use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, ModelTrait, QueryFilter};

use super::ServiceError;

pub trait UserService {
    fn get(&self,id:i64)->impl std::future::Future<Output = Result<Model,ServiceError>> + Send;
    fn find_by_qq(&self,qq:i64)->impl std::future::Future<Output = Result<Model,ServiceError>> + Send;
    fn update_qq(&self,id:i64,qq:i64)->impl std::future::Future<Output = Result<(),ServiceError>> + Send;
} 

pub struct StuServiceImpl{
    repo:StudentRepo
}
impl StuServiceImpl {
    pub fn new(db:Arc<DatabaseConnection>)->Self{
        StuServiceImpl { repo: StudentRepo::new(db) }
    }
}

impl UserService for StuServiceImpl {
    async fn get(&self,id:i64)->Result<Model,ServiceError> {
        match self.repo.find_by_id(id).await {
            Ok(Some(model))=>Ok(model),
            Err(_)=>Err(ServiceError::new("student", "database broken")),
            _=>Err(ServiceError::new("student", "undefined"))
        }
    }
    
    async fn find_by_qq(&self,qq:i64)->Result<Model,ServiceError> {
        match self.repo.find_by_qq(qq).await{
            Ok(Some(model))=>Ok(model),
            Err(_)=>Err(ServiceError::new("student", "database broken")),
            _=>Err(ServiceError::new("student", "student is not found"))
        }
    }
    
    async fn update_qq(&self,id:i64,qq:i64)->Result<(),ServiceError>{
        match self.repo.update_qq(id, qq).await{
            Err(err)=>Err(ServiceError::new("student", &format!("failed to update qq: {}",err.to_string()))),
            _=>Ok(())
        }
        
    }
    
}
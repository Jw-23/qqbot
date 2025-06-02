use std::sync::Arc;

use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use crate::models::student::{ActiveModel, Column, Entity, Model};
use super::DbErr;

#[async_trait]
pub trait StudentRepository {
    async fn register(
        &self,
        name: String,
        student_id: i64,
        qq: i64,
        group_id: i64
    ) -> Result<Model, DbErr>;
    
    async fn find_by_qq(&self, qq: i64) -> Result<Option<Model>, DbErr>;
    async fn find_by_id(&self,id: i64)->Result<Option<Model>,DbErr>;
    async fn update_qq(&self,id:i64,qq:i64)->Result<(),DbErr>;
}

pub struct StudentRepo {
    db: Arc<DatabaseConnection>,
}

impl StudentRepo {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl StudentRepository for StudentRepo {
    async fn register(
        &self,
        name: String,
        student_id: i64,
        qq: i64,
        group_id: i64
    ) -> Result<Model, DbErr> {
        let student = ActiveModel {
            name: Set(name),
            student_id: Set(student_id),
            qq_number: Set(qq),
            group_id: Set(group_id),
            ..Default::default()
        };
        
        Entity::insert(student)
            .exec_with_returning(self.db.as_ref())
            .await
    }

    async fn find_by_qq(&self, qq: i64) -> Result<Option<Model>, DbErr> {
        Entity::find()
            .filter(Column::QqNumber.eq(qq))
            .one(self.db.as_ref())
            .await
    }
    async fn find_by_id(&self,id: i64)->Result<Option<Model>,DbErr>{
        Entity::find()
        .filter(Column::StudentId.eq(id))
        .one(self.db.as_ref())
        .await
    }
    async fn update_qq(&self,id:i64,qq:i64)->Result<(),DbErr>{
       match self.find_by_id(id).await {
            Ok(Some(stu))=> {
              let mut active:ActiveModel = stu.into();
                active.qq_number=Set(qq);
                active.update(self.db.as_ref()).await?;
                Ok(())
            },
            Err(err)=>Err(err),
            _=> Err(DbErr::Custom("not defined behavior".into()))   
       }
    }
} 

use std::sync::Arc;

use crate::{
    error::{AppError, AppResult},
    models::student::Model,
    repo::student::{StudentRepo, StudentRepository},
};
use sea_orm::DatabaseConnection;

pub trait UserService {
    fn get(&self, id: i64) -> impl std::future::Future<Output = AppResult<Model>> + Send;
    fn find_by_qq(&self, qq: i64) -> impl std::future::Future<Output = AppResult<Model>> + Send;
    fn update_qq(
        &self,
        id: i64,
        qq: i64,
    ) -> impl std::future::Future<Output = AppResult<()>> + Send;
}

pub struct StuServiceImpl {
    repo: StudentRepo,
}

impl StuServiceImpl {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        StuServiceImpl {
            repo: StudentRepo::new(db),
        }
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

// 添加便利函数用于admin后台
pub async fn create_student(
    student_id: i64,
    name: &str,
    qq_number: i64,
    group_id: i64,
) -> AppResult<Model> {
    use crate::config::get_db;
    use crate::models::student::{ActiveModel, Entity as Student};
    use sea_orm::{ActiveModelTrait, Set};

    let db = get_db().await;

    let student = ActiveModel {
        student_id: Set(student_id),
        name: Set(name.to_string()),
        qq_number: Set(qq_number),
        group_id: Set(group_id),
        ..Default::default()
    };

    let result = student.insert(db.as_ref()).await.map_err(AppError::from)?;
    Ok(result)
}

pub async fn update_student_by_id(
    id: i64,
    student_id: Option<i64>,
    name: Option<String>,
    qq_number: Option<i64>,
    group_id: Option<i64>,
) -> AppResult<Model> {
    use crate::config::get_db;
    use crate::models::student::{ActiveModel, Entity as Student};
    use sea_orm::{ActiveModelTrait, EntityTrait, Set};
    
    let db = get_db().await;
    
    let student = Student::find_by_id(id)
        .one(db.as_ref())
        .await
        .map_err(AppError::from)?
        .ok_or_else(|| AppError::not_found(format!("学生ID: {}", id)))?;
    
    let mut active_model: ActiveModel = student.into();
    
    if let Some(student_id) = student_id {
        active_model.student_id = Set(student_id);
    }
    if let Some(name) = name {
        active_model.name = Set(name);
    }
    if let Some(qq_number) = qq_number {
        active_model.qq_number = Set(qq_number);
    }
    if let Some(group_id) = group_id {
        active_model.group_id = Set(group_id);
    }
    
    let result = active_model.update(db.as_ref()).await.map_err(AppError::from)?;
    Ok(result)
}

pub async fn delete_student_by_id(id: i64) -> AppResult<()> {
    use crate::config::get_db;
    use crate::models::student::Entity as Student;
    use sea_orm::EntityTrait;

    let db = get_db().await;

    Student::delete_by_id(id)
        .exec(db.as_ref())
        .await
        .map_err(AppError::from)?;

    Ok(())
}

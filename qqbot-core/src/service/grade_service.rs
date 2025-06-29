use std::sync::Arc;

use sea_orm::DatabaseConnection;

use crate::{
    error::{AppError, AppResult},
    models::grade::Model,
    permission::check_permission,
    repo::{GradeRepository, grade::GradeRepo},
};

use super::{StuServiceImpl, UserService};

pub trait GradeService {
    fn find_grades(
        &self,
        qq: i64,
    ) -> impl std::future::Future<Output = AppResult<Vec<Model>>> + Send;
}

pub struct GradeServiceImpl {
    repo: GradeRepo,
    db: Arc<DatabaseConnection>,
}

impl GradeServiceImpl {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        GradeServiceImpl {
            repo: GradeRepo::new(db.clone()),
            db,
        }
    }
}

impl GradeService for GradeServiceImpl {
    async fn find_grades(&self, qq: i64) -> AppResult<Vec<Model>> {
        let ss = StuServiceImpl::new(self.db.clone());

        let stu = ss.find_by_qq(qq).await?;

        // 检查权限
        if stu.qq_number != qq && !check_permission(qq) {
            return Err(AppError::permission("你无法查看他人的成绩"));
        }

        let grades = self.repo.query_grades(stu.student_id).await?;
        Ok(grades)
    }
}

// 添加便利函数用于admin后台
pub async fn create_grade(
    student_name: &str,
    exam_name: &str,
    course_id: i32,
    course_seq: i8,
    student_id: i64,
    score: i8,
    category: &str,
) -> AppResult<Model> {
    use crate::config::get_db;
    use crate::models::grade::{ActiveModel, Entity as Grade};
    use sea_orm::{ActiveModelTrait, Set};

    let db = get_db().await;

    let grade = ActiveModel {
        student_name: Set(student_name.to_string()),
        exam_name: Set(exam_name.to_string()),
        course_id: Set(course_id),
        course_seq: Set(course_seq),
        student_id: Set(student_id),
        score: Set(score),
        category: Set(category.to_string()),
        ..Default::default()
    };

    let result = grade.insert(db.as_ref()).await.map_err(AppError::from)?;
    Ok(result)
}

pub async fn update_grade_by_id(
    id: i64,
    student_name: Option<String>,
    exam_name: Option<String>,
    course_id: Option<i32>,
    course_seq: Option<i8>,
    student_id: Option<i64>,
    score: Option<i8>,
    category: Option<String>,
) -> AppResult<Model> {
    use crate::config::get_db;
    use crate::models::grade::{ActiveModel, Entity as Grade};
    use sea_orm::{ActiveModelTrait, EntityTrait, Set};
    
    let db = get_db().await;
    
    let grade = Grade::find_by_id(id)
        .one(db.as_ref())
        .await
        .map_err(AppError::from)?
        .ok_or_else(|| AppError::not_found(format!("成绩ID: {}", id)))?;
    
    let mut active_model: ActiveModel = grade.into();
    
    if let Some(student_name) = student_name {
        active_model.student_name = Set(student_name);
    }
    if let Some(exam_name) = exam_name {
        active_model.exam_name = Set(exam_name);
    }
    if let Some(course_id) = course_id {
        active_model.course_id = Set(course_id);
    }
    if let Some(course_seq) = course_seq {
        active_model.course_seq = Set(course_seq);
    }
    if let Some(student_id) = student_id {
        active_model.student_id = Set(student_id);
    }
    if let Some(score) = score {
        active_model.score = Set(score);
    }
    if let Some(category) = category {
        active_model.category = Set(category);
    }
    
    let result = active_model.update(db.as_ref()).await.map_err(AppError::from)?;
    Ok(result)
}

pub async fn delete_grade_by_id(id: i64) -> AppResult<()> {
    use crate::config::get_db;
    use crate::models::grade::Entity as Grade;
    use sea_orm::EntityTrait;

    let db = get_db().await;

    Grade::delete_by_id(id)
        .exec(db.as_ref())
        .await
        .map_err(AppError::from)?;

    Ok(())
}

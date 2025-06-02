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

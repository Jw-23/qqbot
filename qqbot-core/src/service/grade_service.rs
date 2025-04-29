use std::sync::Arc;

use sea_orm::{DatabaseConnection, ModelTrait};

use crate::{
    models::grade::{Model}, permission::check_permission, repo::{grade::GradeRepo, GradeRepository}
};

use super::{ServiceError, StuServiceImpl, UserService};

pub trait GradeService {
    fn find_grades(&self, qq: i64) -> impl std::future::Future<Output = Result<Vec<Model>,ServiceError>> + Send;
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
    async fn find_grades(&self, qq: i64) -> Result<Vec<Model>, ServiceError> {
        let ss = StuServiceImpl::new(self.db.clone());
        if let Ok(stu) = ss.find_by_qq(qq).await  {
            // check permission
            if stu.qq_number!=qq && !check_permission(qq) {
                return Err(ServiceError::new("grade", "you can't others' grades"))
            }
            return self
                .repo
                .query_grades(stu.student_id)
                .await
                .map_err(|err| ServiceError::new("grade", &err.to_string()));
        }
        Err(ServiceError::new(
            "grade",
            &format!("{} not found in database", qq),
        ))
    }
}

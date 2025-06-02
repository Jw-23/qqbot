use std::sync::Arc;

use super::{DbErr, GradeRepository};
use crate::models::grade::{ActiveModel, Column, Entity as GradeEntity, Model as GradeModel};
use async_trait::async_trait;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

pub struct GradeRepo {
    db: Arc<DatabaseConnection>,
}

impl GradeRepo {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl GradeRepository for GradeRepo {
    async fn query_grades(&self, student_id: i64) -> Result<Vec<GradeModel>, DbErr> {
        GradeEntity::find()
            .filter(Column::StudentId.eq(student_id))
            .all(self.db.as_ref())
            .await
    }

    async fn add_grade(&self, course_id: i32, student_id: i64, score: i8,exam_name:String,student_name:String) -> Result<(), DbErr> {
        let grade = ActiveModel {
            course_id: Set(course_id),
            student_id: Set(student_id),
            score: Set(score),
            exam_name:Set(exam_name),
            student_name:Set(student_name),
            ..Default::default()
        };
        
        GradeEntity::insert(grade).exec(self.db.as_ref()).await?;
        
        Ok(())
    }
}


#[tokio::test]
async fn query_grade_test() -> Result<(), Box<dyn std::error::Error>> {
    let db: DatabaseConnection = sea_orm::Database::connect("mysql://root:@localhost/diesel_demo").await?;
    let repo = GradeRepo::new(Arc::new(db));
    let grades = repo.query_grades(12345678).await?;
    println!("grades is: {:#?}", grades);
    Ok(())
}

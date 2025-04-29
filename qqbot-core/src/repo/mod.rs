use crate::models::{grade::Model as GradeModel, student::Model as StudentModel};
use async_trait::async_trait;
use sea_orm::DbErr;

#[async_trait]
pub trait StudentRepository {
    async fn find_by_id(&self, id: i64) -> Result<Option<StudentModel>, DbErr>;
    async fn add_student(&self, name: String, student_id: i64) -> Result<(), DbErr>;
}

#[async_trait]
pub trait GradeRepository {
    async fn query_grades(&self, student_id: i64) -> Result<Vec<GradeModel>, DbErr>;
    async fn add_grade(
        &self,
        course_id: i32,
        student_id: i64,
        score: i8,
        exam_name: String,
        student_name: String,
    ) -> Result<(), DbErr>;
}

pub mod grade;
pub mod student;

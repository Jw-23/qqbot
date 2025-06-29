use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GradeDto {
    pub id: Option<i64>,
    pub student_name: String,
    pub exam_name: String,
    pub course_id: i32,
    pub course_seq: i8,
    pub student_id: i64,
    pub score: i8,
    pub category: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateGradeRequest {
    pub student_name: String,
    pub exam_name: String,
    pub course_id: i32,
    pub course_seq: i8,
    pub student_id: i64,
    pub score: i8,
    pub category: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateGradeRequest {
    pub student_name: Option<String>,
    pub exam_name: Option<String>,
    pub course_id: Option<i32>,
    pub course_seq: Option<i8>,
    pub student_id: Option<i64>,
    pub score: Option<i8>,
    pub category: Option<String>,
}

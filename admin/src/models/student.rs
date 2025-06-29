use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct StudentDto {
    pub id: Option<i64>,
    pub student_id: i64,
    pub name: String,
    pub qq_number: i64,
    pub group_id: i64,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateStudentRequest {
    pub student_id: i64,
    pub name: String,
    pub qq_number: i64,
    pub group_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateStudentRequest {
    pub student_id: Option<i64>,
    pub name: Option<String>,
    pub qq_number: Option<i64>,
    pub group_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BulkMessageRequest {
    pub student_ids: Vec<i64>,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImportStudentsRequest {
    pub students: Vec<CreateStudentRequest>,
}

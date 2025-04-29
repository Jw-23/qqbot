pub mod student_service;
use core::fmt;

use serde::{Deserialize, Serialize};
pub use student_service::*;
pub mod grade_service;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceError {
    category: String,
    msg: String,
}
impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "service {} error: {}", self.category, self.msg)
    }
}
impl ServiceError {
    pub fn new(category: &str, msg: &str) -> Self {
        ServiceError {
            category: category.into(),
            msg: msg.into(),
        }
    }
}
impl std::error::Error for ServiceError {}

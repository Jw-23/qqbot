pub mod student_service;
pub use student_service::*;
pub mod grade_service;
pub mod group_config_service;
pub mod user_config_service;

// 重新导出新的错误类型
pub use crate::error::{AppError, AppResult};

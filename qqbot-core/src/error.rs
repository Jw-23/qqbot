use serde_json;
use thiserror::Error;

/// 统一的应用错误类型
#[derive(Error, Debug)]
pub enum AppError {
    /// 数据库相关错误
    #[error("数据库错误: {0}")]
    Database(#[from] sea_orm::DbErr),

    /// 配置相关错误
    #[error("配置错误: {message}")]
    Config { message: String },

    /// 学生服务相关错误
    #[error("学生服务错误: {message}")]
    Student { message: String },

    /// 成绩服务相关错误
    #[error("成绩服务错误: {message}")]
    Grade { message: String },

    /// 权限相关错误
    #[error("权限错误: {message}")]
    Permission { message: String },

    /// 命令执行错误
    #[error("命令执行错误: {message}")]
    Command { message: String },

    /// 回复策略错误
    #[error("回复错误: {message}")]
    Reply { message: String },

    /// HTTP请求错误
    #[error("HTTP请求错误: {0}")]
    Http(#[from] reqwest::Error),

    /// JSON序列化/反序列化错误
    #[error("JSON处理错误: {0}")]
    Json(#[from] serde_json::Error),

    /// 参数解析错误
    #[error("参数解析错误: {0}")]
    ArgumentParsing(#[from] clap::Error),

    /// 格式化错误
    #[error("格式化错误: {0}")]
    Formatting(#[from] std::fmt::Error),

    /// 配置文件错误
    #[error("配置文件错误: {0}")]
    ConfigFile(#[from] config::ConfigError),

    /// 通用错误
    #[error("内部错误: {message}")]
    Internal { message: String },

    /// 资源未找到
    #[error("资源未找到: {resource}")]
    NotFound { resource: String },

    /// 验证错误
    #[error("验证失败: {message}")]
    Validation { message: String },
}

impl AppError {
    /// 创建配置错误
    pub fn config(message: impl Into<String>) -> Self {
        Self::Config {
            message: message.into(),
        }
    }

    /// 创建学生服务错误
    pub fn student(message: impl Into<String>) -> Self {
        Self::Student {
            message: message.into(),
        }
    }

    /// 创建成绩服务错误
    pub fn grade(message: impl Into<String>) -> Self {
        Self::Grade {
            message: message.into(),
        }
    }

    /// 创建权限错误
    pub fn permission(message: impl Into<String>) -> Self {
        Self::Permission {
            message: message.into(),
        }
    }

    /// 创建命令执行错误
    pub fn command(message: impl Into<String>) -> Self {
        Self::Command {
            message: message.into(),
        }
    }

    /// 创建回复错误
    pub fn reply(message: impl Into<String>) -> Self {
        Self::Reply {
            message: message.into(),
        }
    }

    /// 创建内部错误
    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal {
            message: message.into(),
        }
    }

    /// 创建资源未找到错误
    pub fn not_found(resource: impl Into<String>) -> Self {
        Self::NotFound {
            resource: resource.into(),
        }
    }

    /// 创建验证错误
    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation {
            message: message.into(),
        }
    }
}

/// 应用结果类型别名
pub type AppResult<T> = Result<T, AppError>;

/// 从旧的错误类型转换的便利宏
#[macro_export]
macro_rules! app_error {
    ($kind:ident, $msg:expr) => {
        $crate::error::AppError::$kind($msg.into())
    };
}

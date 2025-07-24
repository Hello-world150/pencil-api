use serde::Serialize;
use std::error::Error;
use std::fmt;

/// 应用统一错误类型
#[derive(Debug)]
pub enum AppError {
    /// 用户相关错误
    User(String),
    /// 文集相关错误
    Collection(String),
    /// Hitokoto相关错误
    Hitokoto(String),
    /// 数据库/存储相关错误
    Storage(String),
    /// 文件IO错误
    Io(String),
    /// JSON序列化/反序列化错误
    Json(String),
    /// 验证错误
    Validation(String),
    /// 资源不存在错误
    NotFound(String),
    /// 资源已存在错误
    AlreadyExists(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::User(msg) => write!(f, "用户错误: {}", msg),
            AppError::Collection(msg) => write!(f, "文集错误: {}", msg),
            AppError::Hitokoto(msg) => write!(f, "Hitokoto错误: {}", msg),
            AppError::Storage(msg) => write!(f, "存储错误: {}", msg),
            AppError::Io(msg) => write!(f, "文件操作错误: {}", msg),
            AppError::Json(msg) => write!(f, "JSON格式错误: {}", msg),
            AppError::Validation(msg) => write!(f, "验证错误: {}", msg),
            AppError::NotFound(msg) => write!(f, "资源不存在: {}", msg),
            AppError::AlreadyExists(msg) => write!(f, "资源已存在: {}", msg),
        }
    }
}

impl Error for AppError {}

/// 用于API响应的错误结构
#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub code: String,
}

impl AppError {
    /// 转换为适合API响应的错误结构
    pub fn to_response(&self) -> ErrorResponse {
        let (error, code) = match self {
            AppError::User(msg) => (format!("用户错误: {}", msg), "USER_ERROR".to_string()),
            AppError::Collection(msg) => {
                (format!("文集错误: {}", msg), "COLLECTION_ERROR".to_string())
            }
            AppError::Hitokoto(msg) => (
                format!("Hitokoto错误: {}", msg),
                "HITOKOTO_ERROR".to_string(),
            ),
            AppError::Storage(msg) => (format!("存储错误: {}", msg), "STORAGE_ERROR".to_string()),
            AppError::Io(msg) => (format!("文件操作错误: {}", msg), "IO_ERROR".to_string()),
            AppError::Json(msg) => (format!("JSON格式错误: {}", msg), "JSON_ERROR".to_string()),
            AppError::Validation(msg) => {
                (format!("验证错误: {}", msg), "VALIDATION_ERROR".to_string())
            }
            AppError::NotFound(msg) => (format!("资源不存在: {}", msg), "NOT_FOUND".to_string()),
            AppError::AlreadyExists(msg) => {
                (format!("资源已存在: {}", msg), "ALREADY_EXISTS".to_string())
            }
        };

        ErrorResponse { error, code }
    }

    /// 获取HTTP状态码
    pub fn status_code(&self) -> rocket::http::Status {
        use rocket::http::Status;
        match self {
            AppError::NotFound(_) => Status::NotFound,
            AppError::AlreadyExists(_) => Status::Conflict,
            AppError::Validation(_) => Status::BadRequest,
            AppError::User(_) | AppError::Collection(_) | AppError::Hitokoto(_) => {
                Status::BadRequest
            }
            AppError::Storage(_) | AppError::Io(_) | AppError::Json(_) => {
                Status::InternalServerError
            }
        }
    }
}

/// 从std::io::Error转换
impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err.to_string())
    }
}

/// 从serde_json::Error转换
impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::Json(err.to_string())
    }
}

/// 从Box<dyn Error + Send + Sync>转换
impl From<Box<dyn Error + Send + Sync>> for AppError {
    fn from(err: Box<dyn Error + Send + Sync>) -> Self {
        AppError::Storage(err.to_string())
    }
}

/// 从String转换（用于简单的错误消息）
impl From<String> for AppError {
    fn from(msg: String) -> Self {
        AppError::Validation(msg)
    }
}

/// 从&str转换（用于简单的错误消息）
impl From<&str> for AppError {
    fn from(msg: &str) -> Self {
        AppError::Validation(msg.to_string())
    }
}

/// 应用结果类型别名
pub type AppResult<T> = Result<T, AppError>;

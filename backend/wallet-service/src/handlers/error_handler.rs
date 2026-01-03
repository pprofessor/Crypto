//! مدیریت خطاهای API
//!
//! این ماژول شامل تعریف انواع خطاها و تبدیل آنها به پاسخ‌های HTTP مناسب است

use actix_web::{HttpResponse, ResponseError};
use serde_json::json;
use thiserror::Error;
use tracing::error;

/// انواع خطاهای API
#[derive(Error, Debug)]
pub enum ApiError {
    /// خطای اعتبارسنجی ورودی (400)
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    /// خطای احراز هویت (401)
    #[error("Authentication required")]
    Unauthorized,
    
    /// خطای دسترسی (403)
    #[error("Access forbidden: {0}")]
    Forbidden(String),
    
    /// منبع یافت نشد (404)
    #[error("Resource not found: {0}")]
    NotFound(String),
    
    /// خطای سرور داخلی (500)
    #[error("Internal server error: {0}")]
    InternalError(String),
    
    /// سرویس در دسترس نیست (503)
    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),
    
    /// خطای دیتابیس
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

impl ApiError {
    /// ایجاد خطای اعتبارسنجی
    pub fn validation_error(message: &str) -> Self {
        ApiError::ValidationError(message.to_string())
    }
    
    /// ایجاد خطای عدم یافتن
    pub fn not_found(message: &str) -> Self {
        ApiError::NotFound(message.to_string())
    }
    
    /// ایجاد خطای داخلی
    pub fn internal_error(message: &str) -> Self {
        ApiError::InternalError(message.to_string())
    }
    
    /// ایجاد خطای سرویس غیرفعال
    pub fn service_unavailable(message: &str) -> Self {
        ApiError::ServiceUnavailable(message.to_string())
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        error!("API Error: {}", self);
        
        match self {
            ApiError::ValidationError(msg) => {
                HttpResponse::BadRequest().json(json!({
                    "error": "validation_error",
                    "message": msg,
                    "code": 400
                }))
            }
            
            ApiError::Unauthorized => {
                HttpResponse::Unauthorized().json(json!({
                    "error": "unauthorized",
                    "message": "Authentication required",
                    "code": 401
                }))
            }
            
            ApiError::Forbidden(msg) => {
                HttpResponse::Forbidden().json(json!({
                    "error": "forbidden",
                    "message": msg,
                    "code": 403
                }))
            }
            
            ApiError::NotFound(msg) => {
                HttpResponse::NotFound().json(json!({
                    "error": "not_found",
                    "message": msg,
                    "code": 404
                }))
            }
            
            ApiError::InternalError(msg) => {
                HttpResponse::InternalServerError().json(json!({
                    "error": "internal_error",
                    "message": msg,
                    "code": 500
                }))
            }
            
            ApiError::ServiceUnavailable(msg) => {
                HttpResponse::ServiceUnavailable().json(json!({
                    "error": "service_unavailable",
                    "message": msg,
                    "code": 503
                }))
            }
            
            ApiError::DatabaseError(e) => {
                // لاگ خطای دیتابیس با جزئیات کامل
                error!("Database error details: {:?}", e);
                
                // به کاربر جزئیات فنی نمایش نده
                HttpResponse::InternalServerError().json(json!({
                    "error": "database_error",
                    "message": "Database operation failed",
                    "code": 500
                }))
            }
        }
    }
    
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            ApiError::ValidationError(_) => actix_web::http::StatusCode::BAD_REQUEST,
            ApiError::Unauthorized => actix_web::http::StatusCode::UNAUTHORIZED,
            ApiError::Forbidden(_) => actix_web::http::StatusCode::FORBIDDEN,
            ApiError::NotFound(_) => actix_web::http::StatusCode::NOT_FOUND,
            ApiError::InternalError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::ServiceUnavailable(_) => actix_web::http::StatusCode::SERVICE_UNAVAILABLE,
            ApiError::DatabaseError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

/// ماکرو برای مدیریت آسان خطاها
#[macro_export]
macro_rules! api_try {
    () => {
        match  {
            Ok(val) => val,
            Err(e) => {
                tracing::error!("Operation failed: {:?}", e);
                return Err(crate::errors::ApiError::internal_error(&format!("Operation failed: {}", e)));
            }
        }
    };
    (, ) => {
        match  {
            Ok(val) => val,
            Err(e) => {
                tracing::error!("{}: {:?}", , e);
                return Err(crate::errors::ApiError::internal_error());
            }
        }
    };
}

/// ماکرو برای اعتبارسنجی
#[macro_export]
macro_rules! validate {
    (, ) => {
        if ! {
            return Err(crate::errors::ApiError::validation_error());
        }
    };
}

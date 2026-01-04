use actix_web::{ HttpResponse, ResponseError };
use serde_json::json;
use std::fmt;

/// شمارنده (enum) اصلی خطاهای سرویس کیف پول.
/// هر واریانت یک نوع خطای ممکن را نشان می‌دهد.
#[derive(Debug)]
pub enum ServiceError {
    /// خطای اعتبارسنجی داده‌های ورودی (مثلاً آدرس نامعتبر)
    ValidationError(String),
    /// منبع درخواست شده یافت نشد (مثلاً WalletNotFound, UserNotFound)
    NotFound(String),
    /// خطای مربوط به دیتابیس (اتصال، کوئری، و...)
    DatabaseError(sqlx::Error),
    /// خطای داخلی سرور (برای خطاهای غیرمنتظره)
    InternalServerError(String),
    /// دسترسی غیرمجاز (مثلاً کاربر سعی کند به کیف پول کاربر دیگر دسترسی پیدا کند)
    Forbidden(String),
    /// درخواست بد (مثلاً پارامترهای اشتباه)
    BadRequest(String),
}

// پیاده‌سازی نمایش متنی برای خطاها (برای لاگ کردن)
impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ServiceError::ValidationError(msg) => write!(f, "Validation Error: {}", msg),
            ServiceError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            ServiceError::DatabaseError(err) => write!(f, "Database Error: {}", err),
            ServiceError::InternalServerError(msg) => write!(f, "Internal Server Error: {}", msg),
            ServiceError::Forbidden(msg) => write!(f, "Forbidden: {}", msg),
            ServiceError::BadRequest(msg) => write!(f, "Bad Request: {}", msg),
        }
    }
}

// تبدیل خطاهای sqlx به ServiceError
impl From<sqlx::Error> for ServiceError {
    fn from(err: sqlx::Error) -> Self {
        // می‌توانید اینجا منطق دقیق‌تری برای تشخیص نوع خطای دیتابیس پیاده کنید
        ServiceError::DatabaseError(err)
    }
}

// پیاده‌سازی ResponseError برای Actix-web تا بتواند خطاها را به پاسخ HTTP تبدیل کند
impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::ValidationError(msg) => {
                HttpResponse::BadRequest().json(json!({ "error": msg }))
            }
            ServiceError::NotFound(msg) => {
                HttpResponse::NotFound().json(json!({ "error": msg }))
            }
            ServiceError::DatabaseError(_) =>
                HttpResponse::InternalServerError().json(
                    json!({ "error": "A database error occurred" })
                ),
            ServiceError::InternalServerError(msg) => {
                HttpResponse::InternalServerError().json(json!({ "error": msg }))
            }
            ServiceError::Forbidden(msg) => {
                HttpResponse::Forbidden().json(json!({ "error": msg }))
            }
            ServiceError::BadRequest(msg) => {
                HttpResponse::BadRequest().json(json!({ "error": msg }))
            }
        }
    }
}

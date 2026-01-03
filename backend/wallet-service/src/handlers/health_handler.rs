//! Handlerهای مربوط به سلامت سرویس (Health Checks)

use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use tracing::{info, error};
use chrono::Utc;

use crate::AppState;

/// بررسی سلامت سرویس و اتصال به دیتابیس
/// 
/// # Endpoint
/// GET /health
/// 
/// # Response
/// `json
/// {
///     "status": "healthy",
///     "timestamp": "2026-01-03T18:40:00Z",
///     "service": "wallet-service",
///     "version": "0.1.0",
///     "database": "connected",
///     "uptime": "5m30s"
/// }
/// `
pub async fn health_check(state: web::Data<AppState>) -> impl Responder {
    info!("Health check requested");
    
    // بررسی اتصال به دیتابیس
    let db_status = check_database_connection(&state.pool).await;
    
    // زمان uptime (در نسخه واقعی از process::idle_time استفاده می‌شود)
    let uptime = "5m30s"; // Mock - در production واقعی محاسبه شود
    
    let response = HealthResponse {
        status: if db_status { "healthy" } else { "unhealthy" },
        timestamp: Utc::now(),
        service: "wallet-service",
        version: env!("CARGO_PKG_VERSION"),
        database: if db_status { "connected" } else { "disconnected" },
        uptime: uptime.to_string(),
    };
    
    let status_code = if db_status {
        actix_web::http::StatusCode::OK
    } else {
        actix_web::http::StatusCode::SERVICE_UNAVAILABLE
    };
    
    HttpResponse::build(status_code).json(response)
}

/// بررسی اتصال به پایگاه داده
async fn check_database_connection(pool: &PgPool) -> bool {
    match sqlx::query("SELECT 1").execute(pool).await {
        Ok(_) => {
            info!("Database connection is healthy");
            true
        }
        Err(e) => {
            error!("Database connection failed: {}", e);
            false
        }
    }
}

/// بررسی سلامت عمیق سرویس (شامل وابستگی‌های خارجی)
/// 
/// # Endpoint  
/// GET /health/deep
pub async fn deep_health_check(state: web::Data<AppState>) -> impl Responder {
    info!("Deep health check requested");
    
    let mut checks = Vec::new();
    
    // بررسی دیتابیس
    let db_ok = check_database_connection(&state.pool).await;
    checks.push(HealthCheck {
        name: "database".to_string(),
        status: if db_ok { "healthy" } else { "unhealthy" }.to_string(),
        duration_ms: 0, // در نسخه واقعی اندازه‌گیری شود
    });
    
    // بررسی Redis (اگر در آینده اضافه شود)
    checks.push(HealthCheck {
        name: "redis".to_string(),
        status: "not_implemented".to_string(),
        duration_ms: 0,
    });
    
    // بررسی TRON node (اگر در آینده اضافه شود)
    checks.push(HealthCheck {
        name: "tron_node".to_string(),
        status: "not_implemented".to_string(),
        duration_ms: 0,
    });
    
    let all_healthy = checks.iter().all(|c| c.status == "healthy");
    
    let response = DeepHealthResponse {
        status: if all_healthy { "healthy" } else { "degraded" },
        timestamp: Utc::now(),
        service: "wallet-service",
        checks,
        total_checks: checks.len() as u32,
        successful_checks: checks.iter().filter(|c| c.status == "healthy").count() as u32,
    };
    
    let status_code = if all_healthy {
        actix_web::http::StatusCode::OK
    } else {
        actix_web::http::StatusCode::SERVICE_UNAVAILABLE
    };
    
    HttpResponse::build(status_code).json(response)
}

/// پاسخ سلامت ساده
#[derive(serde::Serialize)]
struct HealthResponse {
    status: &'static str,
    timestamp: chrono::DateTime<chrono::Utc>,
    service: &'static str,
    version: &'static str,
    database: &'static str,
    uptime: String,
}

/// پاسخ سلامت عمیق
#[derive(serde::Serialize)]
struct DeepHealthResponse {
    status: &'static str,
    timestamp: chrono::DateTime<chrono::Utc>,
    service: &'static str,
    checks: Vec<HealthCheck>,
    total_checks: u32,
    successful_checks: u32,
}

/// ساختار بررسی سلامت
#[derive(serde::Serialize)]
struct HealthCheck {
    name: String,
    status: String,
    duration_ms: u64,
}

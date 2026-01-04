use crate::models::CreateWalletRequest;
use crate::services::WalletService;
use actix_web::{ web, HttpResponse, Responder };
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

/// هندلر برای ایجاد یک کیف پول جدید (POST /wallets)
///
/// # درخواست (Request Body)
/// ```json
/// {
///   "user_id": "uuid-string",
///   "currency_symbol": "BTC",
///   "public_address": "optional-address"
/// }
/// ```
///
/// # پاسخ (Response)
/// * `201 Created` - کیف پول با موفقیت ایجاد شد. بدنه پاسخ شامل کیف پول ایجاد شده است.
/// * `400 Bad Request` - داده‌های ورودی نامعتبر یا کیف پول تکراری.
/// * `403 Forbidden` - کاربر سعی کرده برای کاربر دیگری کیف پول ایجاد کند.
/// * `500 Internal Server Error` - خطای سرور.
use actix_web::ResponseError;
pub async fn create_wallet(
    // کاربر احراز هویت شده (شناسه از توکن JWT استخراج می‌شود). فعلاً از path دریافت می‌شود.
    user_id: web::Path<Uuid>,
    request: web::Json<CreateWalletRequest>,
    wallet_service: web::Data<WalletService>,
    db_pool: web::Data<PgPool>
) -> impl Responder {
    let user_id = user_id.into_inner();
    let request = request.into_inner();

    match wallet_service.create_wallet(user_id, request, db_pool.get_ref()).await {
        Ok(wallet) => HttpResponse::Created().json(wallet),
        Err(e) => {
            // خطا توسط `ServiceError` خودش به پاسخ HTTP مناسب تبدیل می‌شود.
            e.error_response()
        }
    }
}

/// هندلر برای دریافت لیست تمام کیف پول‌های یک کاربر (GET /users/{user_id}/wallets)
pub async fn get_user_wallets(
    user_id: web::Path<Uuid>,
    wallet_service: web::Data<WalletService>
) -> impl Responder {
    let user_id = user_id.into_inner();

    match wallet_service.get_user_wallets(user_id).await {
        Ok(wallets) => HttpResponse::Ok().json(wallets),
        Err(e) => e.error_response(),
    }
}

/// هندلر برای دریافت اطلاعات یک کیف پول خاص (GET /wallets/{wallet_id})
/// مالکیت کیف پول توسط سرویس بررسی می‌شود.
pub async fn get_wallet(
    path: web::Path<(Uuid, Uuid)>, // (user_id, wallet_id)
    wallet_service: web::Data<WalletService>
) -> impl Responder {
    let (user_id, wallet_id) = path.into_inner();

    match wallet_service.get_wallet_by_id(wallet_id, user_id).await {
        Ok(wallet) => HttpResponse::Ok().json(wallet),
        Err(e) => e.error_response(),
    }
}

/// هندلر سلامت سرویس (health check) - GET /health
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(json!({ "status": "ok", "service": "wallet-service" }))
}

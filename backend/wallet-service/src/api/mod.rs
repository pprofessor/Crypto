//! ماژول API (لایه کنترلر).
//! این ماژول endpointهای HTTP و routing آنها را تعریف می‌کند.
//! هر هندلر، درخواست HTTP را دریافت، پردازش اولیه کرده و به سرویس مربوطه تحویل می‌دهد.

mod wallet_handlers;

use actix_web::web;

/// تمام routeهای مربوط به سرویس کیف پول را در یک scope (`/api`) گروه‌بندی و پیکربندی می‌کند.
///
/// # مسیرها (Routes)
/// * `POST   /users/{user_id}/wallets` - ایجاد کیف پول جدید (`wallet_handlers::create_wallet`)
/// * `GET    /users/{user_id}/wallets` - دریافت لیست کیف پول‌های یک کاربر (`wallet_handlers::get_user_wallets`)
/// * `GET    /users/{user_id}/wallets/{wallet_id}` - دریافت اطلاعات یک کیف پول خاص (`wallet_handlers::get_wallet`)
/// * `GET    /health` - بررسی سلامت سرویس (`wallet_handlers::health_check`)
///
/// # نکات
/// * الگوی مسیر `{user_id}` تضمین می‌کند که کاربر فقط به منابع خود دسترسی دارد.
/// * scope `/api` باعث می‌شود همه endpointها با پیشوند `/api` قابل دسترسی باشند (مثلاً `http://localhost:8080/api/health`).
pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web
            ::scope("/api")
            // سلامت سرویس
            .route("/health", web::get().to(wallet_handlers::health_check))
            // ایجاد کیف پول برای کاربر خاص
            .route("/users/{user_id}/wallets", web::post().to(wallet_handlers::create_wallet))
            // دریافت لیست کیف پول‌های کاربر خاص
            .route("/users/{user_id}/wallets", web::get().to(wallet_handlers::get_user_wallets))
            // دریافت یک کیف پول خاص کاربر
            .route(
                "/users/{user_id}/wallets/{wallet_id}",
                web::get().to(wallet_handlers::get_wallet)
            )
    );
}

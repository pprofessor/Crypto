use crate::config::Config;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;
use tracing::info;

/// یک connection pool به دیتابیس PostgreSQL ایجاد و برمی‌گرداند.
///
/// # آرگومان‌ها
/// * `config` - پیکربندی سرویس که شامل تنظیمات دیتابیس است.
///
/// # مقدار بازگشتی
/// * `Result<PgPool, sqlx::Error>` - در صورت موفقیت، پول اتصال. در صورت خطا، خطای مربوطه.
///
/// # نکات
/// * از `PgPoolOptions` برای تنظیمات پیشرفته pool مانند حداکثر connection و timeout استفاده می‌کند.
/// * URL دیتابیس از متغیر محیطی `DATABASE_URL` که در `Config` لود شده، خوانده می‌شود.
pub async fn create_pool(config: &Config) -> Result<PgPool, sqlx::Error> {
    info!("Creating database connection pool...");

    let pool = PgPoolOptions::new()
        // حداکثر تعداد connection‌های همزمان در pool
        .max_connections(20)
        // حداقل زمان نگهداری یک connection در pool قبل از بسته شدن
        .min_connections(5)
        // حداکثر زمانی که یک connection می‌تواند idle بماند
        .idle_timeout(Some(Duration::from_secs(300)))
        // زمان انتظار برای گرفتن یک connection از pool
        .acquire_timeout(Duration::from_secs(10))
        // تست connection‌ها قبل از دادن به درخواست‌ها برای اطمینان از سلامت
        .test_before_acquire(true)
        .connect(&config.database_url)
        .await?;

    info!("Database connection pool created successfully.");
    Ok(pool)
}

/// یک تابع کمکی برای اجرای یک مهاجرت ساده (migration) و اطمینان از وجود schema و جداول.
/// در این مرحله، فرض می‌کنیم جداول از قبل توسط سرویس دیگر (مثلاً init-db) ایجاد شده‌اند.
/// این تابع می‌تواند برای بررسی سالم بودن اتصال و وجود جدول مورد نظر استفاده شود.
pub async fn check_database(pool: &PgPool) -> Result<(), sqlx::Error> {
    info!("Checking database connection and essential table...");
    // یک کوئری ساده برای بررسی وجود جدول wallets در schema crypto اجرا می‌کنیم.
    let _result: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM crypto.wallets LIMIT 1")
        .fetch_one(pool)
        .await?;
    info!("Database check passed. Table 'crypto.wallets' exists.");
    Ok(())
}
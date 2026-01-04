use config::Config as ConfigBuilder;
use serde::Deserialize;
use std::env;

/// ساختار اصلی نگهداری تنظیمات (Configuration) میکروسرویس.
/// فیلدها از متغیرهای محیطی یا فایل `.env` بارگذاری می‌شوند.
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// آدرس اتصال به دیتابیس PostgreSQL (مثال: postgres://user:pass@localhost:5432/crypto)
    pub database_url: String,
    /// پورتی که سرور HTTP روی آن گوش می‌دهد.
    pub server_port: u16,
    /// آدرس host که سرور HTTP به آن bind می‌شود.
    pub server_host: String,
}

impl Config {
    /// یک نمونه جدید از `Config` را با بارگذاری تنظیمات از متغیرهای محیطی ایجاد می‌کند.
    ///
    /// # ترتیب بارگذاری (اولویت از بالا به پایین):
    /// 1. متغیرهای محیطی سیستم
    /// 2. فایل `.env` در ریشه پروژه (اگر وجود داشته باشد)
    ///
    /// # خطاها
    /// در صورت عدم وجود متغیرهای ضروری، خطا برمی‌گرداند.
    pub fn new() -> Result<Self, config::ConfigError> {
        let env_file = env::var("WALLET_ENV_FILE").unwrap_or_else(|_| ".env".to_string());

        let config = ConfigBuilder::builder()
            // شروع با فایل `.env` (اختیاری)
            .add_source(config::File::with_name(&env_file).required(false))
            // اضافه کردن متغیرهای محیطی با پیشوند `WALLET_`
            .add_source(config::Environment::with_prefix("WALLET").separator("__"))
            .build()?;

        config.try_deserialize()
    }

    /// یک تابع کمکی برای گرفتن آدرس کامل (host:port) که سرور باید روی آن اجرا شود.
    pub fn server_address(&self) -> String {
        format!("{}:{}", self.server_host, self.server_port)
    }
}

// پیاده‌سازی `Default` برای استفاده در تست‌ها یا موارد توسعه.
impl Default for Config {
    fn default() -> Self {
        Self {
            database_url: "postgres://postgres:postgres@localhost:5432/crypto".to_string(),
            server_port: 8080,
            server_host: "0.0.0.0".to_string(),
        }
    }
}

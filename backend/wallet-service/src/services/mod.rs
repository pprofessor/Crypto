//! ماژول سرویس‌ها (Services).
//! این ماژول منطق کسب‌وکار (business logic) برنامه را کپسوله می‌کند.
//! سرویس‌ها از ریپازیتوری‌ها استفاده کرده و قوانین حوزه (domain rules)، اعتبارسنجی‌ها و جریان‌های کاری را مدیریت می‌کنند.

mod wallet_service;

// اکسپورت عمومی (public) ساختارها و توابع تعریف شده در فایل wallet_service.rs
pub use wallet_service::WalletService;
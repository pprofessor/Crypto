//! ماژول مدل‌های داده (Data Models).
//! این ماژول ساختارهای دادهای منطبق با جداول دیتابیس و درخواست/پاسخ‌های API را تعریف می‌کند.

mod wallet;
// اکسپورت عمومی (public) ساختارهای تعریف شده در فایل wallet.rs
pub use wallet::{CreateWalletRequest, Wallet};
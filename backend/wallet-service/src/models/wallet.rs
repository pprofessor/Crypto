use chrono::{ DateTime, Utc };
use serde::{ Deserialize, Serialize };
use sqlx::FromRow;
use uuid::Uuid;

/// مدل اصلی داده‌های کیف پول، مطابق با جدول `crypto.wallets` در دیتابیس.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Wallet {
    pub id: Uuid,
    pub user_id: Uuid,
    pub currency_symbol: String,
    pub public_address: String,
    pub balance: i64, // تغییر به i64 (واحد: کوچکترین جزء ارز، مثلاً Satoshi)
    pub locked_balance: i64, // تغییر به i64
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// مدل برای ایجاد یک کیف پول جدید
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWalletRequest {
    pub user_id: Uuid,
    pub currency_symbol: String,
    pub public_address: Option<String>,
}

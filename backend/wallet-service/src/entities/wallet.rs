use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use bigdecimal::BigDecimal;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Wallet {
    /// شناسه منحصر به فرد کیف پول (UUID v4)
    pub id: Uuid,
    
    /// شناسه کاربر صاحب کیف پول (ارجاع به جدول users)
    pub user_id: Uuid,
    
    /// آدرس TRON کیف پول (با 'T' شروع می‌شود)
    #[sqlx(default)]
    pub tron_address: Option<String>,
    
    /// موجودی USDT با دقت ۸ رقم اعشار
    #[sqlx(default = "default_balance")]
    pub usdt_balance: BigDecimal,
    
    /// زمان ایجاد کیف پول
    pub created_at: DateTime<Utc>,
    
    /// زمان آخرین به‌روزرسانی
    pub updated_at: DateTime<Utc>,
}

fn default_balance() -> BigDecimal {
    BigDecimal::from(0)
}

impl Wallet {
    /// ایجاد یک کیف پول جدید با مقادیر پیش‌فرض
    pub fn new(user_id: Uuid) -> Self {
        let now = Utc::now();
        
        Self {
            id: Uuid::new_v4(),
            user_id,
            tron_address: None,
            usdt_balance: BigDecimal::from(0),
            created_at: now,
            updated_at: now,
        }
    }
    
    /// بررسی اینکه کیف پول فعال است یا خیر
    pub fn is_active(&self) -> bool {
        self.tron_address.is_some()
    }
    
    /// افزایش موجودی کیف پول
    pub fn deposit(&mut self, amount: BigDecimal) -> Result<(), String> {
        if amount <= BigDecimal::from(0) {
            return Err("مقدار واریز باید بزرگتر از صفر باشد".to_string());
        }
        
        self.usdt_balance += amount;
        self.updated_at = Utc::now();
        Ok(())
    }
    
    /// کاهش موجودی کیف پول
    pub fn withdraw(&mut self, amount: BigDecimal) -> Result<(), String> {
        if amount <= BigDecimal::from(0) {
            return Err("مقدار برداشت باید بزرگتر از صفر باشد".to_string());
        }
        
        if &self.usdt_balance < &amount {
            return Err("موجودی کافی نیست".to_string());
        }
        
        self.usdt_balance -= amount;
        self.updated_at = Utc::now();
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateWalletRequest {
    /// شناسه کاربر برای ایجاد کیف پول
    pub user_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WalletResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub tron_address: Option<String>,
    pub usdt_balance: String,
    pub created_at: DateTime<Utc>,
    pub is_active: bool,
}

impl From<Wallet> for WalletResponse {
    fn from(wallet: Wallet) -> Self {
        Self {
            id: wallet.id,
            user_id: wallet.user_id,
            tron_address: wallet.tron_address,
            usdt_balance: wallet.usdt_balance.to_string(),
            created_at: wallet.created_at,
            is_active: wallet.tron_address.is_some(),
        }
    }
}

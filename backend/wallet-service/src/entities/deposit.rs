use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use bigdecimal::BigDecimal;
use std::str::FromStr;

/// وضعیت‌های ممکن برای یک واریز
#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "deposit_status", rename_all = "snake_case")]
pub enum DepositStatus {
    /// در انتظار واریز
    Pending,
    /// در حال پردازش
    Processing,
    /// تأیید شده
    Confirmed,
    /// ناموفق
    Failed,
    /// منقضی شده
    Expired,
    /// لغو شده
    Cancelled,
}

/// روش‌های پرداخت پشتیبانی شده
#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "payment_method", rename_all = "snake_case")]
pub enum PaymentMethod {
    /// USDT روی شبکه TRON
    TronUsdt,
    /// Bitcoin
    Bitcoin,
    /// Ethereum
    Ethereum,
    /// انتقال بانکی (آینده)
    BankTransfer,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Deposit {
    /// شناسه منحصر به فرد واریز
    pub id: Uuid,
    
    /// شناسه کاربر درخواست‌دهنده
    pub user_id: Uuid,
    
    /// مبلغ درخواستی برای واریز
    pub amount: BigDecimal,
    
    /// مبلغ واقعی دریافت شده (در صورت متفاوت بودن)
    #[sqlx(default)]
    pub received_amount: Option<BigDecimal>,
    
    /// هش تراکنش بلاکچین
    #[sqlx(default)]
    pub transaction_hash: Option<String>,
    
    /// آدرس واریز (کیف پول موقت ما)
    pub deposit_address: String,
    
    /// روش پرداخت انتخاب شده
    pub payment_method: PaymentMethod,
    
    /// وضعیت فعلی واریز
    pub status: DepositStatus,
    
    /// بلوک تأیید (برای بلاکچین)
    #[sqlx(default)]
    pub confirmation_block: Option<i64>,
    
    /// تعداد تأییدیه‌های دریافتی
    #[sqlx(default = "default_confirmations")]
    pub confirmations: i32,
    
    /// تعداد تأییدیه‌های مورد نیاز
    #[sqlx(default = "default_required_confirmations")]
    pub required_confirmations: i32,
    
    /// زمان تأیید نهایی
    #[sqlx(default)]
    pub confirmed_at: Option<DateTime<Utc>>,
    
    /// زمان انقضای درخواست واریز
    pub expires_at: DateTime<Utc>,
    
    /// زمان ایجاد درخواست
    pub created_at: DateTime<Utc>,
    
    /// زمان آخرین به‌روزرسانی
    pub updated_at: DateTime<Utc>,
}

fn default_confirmations() -> i32 { 0 }
fn default_required_confirmations() -> i32 { 12 }

impl Deposit {
    /// ایجاد یک درخواست واریز جدید
    pub fn new(
        user_id: Uuid,
        amount: BigDecimal,
        deposit_address: String,
        payment_method: PaymentMethod,
        expires_in_hours: i64,
    ) -> Result<Self, String> {
        // اعتبارسنجی مبلغ
        if amount <= BigDecimal::from_str("0.00000001").unwrap() {
            return Err("مبلغ واریز باید بزرگتر از 0.00000001 باشد".to_string());
        }
        
        if amount > BigDecimal::from_str("1000000").unwrap() {
            return Err("مبلغ واریز نمی‌تواند بیشتر از 1,000,000 باشد".to_string());
        }
        
        let now = Utc::now();
        let expires_at = now + chrono::Duration::hours(expires_in_hours);
        
        Ok(Self {
            id: Uuid::new_v4(),
            user_id,
            amount,
            received_amount: None,
            transaction_hash: None,
            deposit_address,
            payment_method,
            status: DepositStatus::Pending,
            confirmation_block: None,
            confirmations: 0,
            required_confirmations: 12,
            confirmed_at: None,
            expires_at,
            created_at: now,
            updated_at: now,
        })
    }
    
    /// بررسی اینکه آیا واریز منقضی شده است
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }
    
    /// بررسی اینکه آیا واریز تأیید شده است
    pub fn is_confirmed(&self) -> bool {
        matches!(self.status, DepositStatus::Confirmed)
    }
    
    /// افزایش تعداد تأییدیه‌ها
    pub fn add_confirmation(&mut self, block_height: i64) {
        self.confirmations += 1;
        self.confirmation_block = Some(block_height);
        self.updated_at = Utc::now();
        
        // اگر به تعداد کافی تأییدیه رسید، وضعیت را تغییر بده
        if self.confirmations >= self.required_confirmations {
            self.status = DepositStatus::Confirmed;
            self.confirmed_at = Some(Utc::now());
        }
    }
    
    /// تغییر وضعیت واریز
    pub fn update_status(&mut self, new_status: DepositStatus) {
        self.status = new_status;
        self.updated_at = Utc::now();
        
        // اگر وضعیت به تأیید شده تغییر کرد، زمان تأیید را ثبت کن
        if matches!(new_status, DepositStatus::Confirmed) && self.confirmed_at.is_none() {
            self.confirmed_at = Some(Utc::now());
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDepositRequest {
    /// شناسه کاربر
    pub user_id: Uuid,
    
    /// مبلغ واریز (به صورت رشته برای دقت)
    pub amount: String,
    
    /// روش پرداخت
    pub payment_method: PaymentMethod,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DepositResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub amount: String,
    pub received_amount: Option<String>,
    pub deposit_address: String,
    pub payment_method: PaymentMethod,
    pub status: DepositStatus,
    pub confirmations: i32,
    pub required_confirmations: i32,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub is_expired: bool,
    pub is_confirmed: bool,
}

impl From<Deposit> for DepositResponse {
    fn from(deposit: Deposit) -> Self {
        Self {
            id: deposit.id,
            user_id: deposit.user_id,
            amount: deposit.amount.to_string(),
            received_amount: deposit.received_amount.map(|amt| amt.to_string()),
            deposit_address: deposit.deposit_address,
            payment_method: deposit.payment_method,
            status: deposit.status,
            confirmations: deposit.confirmations,
            required_confirmations: deposit.required_confirmations,
            expires_at: deposit.expires_at,
            created_at: deposit.created_at,
            is_expired: deposit.is_expired(),
            is_confirmed: deposit.is_confirmed(),
        }
    }
}

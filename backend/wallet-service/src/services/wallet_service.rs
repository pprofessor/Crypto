use crate::errors::ServiceError;
use crate::models::{ CreateWalletRequest, Wallet };
use crate::repositories::WalletRepository;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

/// سرویس اصلی برای مدیریت عملیات کیف پول.
/// این سرویس منطق کسب‌وکار را کپسوله کرده و از ریپازیتوری برای تعامل با دیتابیس استفاده می‌کند.
#[derive(Clone)]
pub struct WalletService {
    repository: Arc<WalletRepository>,
}

impl WalletService {
    /// یک نمونه جدید از `WalletService` می‌سازد.
    pub fn new(repository: WalletRepository) -> Self {
        Self {
            repository: Arc::new(repository),
        }
    }

    /// یک کیف پول جدید برای کاربر ایجاد می‌کند.
    /// این متد اعتبارسنجی‌های لازم (مانند عدم وجود کیف پول تکراری برای همان ارز) را انجام می‌دهد.
    ///
    /// # آرگومان‌ها
    /// * `user_id` - شناسه کاربر درخواست‌دهنده
    /// * `request` - داده‌های مورد نیاز برای ایجاد کیف پول
    /// * `pool` - connection pool دیتابیس (برای شروع تراکنش)
    ///
    /// # خطاها
    /// * `ServiceError::BadRequest` - اگر کاربر از قبل برای این ارز کیف پول داشته باشد.
    /// * `ServiceError::ValidationError` - اگر داده‌های درخواست معتبر نباشند.
    /// * `ServiceError::DatabaseError` - در صورت بروز خطا در دیتابیس.
    pub async fn create_wallet(
        &self,
        user_id: Uuid,
        request: CreateWalletRequest,
        pool: &PgPool
    ) -> Result<Wallet, ServiceError> {
        // اعتبارسنجی اولیه: کاربر فقط می‌تواند برای خودش کیف پول ایجاد کند.
        if user_id != request.user_id {
            return Err(
                ServiceError::Forbidden("Cannot create wallet for another user".to_string())
            );
        }

        // اعتبارسنجی: کاربر نباید از قبل برای این ارز خاص کیف پول فعال داشته باشد.
        let exists = self.repository.exists_by_user_and_currency(
            user_id,
            &request.currency_symbol
        ).await?;

        if exists {
            return Err(
                ServiceError::BadRequest(
                    format!("User already has a wallet for currency {}", request.currency_symbol)
                )
            );
        }

        // اعتبارسنجی نماد ارز (می‌تواند در آینده با لیست ثابتی از ارزهای پشتیبانی شده چک شود).
        // فعلاً فقط یک چک ساده غیرخالی بودن.
        if request.currency_symbol.trim().is_empty() {
            return Err(
                ServiceError::ValidationError("Currency symbol cannot be empty".to_string())
            );
        }

        // شروع یک تراکنش دیتابیس برای اطمینان از原子یک بودن عملیات.
        let mut tx = pool.begin().await.map_err(ServiceError::from)?;

        // ایجاد کیف پول در دیتابیس (در داخل تراکنش).
        let new_wallet = self.repository.create(Some(&mut tx), &request).await?;

        // commit تراکنش در صورت موفقیت.
        tx.commit().await.map_err(ServiceError::from)?;

        Ok(new_wallet)
    }

    /// تمام کیف پول‌های یک کاربر را برمی‌گرداند.
    pub async fn get_user_wallets(&self, user_id: Uuid) -> Result<Vec<Wallet>, ServiceError> {
        self.repository.find_by_user_id(user_id).await
    }

    /// یک کیف پول خاص را بر اساس شناسه آن برمی‌گرداند.
    /// بررسی می‌کند که کیف پول متعلق به کاربر درخواست‌دهنده باشد (برای امنیت).
    pub async fn get_wallet_by_id(
        &self,
        wallet_id: Uuid,
        requesting_user_id: Uuid
    ) -> Result<Wallet, ServiceError> {
        // نوع نتیجه را به صورت واضح مشخص می‌کنیم: Option<Wallet>
        let wallet_opt: Option<Wallet> = self.repository.find_by_id(wallet_id).await?;

        // اگر کیف پول پیدا نشد، خطای Not Found برگردان
        let wallet = wallet_opt.ok_or_else(||
            ServiceError::NotFound("Wallet not found".to_string())
        )?;

        // بررسی مالکیت: فقط مالک کیف پول می‌تواند آن را ببیند.
        if wallet.user_id != requesting_user_id {
            return Err(ServiceError::Forbidden("Access to this wallet is denied".to_string()));
        }

        Ok(wallet)
    }
}

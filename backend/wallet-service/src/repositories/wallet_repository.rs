use crate::errors::ServiceError;
use crate::models::{ CreateWalletRequest, Wallet };
use async_trait::async_trait;
use sqlx::{ PgPool, Postgres, Transaction };
use uuid::Uuid;

/// `Repository` برای موجودیت `Wallet`.
/// تمام عملیات دیتابیس مرتبط با کیف پول‌ها در اینجا کپسوله شده است.
/// این یک `struct` است که یک connection pool به PostgreSQL را نگه می‌دارد.
#[derive(Clone)]
pub struct WalletRepository {
    pool: PgPool,
}

impl WalletRepository {
    /// سازنده (constructor) جدید. یک نمونه از `WalletRepository` با connection pool داده شده می‌سازد.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// یک کیف پول جدید در دیتابیس ایجاد می‌کند.
    /// توجه: در حال حاضر، فیلد `public_address` می‌تواند `None` باشد. منطق تولید آدرس بعداً اضافه خواهد شد.
    pub async fn create(
        &self,
        tx: Option<&mut Transaction<'_, Postgres>>,
        request: &CreateWalletRequest
    ) -> Result<Wallet, ServiceError> {
        let wallet_id = Uuid::new_v4();
        let now = chrono::Utc::now();
        // مقدار پیش‌فرض برای بالانس‌ها صفر است.
        let zero_balance = rust_decimal_macros::dec!(0);
        // اگر آدرس ارائه نشده، فعلاً `NULL` در دیتابیس ذخیره می‌شود.
        let public_address = request.public_address.as_deref();

        let query = sqlx
            ::query_as::<_, Wallet>(
                r#"
            INSERT INTO crypto.wallets 
                (id, user_id, currency_symbol, public_address, balance, locked_balance, status, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING *
            "#
            )
            .bind(wallet_id)
            .bind(request.user_id)
            .bind(&request.currency_symbol)
            .bind(public_address)
            .bind(zero_balance)
            .bind(zero_balance)
            .bind("active") // وضعیت پیش‌فرض
            .bind(now);

        // اجرای کوئری داخل تراکنیه ارائه شده، یا مستقیماً روی pool
        let result = if let Some(transaction) = tx {
            query.fetch_one(&mut **transaction).await
        } else {
            query.fetch_one(&self.pool).await
        };

        result.map_err(ServiceError::from)
    }

    /// یک کیف پول را بر اساس شناسه یکتای آن پیدا می‌کند.
    pub async fn find_by_id(&self, wallet_id: Uuid) -> Result<Option<Wallet>, ServiceError> {
        let query = sqlx
            ::query_as::<_, Wallet>(
                r#"
            SELECT * FROM crypto.wallets 
            WHERE id = $1
            "#
            )
            .bind(wallet_id);

        query.fetch_optional(&self.pool).await.map_err(ServiceError::from)
    }

    /// تمام کیف پول‌های متعلق به یک کاربر خاص را پیدا می‌کند.
    pub async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<Wallet>, ServiceError> {
        let query = sqlx
            ::query_as::<_, Wallet>(
                r#"
            SELECT * FROM crypto.wallets 
            WHERE user_id = $1
            ORDER BY created_at DESC
            "#
            )
            .bind(user_id);

        query.fetch_all(&self.pool).await.map_err(ServiceError::from)
    }

    /// بررسی می‌کند که آیا کاربر قبلاً یک کیف پول برای یک ارز خاص دارد یا نه.
    /// این برای جلوگیری از ایجاد کیف پول تکراری لازم است.
    pub async fn exists_by_user_and_currency(
        &self,
        user_id: Uuid,
        currency_symbol: &str
    ) -> Result<bool, ServiceError> {
        let query = sqlx
            ::query(
                r#"
            SELECT EXISTS(
                SELECT 1 FROM crypto.wallets 
                WHERE user_id = $1 AND currency_symbol = $2
            )
            "#
            )
            .bind(user_id)
            .bind(currency_symbol);

        let exists: (bool,) = query.fetch_one(&self.pool).await.map_err(ServiceError::from)?;

        Ok(exists.0)
    }
}

// پیاده‌سازی `async_trait` برای `Repository`. فعلاً بدون متد اضافی.
#[async_trait]
impl crate::repositories::Repository for WalletRepository {}

//! ماژول ریپازیتوری‌ها (Repositories).

use crate::errors::ServiceError;
use crate::models::{ CreateWalletRequest, Wallet };
use async_trait::async_trait;
use sqlx::{ PgPool, Postgres, Transaction };
use uuid::Uuid;

/// `Repository` برای موجودیت `Wallet`.
#[derive(Clone)]
pub struct WalletRepository {
    pool: PgPool,
}

impl WalletRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        tx: Option<&mut Transaction<'_, Postgres>>,
        request: &CreateWalletRequest
    ) -> Result<Wallet, ServiceError> {
        let wallet_id = Uuid::new_v4();
        let now = chrono::Utc::now();
        let zero_balance: i64 = 0; // تغییر اینجا
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
            .bind("active")
            .bind(now);

        let result = if let Some(transaction) = tx {
            query.fetch_one(&mut **transaction).await
        } else {
            query.fetch_one(&self.pool).await
        };

        result.map_err(ServiceError::from)
    }

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

    pub async fn exists_by_user_and_currency(
        &self,
        user_id: Uuid,
        currency_symbol: &str
    ) -> Result<bool, ServiceError> {
        let exists: bool = sqlx
            ::query_scalar(
                r#"
            SELECT EXISTS(
                SELECT 1 FROM crypto.wallets 
                WHERE user_id = $1 AND currency_symbol = $2
            )
            "#
            )
            .bind(user_id)
            .bind(currency_symbol)
            .fetch_one(&self.pool).await
            .map_err(ServiceError::from)?;

        Ok(exists)
    }
}

pub trait Repository {}

#[async_trait]
impl Repository for WalletRepository {}

//! Repository pattern برای دسترسی به پایگاه داده
//! 
//! این ماژول شامل توابعی برای تعامل با جداول دیتابیس است

use sqlx::{PgPool, Error};
use uuid::Uuid;
use chrono::Utc;
use bigdecimal::BigDecimal;

use crate::entities::{wallet::Wallet, deposit::{Deposit, DepositStatus, PaymentMethod}};

/// Repository برای مدیریت عملیات کیف پول
pub struct WalletRepository {
    pool: PgPool,
}

impl WalletRepository {
    /// ایجاد یک نمونه جدید از WalletRepository
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    
    /// ایجاد یک کیف پول جدید برای کاربر
    pub async fn create_wallet(&self, user_id: Uuid) -> Result<Wallet, Error> {
        let wallet = Wallet::new(user_id);
        
        let result = sqlx::query!(
            r#"
            INSERT INTO wallets (id, user_id, tron_address, usdt_balance, created_at, updated_at)
            VALUES (, , , , , )
            RETURNING *
            "#,
            wallet.id,
            wallet.user_id,
            wallet.tron_address,
            wallet.usdt_balance,
            wallet.created_at,
            wallet.updated_at
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(Wallet {
            id: result.id,
            user_id: result.user_id,
            tron_address: result.tron_address,
            usdt_balance: result.usdt_balance,
            created_at: result.created_at,
            updated_at: result.updated_at,
        })
    }
    
    /// دریافت کیف پول بر اساس شناسه کاربر
    pub async fn get_wallet_by_user_id(&self, user_id: Uuid) -> Result<Option<Wallet>, Error> {
        let result = sqlx::query!(
            r#"
            SELECT * FROM wallets WHERE user_id = 
            "#,
            user_id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        match result {
            Some(record) => Ok(Some(Wallet {
                id: record.id,
                user_id: record.user_id,
                tron_address: record.tron_address,
                usdt_balance: record.usdt_balance,
                created_at: record.created_at,
                updated_at: record.updated_at,
            })),
            None => Ok(None),
        }
    }
    
    /// بروزرسانی موجودی کیف پول
    pub async fn update_wallet_balance(
        &self, 
        wallet_id: Uuid, 
        new_balance: BigDecimal
    ) -> Result<Wallet, Error> {
        let updated_at = Utc::now();
        
        let result = sqlx::query!(
            r#"
            UPDATE wallets 
            SET usdt_balance = , updated_at = 
            WHERE id = 
            RETURNING *
            "#,
            new_balance,
            updated_at,
            wallet_id
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(Wallet {
            id: result.id,
            user_id: result.user_id,
            tron_address: result.tron_address,
            usdt_balance: result.usdt_balance,
            created_at: result.created_at,
            updated_at: result.updated_at,
        })
    }
    
    /// تنظیم آدرس TRON برای کیف پول
    pub async fn set_tron_address(
        &self, 
        wallet_id: Uuid, 
        tron_address: String
    ) -> Result<Wallet, Error> {
        let updated_at = Utc::now();
        
        let result = sqlx::query!(
            r#"
            UPDATE wallets 
            SET tron_address = , updated_at = 
            WHERE id = 
            RETURNING *
            "#,
            tron_address,
            updated_at,
            wallet_id
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(Wallet {
            id: result.id,
            user_id: result.user_id,
            tron_address: result.tron_address,
            usdt_balance: result.usdt_balance,
            created_at: result.created_at,
            updated_at: result.updated_at,
        })
    }
}

/// Repository برای مدیریت عملیات واریز
pub struct DepositRepository {
    pool: PgPool,
}

impl DepositRepository {
    /// ایجاد یک نمونه جدید از DepositRepository
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    
    /// ایجاد یک درخواست واریز جدید
    pub async fn create_deposit(&self, deposit: Deposit) -> Result<Deposit, Error> {
        let result = sqlx::query!(
            r#"
            INSERT INTO deposits (
                id, user_id, amount, received_amount, transaction_hash,
                deposit_address, payment_method, status, confirmation_block,
                confirmations, required_confirmations, confirmed_at,
                expires_at, created_at, updated_at
            )
            VALUES (, , , , , , , , , , , , , , )
            RETURNING *
            "#,
            deposit.id,
            deposit.user_id,
            deposit.amount,
            deposit.received_amount,
            deposit.transaction_hash,
            deposit.deposit_address,
            deposit.payment_method as PaymentMethod,
            deposit.status as DepositStatus,
            deposit.confirmation_block,
            deposit.confirmations,
            deposit.required_confirmations,
            deposit.confirmed_at,
            deposit.expires_at,
            deposit.created_at,
            deposit.updated_at,
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(Deposit {
            id: result.id,
            user_id: result.user_id,
            amount: result.amount,
            received_amount: result.received_amount,
            transaction_hash: result.transaction_hash,
            deposit_address: result.deposit_address,
            payment_method: result.payment_method,
            status: result.status,
            confirmation_block: result.confirmation_block,
            confirmations: result.confirmations,
            required_confirmations: result.required_confirmations,
            confirmed_at: result.confirmed_at,
            expires_at: result.expires_at,
            created_at: result.created_at,
            updated_at: result.updated_at,
        })
    }
    
    /// دریافت تمام واریزهای یک کاربر
    pub async fn get_deposits_by_user_id(
        &self, 
        user_id: Uuid,
        limit: Option<i64>,
        offset: Option<i64>
    ) -> Result<Vec<Deposit>, Error> {
        let limit = limit.unwrap_or(50);
        let offset = offset.unwrap_or(0);
        
        let results = sqlx::query!(
            r#"
            SELECT * FROM deposits 
            WHERE user_id =  
            ORDER BY created_at DESC
            LIMIT  OFFSET 
            "#,
            user_id,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await?;
        
        let deposits = results.into_iter().map(|record| Deposit {
            id: record.id,
            user_id: record.user_id,
            amount: record.amount,
            received_amount: record.received_amount,
            transaction_hash: record.transaction_hash,
            deposit_address: record.deposit_address,
            payment_method: record.payment_method,
            status: record.status,
            confirmation_block: record.confirmation_block,
            confirmations: record.confirmations,
            required_confirmations: record.required_confirmations,
            confirmed_at: record.confirmed_at,
            expires_at: record.expires_at,
            created_at: record.created_at,
            updated_at: record.updated_at,
        }).collect();
        
        Ok(deposits)
    }
    
    /// دریافت واریز بر اساس شناسه
    pub async fn get_deposit_by_id(&self, deposit_id: Uuid) -> Result<Option<Deposit>, Error> {
        let result = sqlx::query!(
            r#"
            SELECT * FROM deposits WHERE id = 
            "#,
            deposit_id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        match result {
            Some(record) => Ok(Some(Deposit {
                id: record.id,
                user_id: record.user_id,
                amount: record.amount,
                received_amount: record.received_amount,
                transaction_hash: record.transaction_hash,
                deposit_address: record.deposit_address,
                payment_method: record.payment_method,
                status: record.status,
                confirmation_block: record.confirmation_block,
                confirmations: record.confirmations,
                required_confirmations: record.required_confirmations,
                confirmed_at: record.confirmed_at,
                expires_at: record.expires_at,
                created_at: record.created_at,
                updated_at: record.updated_at,
            })),
            None => Ok(None),
        }
    }
    
    /// بروزرسانی وضعیت واریز
    pub async fn update_deposit_status(
        &self,
        deposit_id: Uuid,
        new_status: DepositStatus,
        transaction_hash: Option<String>,
        received_amount: Option<BigDecimal>
    ) -> Result<Deposit, Error> {
        let updated_at = Utc::now();
        
        let result = sqlx::query!(
            r#"
            UPDATE deposits 
            SET status = , 
                transaction_hash = COALESCE(, transaction_hash),
                received_amount = COALESCE(, received_amount),
                updated_at = 
            WHERE id = 
            RETURNING *
            "#,
            new_status as DepositStatus,
            transaction_hash,
            received_amount,
            updated_at,
            deposit_id
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(Deposit {
            id: result.id,
            user_id: result.user_id,
            amount: result.amount,
            received_amount: result.received_amount,
            transaction_hash: result.transaction_hash,
            deposit_address: result.deposit_address,
            payment_method: result.payment_method,
            status: result.status,
            confirmation_block: result.confirmation_block,
            confirmations: result.confirmations,
            required_confirmations: result.required_confirmations,
            confirmed_at: result.confirmed_at,
            expires_at: result.expires_at,
            created_at: result.created_at,
            updated_at: result.updated_at,
        })
    }
    
    /// افزایش تعداد تأییدیه‌های واریز
    pub async fn add_deposit_confirmation(
        &self,
        deposit_id: Uuid,
        block_height: i64
    ) -> Result<Deposit, Error> {
        let updated_at = Utc::now();
        
        let result = sqlx::query!(
            r#"
            UPDATE deposits 
            SET confirmations = confirmations + 1,
                confirmation_block = ,
                updated_at = 
            WHERE id = 
            RETURNING *
            "#,
            block_height,
            updated_at,
            deposit_id
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(Deposit {
            id: result.id,
            user_id: result.user_id,
            amount: result.amount,
            received_amount: result.received_amount,
            transaction_hash: result.transaction_hash,
            deposit_address: result.deposit_address,
            payment_method: result.payment_method,
            status: result.status,
            confirmation_block: result.confirmation_block,
            confirmations: result.confirmations,
            required_confirmations: result.required_confirmations,
            confirmed_at: result.confirmed_at,
            expires_at: result.expires_at,
            created_at: result.created_at,
            updated_at: result.updated_at,
        })
    }
}

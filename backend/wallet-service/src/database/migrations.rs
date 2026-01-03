//! مدیریت مهاجرت‌های پایگاه داده
//!
//! این ماژول شامل اسکریپت‌های SQL برای ایجاد و به‌روزرسانی جداول دیتابیس است

use sqlx::{PgPool, Error};
use tracing::info;

/// اجرای تمام مهاجرت‌های دیتابیس
pub async fn run_migrations(pool: &PgPool) -> Result<(), Error> {
    info!("Starting database migrations...");
    
    // ایجاد enum types اگر وجود ندارند
    create_enum_types(pool).await?;
    
    // ایجاد جدول wallets
    create_wallets_table(pool).await?;
    
    // ایجاد جدول deposits
    create_deposits_table(pool).await?;
    
    // ایجاد indexes برای بهبود عملکرد
    create_indexes(pool).await?;
    
    info!("Database migrations completed successfully");
    Ok(())
}

/// ایجاد enum types مورد نیاز
async fn create_enum_types(pool: &PgPool) -> Result<(), Error> {
    info!("Creating enum types...");
    
    // ایجاد enum برای وضعیت‌های واریز
    sqlx::query(
        r#"
        DO Green 
        BEGIN
            IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'deposit_status') THEN
                CREATE TYPE deposit_status AS ENUM (
                    'pending',
                    'processing',
                    'confirmed',
                    'failed',
                    'expired',
                    'cancelled'
                );
            END IF;
        END Green;
        "#
    )
    .execute(pool)
    .await?;
    
    // ایجاد enum برای روش‌های پرداخت
    sqlx::query(
        r#"
        DO Green 
        BEGIN
            IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'payment_method') THEN
                CREATE TYPE payment_method AS ENUM (
                    'tron_usdt',
                    'bitcoin',
                    'ethereum',
                    'bank_transfer'
                );
            END IF;
        END Green;
        "#
    )
    .execute(pool)
    .await?;
    
    info!("Enum types created");
    Ok(())
}

/// ایجاد جدول wallets
async fn create_wallets_table(pool: &PgPool) -> Result<(), Error> {
    info!("Creating wallets table...");
    
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS wallets (
            -- شناسه منحصر به فرد کیف پول
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            
            -- شناسه کاربر صاحب کیف پول
            user_id UUID NOT NULL,
            
            -- آدرس TRON کیف پول (مثال: TXXXXXXXXXXXXXXXXXXXXXXXXXXX)
            tron_address VARCHAR(255) UNIQUE,
            
            -- موجودی USDT با دقت ۸ رقم اعشار
            usdt_balance DECIMAL(20, 8) NOT NULL DEFAULT 0.00000000,
            
            -- زمان ایجاد کیف پول
            created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
            
            -- زمان آخرین به‌روزرسانی
            updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
            
            -- محدودیت‌های یکتایی
            UNIQUE(user_id),
            
            -- بررسی‌های اعتبارسنجی
            CONSTRAINT valid_tron_address 
                CHECK (
                    tron_address IS NULL OR 
                    (tron_address LIKE 'T%' AND LENGTH(tron_address) = 34)
                ),
            CONSTRAINT non_negative_balance 
                CHECK (usdt_balance >= 0)
        );
        "#
    )
    .execute(pool)
    .await?;
    
    // ایجاد تریگر برای بروزرسانی خودکار updated_at
    sqlx::query(
        r#"
        CREATE OR REPLACE FUNCTION update_wallets_updated_at()
        RETURNS TRIGGER AS Green
        BEGIN
            NEW.updated_at = NOW();
            RETURN NEW;
        END;
        Green language 'plpgsql';
        
        DROP TRIGGER IF EXISTS update_wallets_timestamp ON wallets;
        
        CREATE TRIGGER update_wallets_timestamp
        BEFORE UPDATE ON wallets
        FOR EACH ROW
        EXECUTE FUNCTION update_wallets_updated_at();
        "#
    )
    .execute(pool)
    .await?;
    
    info!("Wallets table created");
    Ok(())
}

/// ایجاد جدول deposits
async fn create_deposits_table(pool: &PgPool) -> Result<(), Error> {
    info!("Creating deposits table...");
    
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS deposits (
            -- شناسه منحصر به فرد واریز
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            
            -- شناسه کاربر درخواست‌دهنده
            user_id UUID NOT NULL,
            
            -- مبلغ درخواستی برای واریز
            amount DECIMAL(20, 8) NOT NULL,
            
            -- مبلغ واقعی دریافت شده
            received_amount DECIMAL(20, 8),
            
            -- هش تراکنش بلاکچین
            transaction_hash VARCHAR(255),
            
            -- آدرس واریز (کیف پول موقت ما)
            deposit_address VARCHAR(255) NOT NULL,
            
            -- روش پرداخت انتخاب شده
            payment_method payment_method NOT NULL,
            
            -- وضعیت فعلی واریز
            status deposit_status NOT NULL DEFAULT 'pending',
            
            -- بلوک تأیید (برای بلاکچین)
            confirmation_block BIGINT,
            
            -- تعداد تأییدیه‌های دریافتی
            confirmations INTEGER NOT NULL DEFAULT 0,
            
            -- تعداد تأییدیه‌های مورد نیاز
            required_confirmations INTEGER NOT NULL DEFAULT 12,
            
            -- زمان تأیید نهایی
            confirmed_at TIMESTAMP WITH TIME ZONE,
            
            -- زمان انقضای درخواست واریز
            expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
            
            -- زمان ایجاد درخواست
            created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
            
            -- زمان آخرین به‌روزرسانی
            updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
            
            -- کلید خارجی به جدول users (در سرویس کاربران)
            -- نکته: در معماری میکروسرویس، این رابطه فقط منطقی است
            
            -- بررسی‌های اعتبارسنجی
            CONSTRAINT positive_amount CHECK (amount > 0),
            CONSTRAINT positive_received_amount 
                CHECK (received_amount IS NULL OR received_amount > 0),
            CONSTRAINT valid_confirmations 
                CHECK (confirmations >= 0 AND confirmations <= required_confirmations),
            CONSTRAINT expires_after_creation 
                CHECK (expires_at > created_at),
            CONSTRAINT confirmed_after_creation 
                CHECK (confirmed_at IS NULL OR confirmed_at >= created_at)
        );
        "#
    )
    .execute(pool)
    .await?;
    
    // ایجاد تریگر برای بروزرسانی خودکار updated_at
    sqlx::query(
        r#"
        CREATE OR REPLACE FUNCTION update_deposits_updated_at()
        RETURNS TRIGGER AS Green
        BEGIN
            NEW.updated_at = NOW();
            RETURN NEW;
        END;
        Green language 'plpgsql';
        
        DROP TRIGGER IF EXISTS update_deposits_timestamp ON deposits;
        
        CREATE TRIGGER update_deposits_timestamp
        BEFORE UPDATE ON deposits
        FOR EACH ROW
        EXECUTE FUNCTION update_deposits_updated_at();
        "#
    )
    .execute(pool)
    .await?;
    
    info!("Deposits table created");
    Ok(())
}

/// ایجاد indexes برای بهبود عملکرد
async fn create_indexes(pool: &PgPool) -> Result<(), Error> {
    info!("Creating indexes...");
    
    // Indexes برای جدول wallets
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_wallets_user_id ON wallets(user_id);"
    )
    .execute(pool)
    .await?;
    
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_wallets_tron_address ON wallets(tron_address);"
    )
    .execute(pool)
    .await?;
    
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_wallets_updated_at ON wallets(updated_at DESC);"
    )
    .execute(pool)
    .await?;
    
    // Indexes برای جدول deposits
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_deposits_user_id ON deposits(user_id);"
    )
    .execute(pool)
    .await?;
    
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_deposits_status ON deposits(status);"
    )
    .execute(pool)
    .await?;
    
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_deposits_created_at ON deposits(created_at DESC);"
    )
    .execute(pool)
    .await?;
    
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_deposits_expires_at ON deposits(expires_at);"
    )
    .execute(pool)
    .await?;
    
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_deposits_deposit_address ON deposits(deposit_address);"
    )
    .execute(pool)
    .await?;
    
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_deposits_transaction_hash ON deposits(transaction_hash);"
    )
    .execute(pool)
    .await?;
    
    // Composite index برای جستجوهای رایج
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_deposits_user_status ON deposits(user_id, status);"
    )
    .execute(pool)
    .await?;
    
    info!("Indexes created");
    Ok(())
}

/// رول‌بک تمام مهاجرت‌ها (برای توسعه)
#[allow(dead_code)]
pub async fn rollback_migrations(pool: &PgPool) -> Result<(), Error> {
    info!("Rolling back database migrations...");
    
    // حذف indexes
    sqlx::query("DROP INDEX IF EXISTS idx_deposits_user_status;").execute(pool).await?;
    sqlx::query("DROP INDEX IF EXISTS idx_deposits_transaction_hash;").execute(pool).await?;
    sqlx::query("DROP INDEX IF EXISTS idx_deposits_deposit_address;").execute(pool).await?;
    sqlx::query("DROP INDEX IF EXISTS idx_deposits_expires_at;").execute(pool).await?;
    sqlx::query("DROP INDEX IF EXISTS idx_deposits_created_at;").execute(pool).await?;
    sqlx::query("DROP INDEX IF EXISTS idx_deposits_status;").execute(pool).await?;
    sqlx::query("DROP INDEX IF EXISTS idx_deposits_user_id;").execute(pool).await?;
    
    sqlx::query("DROP INDEX IF EXISTS idx_wallets_updated_at;").execute(pool).await?;
    sqlx::query("DROP INDEX IF EXISTS idx_wallets_tron_address;").execute(pool).await?;
    sqlx::query("DROP INDEX IF EXISTS idx_wallets_user_id;").execute(pool).await?;
    
    // حذف جداول
    sqlx::query("DROP TABLE IF EXISTS deposits;").execute(pool).await?;
    sqlx::query("DROP TABLE IF EXISTS wallets;").execute(pool).await?;
    
    // حذد enum types
    sqlx::query("DROP TYPE IF EXISTS payment_method;").execute(pool).await?;
    sqlx::query("DROP TYPE IF EXISTS deposit_status;").execute(pool).await?;
    
    info!("Database migrations rolled back");
    Ok(())
}

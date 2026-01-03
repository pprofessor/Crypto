-- ============================================
-- فایل راه‌اندازی اولیه دیتابیس Crypto Exchange
-- این فایل هنگام اولین اجرای کانتینر PostgreSQL اجرا می‌شود
-- تاریخ ایجاد: $(Get-Date -Format "yyyy-MM-dd")
-- ============================================

-- ایجاد اسکمای اصلی برای جداسازی منطقی
CREATE SCHEMA IF NOT EXISTS crypto;
COMMENT ON SCHEMA crypto IS 'Schema اصلی برای موجودیت‌های هسته صرافی';

-- ۱. جدول کاربران (پایه‌ای‌ترین موجودیت)
CREATE TABLE IF NOT EXISTS crypto.users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) NOT NULL UNIQUE,
    username VARCHAR(50) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    first_name VARCHAR(100),
    last_name VARCHAR(100),
    
    -- وضعیت‌های حساب
    is_active BOOLEAN DEFAULT TRUE,
    is_verified BOOLEAN DEFAULT FALSE,
    is_2fa_enabled BOOLEAN DEFAULT FALSE,
    
    -- امنیت
    failed_login_attempts INTEGER DEFAULT 0,
    last_login_at TIMESTAMP WITH TIME ZONE,
    
    -- تمایز سطوح کاربری
    user_type VARCHAR(20) DEFAULT 'regular' CHECK (user_type IN ('regular', 'vip', 'admin')),
    
    -- متادیتا
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    deleted_at TIMESTAMP WITH TIME ZONE
);

-- ایندکس‌های پرکاربرد کاربران
CREATE INDEX idx_users_email ON crypto.users(email) WHERE deleted_at IS NULL;
CREATE INDEX idx_users_status ON crypto.users(is_active, is_verified) WHERE deleted_at IS NULL;
COMMENT ON TABLE crypto.users IS 'جدول اصلی اطلاعات کاربران سیستم';

-- ۲. جدول ارزهای پشتیبانی‌شده (ثابت)
CREATE TABLE IF NOT EXISTS crypto.cryptocurrencies (
    id SERIAL PRIMARY KEY,
    symbol VARCHAR(10) NOT NULL UNIQUE,  -- مثلا: BTC, ETH, TRX
    name VARCHAR(50) NOT NULL,           -- Bitcoin, Ethereum
    network VARCHAR(20) NOT NULL,        -- MAINNET, TESTNET
    is_active BOOLEAN DEFAULT TRUE,
    
    -- اطلاعات فنی
    decimals INTEGER DEFAULT 18,
    min_deposit_amount DECIMAL(30, 18) DEFAULT 0.0001,
    withdrawal_fee DECIMAL(30, 18) DEFAULT 0.001,
    
    -- امنیت
    required_confirmations INTEGER DEFAULT 12,
    
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- داده‌های اولیه ارزها
INSERT INTO crypto.cryptocurrencies (symbol, name, network, decimals) VALUES
    ('BTC', 'Bitcoin', 'MAINNET', 8),
    ('ETH', 'Ethereum', 'MAINNET', 18),
    ('TRX', 'Tron', 'MAINNET', 6),
    ('USDT', 'Tether', 'MAINNET', 6)
ON CONFLICT (symbol) DO NOTHING;

-- ۳. جدول کیف‌پول‌ها (هر کاربر می‌تواند چند کیف‌پول داشته باشد)
CREATE TABLE IF NOT EXISTS crypto.wallets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES crypto.users(id) ON DELETE CASCADE,
    currency_id INTEGER NOT NULL REFERENCES crypto.cryptocurrencies(id),
    
    -- موجودی‌ها
    balance DECIMAL(30, 18) DEFAULT 0 CHECK (balance >= 0),
    locked_balance DECIMAL(30, 18) DEFAULT 0 CHECK (locked_balance >= 0),
    total_balance DECIMAL(30, 18) GENERATED ALWAYS AS (balance + locked_balance) STORED,
    
    -- آدرس‌ها
    deposit_address VARCHAR(255) UNIQUE,
    private_key_encrypted TEXT, -- باید با کلید اصلی رمزگذاری شود
    
    -- امنیت
    last_balance_check TIMESTAMP WITH TIME ZONE,
    is_frozen BOOLEAN DEFAULT FALSE,
    
    -- متادیتا
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    -- هر کاربر فقط یک کیف‌پول برای هر ارز می‌تواند داشته باشد
    UNIQUE(user_id, currency_id)
);

-- ایندکس‌های حیاتی کیف‌پول‌ها
CREATE INDEX idx_wallets_user ON crypto.wallets(user_id) WHERE is_frozen = FALSE;
CREATE INDEX idx_wallets_currency ON crypto.wallets(currency_id);
CREATE INDEX idx_wallets_deposit_address ON crypto.wallets(deposit_address);
COMMENT ON TABLE crypto.wallets IS 'کیف‌پول‌های کاربران برای هر ارز دیجیتال';

-- ۴. جدول تراکنش‌ها (مرکزی برای همه فعالیت‌های مالی)
CREATE TABLE IF NOT EXISTS crypto.transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES crypto.users(id),
    wallet_id UUID REFERENCES crypto.wallets(id),
    currency_id INTEGER NOT NULL REFERENCES crypto.cryptocurrencies(id),
    
    -- شناسه‌های خارجی
    external_tx_id VARCHAR(255),  -- شناسه تراکنش در بلاکچین
    internal_reference VARCHAR(50) UNIQUE,  -- شناسه داخلی
    
    -- مقادیر
    amount DECIMAL(30, 18) NOT NULL,
    fee DECIMAL(30, 18) DEFAULT 0,
    net_amount DECIMAL(30, 18) GENERATED ALWAYS AS (amount - fee) STORED,
    
    -- انواع تراکنش
    type VARCHAR(20) NOT NULL CHECK (type IN ('DEPOSIT', 'WITHDRAWAL', 'TRADE', 'TRANSFER')),
    status VARCHAR(20) NOT NULL CHECK (status IN ('PENDING', 'CONFIRMED', 'FAILED', 'CANCELLED')),
    
    -- آدرس‌ها
    from_address VARCHAR(255),
    to_address VARCHAR(255) NOT NULL,
    
    -- تأییدیه‌های بلاکچین
    confirmations INTEGER DEFAULT 0,
    required_confirmations INTEGER DEFAULT 12,
    
    -- متادیتا
    description TEXT,
    metadata JSONB,
    
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    completed_at TIMESTAMP WITH TIME ZONE
);

-- ایندکس‌های پرکاربرد تراکنش‌ها
CREATE INDEX idx_transactions_user ON crypto.transactions(user_id, created_at DESC);
CREATE INDEX idx_transactions_status ON crypto.transactions(status) WHERE status = 'PENDING';
CREATE INDEX idx_transactions_external ON crypto.transactions(external_tx_id);
CREATE INDEX idx_transactions_created ON crypto.transactions(created_at DESC);
COMMENT ON TABLE crypto.transactions IS 'همه تراکنش‌های مالی سیستم';

-- ۵. جدول سفارش‌های معاملاتی
CREATE TABLE IF NOT EXISTS crypto.orders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES crypto.users(id),
    
    -- جفت‌ارز
    base_currency_id INTEGER NOT NULL REFERENCES crypto.cryptocurrencies(id),
    quote_currency_id INTEGER NOT NULL REFERENCES crypto.cryptocurrencies(id),
    
    -- نوع و جهت
    order_type VARCHAR(10) NOT NULL CHECK (order_type IN ('LIMIT', 'MARKET')),
    side VARCHAR(10) NOT NULL CHECK (side IN ('BUY', 'SELL')),
    
    -- مقادیر
    price DECIMAL(30, 18),  -- برای سفارش‌های MARKET می‌تواند NULL باشد
    original_quantity DECIMAL(30, 18) NOT NULL,
    filled_quantity DECIMAL(30, 18) DEFAULT 0,
    remaining_quantity DECIMAL(30, 18) GENERATED ALWAYS AS (original_quantity - filled_quantity) STORED,
    
    -- وضعیت
    status VARCHAR(20) NOT NULL CHECK (status IN ('OPEN', 'PARTIALLY_FILLED', 'FILLED', 'CANCELLED', 'EXPIRED')),
    
    -- زمان‌ها
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    expired_at TIMESTAMP WITH TIME ZONE,
    
    -- شرایط
    time_in_force VARCHAR(10) DEFAULT 'GTC' CHECK (time_in_force IN ('GTC', 'IOC', 'FOK')),
    
    -- لغو
    cancelled_reason TEXT,
    
    CHECK (base_currency_id != quote_currency_id),
    CHECK (filled_quantity <= original_quantity)
);

-- ایندکس‌های حیاتی سفارش‌ها
CREATE INDEX idx_orders_user ON crypto.orders(user_id, created_at DESC);
CREATE INDEX idx_orders_market ON crypto.orders(base_currency_id, quote_currency_id, status);
CREATE INDEX idx_orders_price ON crypto.orders(price) WHERE order_type = 'LIMIT';
CREATE INDEX idx_orders_status ON crypto.orders(status) WHERE status IN ('OPEN', 'PARTIALLY_FILLED');
COMMENT ON TABLE crypto.orders IS 'سفارش‌های معاملاتی کاربران';

-- ============================================
-- توابع و تریگرهای کمکی
-- ============================================

-- تابع به‌روزرسانی خودکار updated_at
CREATE OR REPLACE FUNCTION crypto.update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- اعمال تریگر برای همه جدول‌های مهم
DO $$ 
DECLARE 
    tbl text;
BEGIN
    FOR tbl IN 
        SELECT table_name 
        FROM information_schema.tables 
        WHERE table_schema = 'crypto' 
        AND table_type = 'BASE TABLE'
    LOOP
        EXECUTE format('
            DROP TRIGGER IF EXISTS update_%s_updated_at ON crypto.%s;
            CREATE TRIGGER update_%s_updated_at
            BEFORE UPDATE ON crypto.%s
            FOR EACH ROW
            EXECUTE FUNCTION crypto.update_updated_at_column();
        ', tbl, tbl, tbl, tbl);
    END LOOP;
END $$;

-- تابع محاسبه موجودی کل کاربر
CREATE OR REPLACE FUNCTION crypto.calculate_user_total_balance(user_uuid UUID)
RETURNS DECIMAL(30, 18) AS $$
DECLARE
    total DECIMAL(30, 18);
BEGIN
    SELECT COALESCE(SUM(w.total_balance), 0)
    INTO total
    FROM crypto.wallets w
    WHERE w.user_id = user_uuid
    AND w.is_frozen = FALSE;
    
    RETURN total;
END;
$$ LANGUAGE plpgsql;

-- نمایش پیام موفقیت
DO $$
BEGIN
    RAISE NOTICE '✅ Schema دیتابیس با موفقیت ایجاد شد.';
    RAISE NOTICE '   تعداد جداول: ۵';
    RAISE NOTICE '   Schema: crypto';
    RAISE NOTICE '   تاریخ: %', NOW();
END $$;

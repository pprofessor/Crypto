# مستندات معماری دیتابیس Crypto Exchange

## فهرست مطالب
1. معماری کلی
2. جداول اصلی
3. ایندکس‌ها
4. دستورات مفید

## معماری کلی
Schema: crypto
- users (کاربران سیستم)
- cryptocurrencies (ارزهای پشتیبانی شده)
- wallets (کیف پول‌های کاربران)
- transactions (تراکنش‌های مالی)
- orders (سفارش‌های معاملاتی)

## جداول اصلی

1. جدول users (کاربران)
CREATE TABLE crypto.users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) NOT NULL UNIQUE,
    username VARCHAR(50) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    is_active BOOLEAN DEFAULT TRUE,
    is_verified BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

توضیحات:
- id: شناسه یکتا (UUID)
- email: ایمیل کاربر (منحصر به فرد)
- password_hash: هش رمز عبور با bcrypt
- is_active: وضعیت فعال بودن حساب
- is_verified: تأیید ایمیل انجام شده

2. جدول cryptocurrencies (ارزها)
CREATE TABLE crypto.cryptocurrencies (
    id SERIAL PRIMARY KEY,
    symbol VARCHAR(10) NOT NULL UNIQUE,
    name VARCHAR(100) NOT NULL,
    network VARCHAR(50) NOT NULL,
    decimals INTEGER DEFAULT 18,
    is_active BOOLEAN DEFAULT TRUE
);

ارزهای پیش‌فرض:
- BTC (Bitcoin)
- ETH (Ethereum)
- TRX (Tron)
- USDT (Tether)

3. جدول wallets (کیف پول‌ها)
CREATE TABLE crypto.wallets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES crypto.users(id),
    currency_id INTEGER NOT NULL REFERENCES crypto.cryptocurrencies(id),
    balance DECIMAL(30, 18) DEFAULT 0,
    locked_balance DECIMAL(30, 18) DEFAULT 0,
    deposit_address VARCHAR(255) UNIQUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

قوانین:
- هر کاربر فقط یک کیف‌پول برای هر ارز می‌تواند داشته باشد
- موجودی نمی‌تواند منفی شود
- locked_balance برای سفارشات باز رزرو می‌شود

4. جدول transactions (تراکنش‌ها)
CREATE TABLE crypto.transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES crypto.users(id),
    wallet_id UUID REFERENCES crypto.wallets(id),
    currency_id INTEGER NOT NULL REFERENCES crypto.cryptocurrencies(id),
    amount DECIMAL(30, 18) NOT NULL,
    type VARCHAR(20) NOT NULL CHECK (type IN ('DEPOSIT', 'WITHDRAWAL', 'TRADE')),
    status VARCHAR(20) NOT NULL CHECK (status IN ('PENDING', 'CONFIRMED', 'FAILED')),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

انواع تراکنش:
- DEPOSIT: واریز به حساب
- WITHDRAWAL: برداشت از حساب
- TRADE: معامله

5. جدول orders (سفارشات)
CREATE TABLE crypto.orders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES crypto.users(id),
    base_currency_id INTEGER NOT NULL REFERENCES crypto.cryptocurrencies(id),
    quote_currency_id INTEGER NOT NULL REFERENCES crypto.cryptocurrencies(id),
    order_type VARCHAR(10) NOT NULL CHECK (order_type IN ('LIMIT', 'MARKET')),
    side VARCHAR(10) NOT NULL CHECK (side IN ('BUY', 'SELL')),
    price DECIMAL(30, 18),
    quantity DECIMAL(30, 18) NOT NULL,
    status VARCHAR(20) DEFAULT 'OPEN' CHECK (status IN ('OPEN', 'FILLED', 'CANCELLED')),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

## ایندکس‌های مهم

CREATE INDEX idx_users_email ON crypto.users(email);
CREATE INDEX idx_wallets_user ON crypto.wallets(user_id);
CREATE INDEX idx_transactions_user_date ON crypto.transactions(user_id, created_at DESC);
CREATE INDEX idx_orders_status ON crypto.orders(status) WHERE status = 'OPEN';

## دستورات مفید

-- مشاهده ساختار دیتابیس
SELECT table_name FROM information_schema.tables WHERE table_schema = 'crypto';

-- مشاهده حجم جداول
SELECT table_name, pg_size_pretty(pg_total_relation_size('crypto.' || table_name))
FROM information_schema.tables WHERE table_schema = 'crypto';

-- تعداد کاربران فعال
SELECT COUNT(*) FROM crypto.users WHERE is_active = TRUE;

-- موجودی کل بیت‌کوین
SELECT SUM(balance) FROM crypto.wallets WHERE currency_id = 1;

## نکات امنیتی

1. همیشه از پارامترهای امن در کوئری‌ها استفاده کنید
2. فیلد password_hash باید با bcrypt هش شود
3. backup روزانه از دیتابیس بگیرید
4. از SSL/TLS برای اتصال به دیتابیس استفاده کنید

آخرین بروزرسانی: ۱۴۰۲/۱۰/۱۴
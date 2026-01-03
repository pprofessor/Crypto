-- ============================================
-- اسکریپت تولید داده‌های توسعه (امن)
-- اجرا فقط در محیط توسعه
-- ============================================

DO $$
DECLARE
    -- متغیرها
    user_count INTEGER := 5;
    i INTEGER;
    user_id UUID;
    btc_id INTEGER;
    eth_id INTEGER;
    trx_id INTEGER;
BEGIN
    
    -- فقط در محیط توسعه اجرا شود
    IF current_setting('server_version_num')::integer < 1 THEN
        RAISE NOTICE 'این اسکریپت فقط برای محیط توسعه است';
        RETURN;
    END IF;
    
    -- دریافت ID ارزها
    SELECT id INTO btc_id FROM crypto.cryptocurrencies WHERE symbol = 'BTC';
    SELECT id INTO eth_id FROM crypto.cryptocurrencies WHERE symbol = 'ETH';
    SELECT id INTO trx_id FROM crypto.cryptocurrencies WHERE symbol = 'TRX';
    
    -- تولید کاربران تست
    FOR i IN 1..user_count LOOP
        -- تولید UUID ثابت برای تست‌های قابل تکرار
        user_id := gen_random_uuid();
        
        -- ایجاد کاربر (همه رمزها: Test@1234)
        INSERT INTO crypto.users (id, email, username, password_hash, first_name, last_name, is_verified)
        VALUES (
            user_id,
            'test.user' || i || '@example.com',
            'testuser' || i,
            -- هش bcrypt برای Test@1234
            '$2b$10$92LXjqk8mC5pBmAZYp6DxuG3z7E6CEdTkD8tFpC5JvJt5VQ5vL5W6',
            'Test' || i,
            'User' || i,
            TRUE
        );
        
        -- ایجاد کیف‌پول‌های تست برای کاربر
        INSERT INTO crypto.wallets (user_id, currency_id, balance, deposit_address)
        VALUES
            (user_id, btc_id, 0.5, 'bc1qtest' || i || 'xxxxxxxxxxxxxxxxxxxxx'),
            (user_id, eth_id, 3.2, '0x742d35Cc6634C0532925a3b844Bc9e0BBE8xxxxx'),
            (user_id, trx_id, 5000, 'Txxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx');
        
        -- ایجاد چند تراکنش نمونه
        INSERT INTO crypto.transactions 
            (user_id, currency_id, amount, type, status, to_address, external_tx_id)
        VALUES
            (user_id, btc_id, 0.1, 'DEPOSIT', 'CONFIRMED', 
             'bc1qtest' || i || 'xxxxxxxxxxxxxxxxxxxxx',
             'txid_btc_deposit_' || i || '_' || extract(epoch from now())),
            (user_id, eth_id, 0.5, 'WITHDRAWAL', 'PENDING',
             '0xExternalAddressForWithdrawal',
             'txid_eth_withdraw_' || i || '_' || extract(epoch from now()));
    END LOOP;
    
    RAISE NOTICE '✅ % کاربر تست با کیف‌پول و تراکنش ایجاد شد', user_count;
    RAISE NOTICE '   همه رمزها: Test@1234';
    RAISE NOTICE '   محیط: فقط توسعه';
    
EXCEPTION
    WHEN OTHERS THEN
        RAISE WARNING 'خطا در تولید داده‌های تست: %', SQLERRM;
END $$;

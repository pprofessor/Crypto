-- داده‌های تست ساده
INSERT INTO crypto.users (id, email, username, password_hash, is_verified) 
VALUES 
    ('11111111-1111-1111-1111-111111111111', 'test1@example.com', 'testuser1', '$2b$10$92LXjqk8mC5pBmAZYp6DxuG3z7E6CEdTkD8tFpC5JvJt5VQ5vL5W6', true),
    ('22222222-2222-2222-2222-222222222222', 'test2@example.com', 'testuser2', '$2b$10$92LXjqk8mC5pBmAZYp6DxuG3z7E6CEdTkD8tFpC5JvJt5VQ5vL5W6', true)
ON CONFLICT (id) DO NOTHING;

-- اضافه کردن کیف‌پول
INSERT INTO crypto.wallets (user_id, currency_id, balance, deposit_address)
SELECT 
    u.id,
    c.id,
    100.0,
    'addr_test_' || u.username
FROM crypto.users u
CROSS JOIN crypto.cryptocurrencies c
WHERE u.email LIKE 'test%@example.com'
ON CONFLICT (user_id, currency_id) DO NOTHING;

SELECT '✅ داده‌های تست اضافه شدند' as message;

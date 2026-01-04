# 📚 API Reference - Crypto Options Exchange

## 🏗️ Base Information
- **Base URL**: `http://localhost:3001/api`
- **Content-Type**: `application/json`
- **Authentication**: Bearer Token (JWT)

## 📊 Authentication & User Management

### 🔐 Authentication Endpoints

#### 1. Register User
POST /auth/register

Request Body:
{
  "email": "user@example.com",
  "username": "john_doe",
  "password": "SecurePassword123!",
  "firstName": "John",
  "lastName": "Doe",
  "phoneNumber": "+989123456789"
}

Validation Rules:
- Email: Valid email format, unique
- Username: 3-50 chars, letters/numbers/underscore, unique
- Password: Min 8 chars, uppercase, lowercase, number, special char
- Phone: Valid international format (optional)

Response:
{
  "user": {
    "id": "uuid",
    "email": "user@example.com",
    "username": "john_doe",
    "firstName": "John",
    "lastName": "Doe",
    "phoneNumber": "+989123456789",
    "isActive": true,
    "isVerified": false,
    "userType": "regular",
    "createdAt": "2026-01-04T09:00:00.000Z",
    "updatedAt": "2026-01-04T09:00:00.000Z"
  },
  "accessToken": "jwt-token-here",
  "refreshToken": "refresh-token-here"
}

#### 2. Login User
POST /auth/login

Request Body (email OR username):
{
  "email": "user@example.com",
  "password": "SecurePassword123!"
}
OR
{
  "username": "john_doe",
  "password": "SecurePassword123!"
}

Response:
{
  "user": {
    "id": "uuid",
    "email": "user@example.com",
    "username": "john_doe",
    "firstName": "John",
    "lastName": "Doe",
    "phoneNumber": "+989123456789",
    "isActive": true,
    "isVerified": true,
    "userType": "regular",
    "lastLoginAt": "2026-01-04T09:00:00.000Z"
  },
  "accessToken": "jwt-token-here",
  "refreshToken": "refresh-token-here"
}

#### 3. Refresh Token
POST /auth/refresh

Request Body:
{
  "refreshToken": "refresh-token-from-login"
}

Response:
{
  "accessToken": "new-jwt-token",
  "refreshToken": "new-refresh-token"
}

#### 4. Logout
POST /auth/logout
Authorization: Bearer <access-token>

Request Body (optional):
{
  "refreshToken": "specific-token-to-revoke"
}

Response: 200 OK

#### 5. Change Password
POST /auth/change-password
Authorization: Bearer <access-token>

Request Body:
{
  "oldPassword": "CurrentPassword123!",
  "newPassword": "NewSecurePassword456!"
}

Response: 200 OK

### 👤 User Profile Endpoints

#### 1. Get Profile
GET /auth/profile
Authorization: Bearer <access-token>

Response:
{
  "id": "uuid",
  "email": "user@example.com",
  "username": "john_doe",
  "firstName": "John",
  "lastName": "Doe",
  "phoneNumber": "+989123456789",
  "isActive": true,
  "isVerified": true,
  "is2faEnabled": false,
  "userType": "regular",
  "kycStatus": "PENDING",
  "dailyWithdrawalLimit": 1000,
  "monthlyTradeLimit": 10000,
  "lastLoginAt": "2026-01-04T09:00:00.000Z",
  "createdAt": "2026-01-04T09:00:00.000Z",
  "updatedAt": "2026-01-04T09:00:00.000Z"
}

#### 2. Update Profile
PUT /auth/profile
Authorization: Bearer <access-token>

Request Body (any fields to update):
{
  "firstName": "John Updated",
  "lastName": "Doe Updated",
  "phoneNumber": "+989987654321",
  "withdrawalWhitelist": ["address1", "address2"]
}

Response: Updated profile object

#### 3. Verify Email
POST /auth/verify-email/:token

Response: 200 OK

#### 4. Request Password Reset
POST /auth/request-password-reset

Request Body:
{
  "email": "user@example.com"
}

Response:
{
  "resetToken": "reset-token-for-email"
}

#### 5. Reset Password
POST /auth/reset-password

Request Body:
{
  "resetToken": "token-from-email",
  "newPassword": "NewPassword123!"
}

Response: 200 OK

## 🗃️ Database Schema Reference

### Tables in 'crypto' Schema:

1. **users** - User accounts and profiles
   - id (UUID, PK)
   - email (VARCHAR(255), UNIQUE)
   - username (VARCHAR(50), UNIQUE)
   - password_hash (VARCHAR(255))
   - is_active (BOOLEAN)
   - is_verified (BOOLEAN)
   - user_type (ENUM: regular, vip, admin)
   - kyc_status (ENUM: PENDING, VERIFIED, REJECTED)
   - daily_withdrawal_limit (DECIMAL(30,18))
   - monthly_trade_limit (DECIMAL(30,18))
   - created_at, updated_at, deleted_at (TIMESTAMPTZ)

2. **refresh_tokens** - JWT refresh tokens
   - id (UUID, PK)
   - token (VARCHAR(255), UNIQUE)
   - user_id (UUID, FK to users)
   - expires_at (TIMESTAMPTZ)
   - is_revoked (BOOLEAN)
   - ip_address, user_agent, device_info

3. **cryptocurrencies** - Supported currencies
   - id (SERIAL, PK)
   - symbol (VARCHAR(10), UNIQUE) - BTC, ETH, TRX, USDT
   - name (VARCHAR(100))
   - network (VARCHAR(50))
   - decimals (INTEGER)
   - is_active (BOOLEAN)

4. **wallets** - User cryptocurrency wallets
   - id (UUID, PK)
   - user_id (UUID, FK to users)
   - currency_id (INTEGER, FK to cryptocurrencies)
   - balance (DECIMAL(30,18))
   - locked_balance (DECIMAL(30,18))
   - deposit_address (VARCHAR(255), UNIQUE)

5. **transactions** - Financial transactions
   - id (UUID, PK)
   - user_id (UUID, FK to users)
   - wallet_id (UUID, FK to wallets)
   - currency_id (INTEGER, FK to cryptocurrencies)
   - amount (DECIMAL(30,18))
   - type (ENUM: DEPOSIT, WITHDRAWAL, TRADE)
   - status (ENUM: PENDING, CONFIRMED, FAILED)
   - created_at (TIMESTAMPTZ)

6. **orders** - Trading orders
   - id (UUID, PK)
   - user_id (UUID, FK to users)
   - base_currency_id (INTEGER, FK to cryptocurrencies)
   - quote_currency_id (INTEGER, FK to cryptocurrencies)
   - order_type (ENUM: LIMIT, MARKET)
   - side (ENUM: BUY, SELL)
   - price (DECIMAL(30,18))
   - quantity (DECIMAL(30,18))
   - status (ENUM: OPEN, FILLED, CANCELLED)
   - created_at (TIMESTAMPTZ)

## 🔧 Environment Variables

### Required for User-Service:
PORT=3001
NODE_ENV=development
DB_HOST=localhost
DB_PORT=5433
DB_USERNAME=crypto_user
DB_PASSWORD=ChangeMe123!
DB_DATABASE=crypto_exchange
DB_SCHEMA=crypto
REDIS_HOST=localhost
REDIS_PORT=6380
REDIS_PASSWORD=RedisPass123!
JWT_SECRET=your-secret-key-here
JWT_EXPIRATION=1d
REFRESH_TOKEN_SECRET=your-refresh-secret
REFRESH_TOKEN_EXPIRATION=7d
BCRYPT_SALT_ROUNDS=10

## 🚀 Quick Start Commands

### Start Database (Podman):
cd docker && podman-compose -f docker-compose-simple.yml up -d

### Start User-Service:
cd backend/user-service
npm install
npx ts-node src/main.ts

### Test API:
curl -X POST http://localhost:3001/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{"email":"test@example.com","username":"testuser","password":"Test@1234"}'

## 📈 Status Codes

- 200: Success
- 201: Created
- 400: Bad Request (validation error)
- 401: Unauthorized (invalid/missing token)
- 403: Forbidden (insufficient permissions)
- 404: Not Found
- 409: Conflict (duplicate email/username)
- 500: Internal Server Error

## 🔒 Security Notes

1. All passwords are hashed with bcrypt (salt rounds: 10)
2. JWT tokens expire in 1 day (configurable)
3. Refresh tokens expire in 7 days
4. Failed login attempts are tracked (5 attempts locks account)
5. CORS is configured for http://localhost:3000
6. Input validation on all endpoints
7. SQL injection prevention via TypeORM

## 🐛 Troubleshooting

### Database Connection Issues:
1. Check if PostgreSQL is running: `podman ps | grep postgres`
2. Test connection: `podman exec crypto-postgres psql -U crypto_user -d crypto_exchange -c "SELECT 1;"`
3. Verify credentials in .env file

### Service Won't Start:
1. Check port 3001 is free: `netstat -tulpn | grep 3001`
2. Verify dependencies: `npm install`
3. Check TypeScript compilation: `npx tsc --noEmit`

### Authentication Issues:
1. Verify JWT_SECRET in .env
2. Check token expiration
3. Validate password meets complexity requirements

---
**Last Updated**: 2026-01-04
**Version**: 2.0.0
**Maintainer**: Crypto Exchange Development Team
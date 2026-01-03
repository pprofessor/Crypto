# API REFERENCE - CRYPTO OPTIONS EXCHANGE

## 📋 Overview
This document describes all API endpoints available in the Crypto Options Exchange platform.

## 🔐 Authentication
All protected endpoints require JWT token in the Authorization header:
\\\
Authorization: Bearer <jwt_token>
\\\

---

## 👤 USER SERVICE (Port: 3000)

### Authentication Endpoints

#### POST /auth/register
Register a new user.

**Request Body:**
\\\json
{
  "email": "user@example.com",
  "password": "SecurePassword123!"
}
\\\

**Response:**
\\\json
{
  "id": "uuid-v4",
  "email": "user@example.com",
  "access_token": "jwt_token",
  "refresh_token": "refresh_token",
  "created_at": "2026-01-03T18:40:00Z"
}
\\\

---

#### POST /auth/login
Authenticate user and receive tokens.

**Request Body:**
\\\json
{
  "email": "user@example.com",
  "password": "SecurePassword123!"
}
\\\

**Response:** Same as register endpoint.

---

#### POST /auth/refresh
Refresh access token using refresh token.

**Request Body:**
\\\json
{
  "refresh_token": "refresh_token_string"
}
\\\

---

#### GET /auth/profile
Get current user profile (Protected).

**Headers:**
\\\
Authorization: Bearer <jwt_token>
\\\

**Response:**
\\\json
{
  "id": "uuid-v4",
  "email": "user@example.com",
  "is_verified": false,
  "created_at": "2026-01-03T18:40:00Z",
  "updated_at": "2026-01-03T18:40:00Z"
}
\\\

---

## 💰 WALLET SERVICE (Port: 8081)

### Wallet Management

#### POST /wallet/create
Create a new wallet for user.

**Request Body:**
\\\json
{
  "user_id": "uuid-v4"
}
\\\

**Response:**
\\\json
{
  "id": "uuid-v4",
  "user_id": "uuid-v4",
  "tron_address": "TXXXXXXXXXXXXXXXXXXXXXXXXXXX",
  "usdt_balance": "0.00000000",
  "created_at": "2026-01-03T18:40:00Z"
}
\\\

---

#### GET /wallet/{user_id}
Get wallet details for specific user.

**Response:**
\\\json
{
  "id": "uuid-v4",
  "user_id": "uuid-v4",
  "tron_address": "TXXXXXXXXXXXXXXXXXXXXXXXXXXX",
  "usdt_balance": "100.50000000",
  "created_at": "2026-01-03T18:40:00Z",
  "updated_at": "2026-01-03T19:00:00Z"
}
\\\

---

#### GET /wallet/balance/{user_id}
Get current wallet balance.

**Response:**
\\\json
{
  "user_id": "uuid-v4",
  "usdt_balance": "100.50000000",
  "last_updated": "2026-01-03T19:00:00Z"
}
\\\

---

## 💳 DEPOSIT SERVICE (Port: 8082)

### Deposit Management

#### POST /deposit/create
Create a new deposit request.

**Request Body:**
\\\json
{
  "user_id": "uuid-v4",
  "amount": "50.00000000",
  "payment_method": "tron_usdt"
}
\\\

**Response:**
\\\json
{
  "id": "uuid-v4",
  "user_id": "uuid-v4",
  "amount": "50.00000000",
  "payment_method": "tron_usdt",
  "deposit_address": "TXXXXXXXXXXXXXXXXXXXXXXXXXXX",
  "status": "pending",
  "expires_at": "2026-01-04T18:40:00Z",
  "created_at": "2026-01-03T18:40:00Z"
}
\\\

---

#### GET /deposits/{user_id}
Get all deposits for a user.

**Response:**
\\\json
[
  {
    "id": "uuid-v4",
    "user_id": "uuid-v4",
    "amount": "50.00000000",
    "payment_method": "tron_usdt",
    "status": "completed",
    "transaction_hash": "0x...",
    "created_at": "2026-01-03T18:40:00Z",
    "confirmed_at": "2026-01-03T18:45:00Z"
  }
]
\\\

---

#### GET /deposit/status/{deposit_id}
Check status of specific deposit.

**Response:**
\\\json
{
  "id": "uuid-v4",
  "status": "pending",
  "confirmations": 2,
  "required_confirmations": 12,
  "estimated_completion": "2026-01-03T18:50:00Z"
}
\\\

---

## 📊 HEALTH CHECKS

#### GET /health (All Services)
Check service health status.

**Response:**
\\\json
{
  "status": "healthy",
  "timestamp": "2026-01-03T18:40:00Z",
  "service": "user-service",
  "version": "1.0.0-alpha",
  "database": "connected",
  "uptime": "5m30s"
}
\\\

---

## 🚨 ERROR CODES

| Code | Meaning | Description |
|------|---------|-------------|
| 200 | OK | Request successful |
| 201 | Created | Resource created successfully |
| 400 | Bad Request | Invalid request parameters |
| 401 | Unauthorized | Authentication required |
| 403 | Forbidden | Insufficient permissions |
| 404 | Not Found | Resource not found |
| 409 | Conflict | Resource already exists |
| 422 | Unprocessable Entity | Validation failed |
| 429 | Too Many Requests | Rate limit exceeded |
| 500 | Internal Server Error | Server error |

---

## 📝 NOTES

1. All amounts are in **USDT** with 8 decimal precision
2. Timestamps are in ISO 8601 format (UTC)
3. UUIDs follow version 4 format
4. JWT tokens expire after 24 hours
5. Deposit addresses expire after 24 hours

---

## 🔄 WEBHOOKS (Future Implementation)

\\\
POST /webhook/tron/transaction
POST /webhook/deposit/confirmed
POST /webhook/withdrawal/processed
\\\

---
*Last Updated: 2026-01-03*
*API Version: v1*

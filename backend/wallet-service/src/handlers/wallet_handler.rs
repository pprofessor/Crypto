//! Handlerهای مربوط به مدیریت کیف پول و واریز
//!
//! این ماژول شامل endpointهای REST API برای عملیات کیف پول است

use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;
use bigdecimal::BigDecimal;
use std::str::FromStr;
use tracing::{info, warn, error};

use crate::{
    AppState,
    entities::{
        wallet::{Wallet, CreateWalletRequest, WalletResponse},
        deposit::{Deposit, CreateDepositRequest, DepositResponse, PaymentMethod, DepositStatus},
    },
    database::{WalletRepository, DepositRepository},
    errors::ApiError,
};

/// ایجاد کیف پول جدید برای کاربر
/// 
/// # Endpoint
/// POST /api/v1/wallets
/// 
/// # Request Body
/// `json
/// {
///     "user_id": "uuid-v4"
/// }
/// `
/// 
/// # Response
/// `json
/// {
///     "id": "uuid-v4",
///     "user_id": "uuid-v4",
///     "tron_address": null,
///     "usdt_balance": "0.00000000",
///     "created_at": "2026-01-03T18:40:00Z",
///     "is_active": false
/// }
/// `
pub async fn create_wallet(
    state: web::Data<AppState>,
    req: web::Json<CreateWalletRequest>,
) -> Result<impl Responder, ApiError> {
    info!("Creating wallet for user: {}", req.user_id);
    
    // اعتبارسنجی ورودی
    if req.user_id.is_nil() {
        return Err(ApiError::validation_error("user_id cannot be empty"));
    }
    
    let repo = WalletRepository::new(state.pool.clone());
    
    // بررسی اینکه آیا کاربر قبلاً کیف پول دارد
    match repo.get_wallet_by_user_id(req.user_id).await {
        Ok(Some(existing_wallet)) => {
            warn!("Wallet already exists for user: {}", req.user_id);
            let response: WalletResponse = existing_wallet.into();
            return Ok(HttpResponse::Ok().json(response));
        }
        Ok(None) => {
            // ایجاد کیف پول جدید
            let wallet = repo.create_wallet(req.user_id).await
                .map_err(|e| {
                    error!("Failed to create wallet: {}", e);
                    ApiError::internal_error("Failed to create wallet")
                })?;
            
            info!("Wallet created successfully: {}", wallet.id);
            let response: WalletResponse = wallet.into();
            Ok(HttpResponse::Created().json(response))
        }
        Err(e) => {
            error!("Database error: {}", e);
            Err(ApiError::internal_error("Database error"))
        }
    }
}

/// دریافت اطلاعات کیف پول کاربر
/// 
/// # Endpoint
/// GET /api/v1/wallets/{user_id}
/// 
/// # Response
/// `json
/// {
///     "id": "uuid-v4",
///     "user_id": "uuid-v4",
///     "tron_address": "TXXXXXXXXXXXXXXXXXXXXXXXXXXX",
///     "usdt_balance": "100.50000000",
///     "created_at": "2026-01-03T18:40:00Z",
///     "is_active": true
/// }
/// `
pub async fn get_wallet(
    state: web::Data<AppState>,
    user_id: web::Path<Uuid>,
) -> Result<impl Responder, ApiError> {
    info!("Getting wallet for user: {}", user_id);
    
    let repo = WalletRepository::new(state.pool.clone());
    
    match repo.get_wallet_by_user_id(*user_id).await {
        Ok(Some(wallet)) => {
            let response: WalletResponse = wallet.into();
            Ok(HttpResponse::Ok().json(response))
        }
        Ok(None) => {
            warn!("Wallet not found for user: {}", user_id);
            Err(ApiError::not_found("Wallet not found"))
        }
        Err(e) => {
            error!("Database error: {}", e);
            Err(ApiError::internal_error("Database error"))
        }
    }
}

/// دریافت موجودی کیف پول کاربر
/// 
/// # Endpoint
/// GET /api/v1/wallets/{user_id}/balance
/// 
/// # Response
/// `json
/// {
///     "user_id": "uuid-v4",
///     "usdt_balance": "100.50000000",
///     "last_updated": "2026-01-03T19:00:00Z"
/// }
/// `
pub async fn get_balance(
    state: web::Data<AppState>,
    user_id: web::Path<Uuid>,
) -> Result<impl Responder, ApiError> {
    info!("Getting balance for user: {}", user_id);
    
    let repo = WalletRepository::new(state.pool.clone());
    
    match repo.get_wallet_by_user_id(*user_id).await {
        Ok(Some(wallet)) => {
            #[derive(serde::Serialize)]
            struct BalanceResponse {
                user_id: Uuid,
                usdt_balance: String,
                last_updated: chrono::DateTime<chrono::Utc>,
            }
            
            let response = BalanceResponse {
                user_id: wallet.user_id,
                usdt_balance: wallet.usdt_balance.to_string(),
                last_updated: wallet.updated_at,
            };
            
            Ok(HttpResponse::Ok().json(response))
        }
        Ok(None) => {
            warn!("Wallet not found for user: {}", user_id);
            Err(ApiError::not_found("Wallet not found"))
        }
        Err(e) => {
            error!("Database error: {}", e);
            Err(ApiError::internal_error("Database error"))
        }
    }
}

/// ایجاد درخواست واریز جدید
/// 
/// # Endpoint
/// POST /api/v1/deposits
/// 
/// # Request Body
/// `json
/// {
///     "user_id": "uuid-v4",
///     "amount": "50.00000000",
///     "payment_method": "tron_usdt"
/// }
/// `
/// 
/// # Response
/// `json
/// {
///     "id": "uuid-v4",
///     "user_id": "uuid-v4",
///     "amount": "50.00000000",
///     "deposit_address": "TXXXXXXXXXXXXXXXXXXXXXXXXXXX",
///     "payment_method": "tron_usdt",
///     "status": "pending",
///     "expires_at": "2026-01-04T18:40:00Z",
///     "created_at": "2026-01-03T18:40:00Z",
///     "is_expired": false,
///     "is_confirmed": false
/// }
/// `
pub async fn create_deposit(
    state: web::Data<AppState>,
    req: web::Json<CreateDepositRequest>,
) -> Result<impl Responder, ApiError> {
    info!("Creating deposit request for user: {}", req.user_id);
    
    // اعتبارسنجی ورودی
    let amount = BigDecimal::from_str(&req.amount)
        .map_err(|_| ApiError::validation_error("Invalid amount format"))?;
    
    // بررسی وجود کیف پول کاربر
    let wallet_repo = WalletRepository::new(state.pool.clone());
    let wallet = wallet_repo.get_wallet_by_user_id(req.user_id).await
        .map_err(|e| {
            error!("Database error: {}", e);
            ApiError::internal_error("Database error")
        })?
        .ok_or_else(|| {
            warn!("Wallet not found for user: {}", req.user_id);
            ApiError::not_found("Wallet not found. Create a wallet first.")
        })?;
    
    // تولید آدرس واریز (در نسخه واقعی، از TRON API استفاده می‌شود)
    // فعلاً از یک آدرس mock استفاده می‌کنیم
    let deposit_address = if wallet.tron_address.is_some() {
        // اگر کاربر آدرس TRON دارد، از همان استفاده کن
        wallet.tron_address.unwrap()
    } else {
        // در غیر این صورت یک آدرس mock تولید کن
        format!("T{}", Uuid::new_v4().to_string().replace("-", ""))[..34].to_string()
    };
    
    // ایجاد درخواست واریز
    let deposit = Deposit::new(
        req.user_id,
        amount,
        deposit_address,
        req.payment_method,
        24, // انقضا بعد از 24 ساعت
    )
    .map_err(|e| ApiError::validation_error(&e))?;
    
    // ذخیره در دیتابیس
    let deposit_repo = DepositRepository::new(state.pool.clone());
    let saved_deposit = deposit_repo.create_deposit(deposit).await
        .map_err(|e| {
            error!("Failed to create deposit: {}", e);
            ApiError::internal_error("Failed to create deposit")
        })?;
    
    info!("Deposit created successfully: {}", saved_deposit.id);
    let response: DepositResponse = saved_deposit.into();
    Ok(HttpResponse::Created().json(response))
}

/// دریافت لیست واریزهای کاربر
/// 
/// # Endpoint
/// GET /api/v1/deposits/user/{user_id}
/// 
/// # Query Parameters
/// - limit: تعداد رکوردها (پیش‌فرض: 50)
/// - offset: شماره شروع (پیش‌فرض: 0)
/// 
/// # Response
/// `json
/// [
///     {
///         "id": "uuid-v4",
///         "user_id": "uuid-v4",
///         "amount": "50.00000000",
///         "deposit_address": "TXXXXXXXXXXXXXXXXXXXXXXXXXXX",
///         "payment_method": "tron_usdt",
///         "status": "confirmed",
///         "expires_at": "2026-01-04T18:40:00Z",
///         "created_at": "2026-01-03T18:40:00Z",
///         "is_expired": false,
///         "is_confirmed": true
///     }
/// ]
/// `
pub async fn get_user_deposits(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    query: web::Query<DepositQuery>,
) -> Result<impl Responder, ApiError> {
    let user_id = path.into_inner();
    info!("Getting deposits for user: {}", user_id);
    
    let repo = DepositRepository::new(state.pool.clone());
    
    let deposits = repo.get_deposits_by_user_id(
        user_id,
        query.limit,
        query.offset
    ).await
    .map_err(|e| {
        error!("Database error: {}", e);
        ApiError::internal_error("Database error")
    })?;
    
    let responses: Vec<DepositResponse> = deposits.into_iter()
        .map(|deposit| deposit.into())
        .collect();
    
    Ok(HttpResponse::Ok().json(responses))
}

/// دریافت اطلاعات واریز خاص
/// 
/// # Endpoint
/// GET /api/v1/deposits/{deposit_id}
pub async fn get_deposit(
    state: web::Data<AppState>,
    deposit_id: web::Path<Uuid>,
) -> Result<impl Responder, ApiError> {
    info!("Getting deposit: {}", deposit_id);
    
    let repo = DepositRepository::new(state.pool.clone());
    
    match repo.get_deposit_by_id(*deposit_id).await {
        Ok(Some(deposit)) => {
            let response: DepositResponse = deposit.into();
            Ok(HttpResponse::Ok().json(response))
        }
        Ok(None) => {
            warn!("Deposit not found: {}", deposit_id);
            Err(ApiError::not_found("Deposit not found"))
        }
        Err(e) => {
            error!("Database error: {}", e);
            Err(ApiError::internal_error("Database error"))
        }
    }
}

/// دریافت وضعیت واریز
/// 
/// # Endpoint
/// GET /api/v1/deposits/{deposit_id}/status
/// 
/// # Response
/// `json
/// {
///     "deposit_id": "uuid-v4",
///     "status": "pending",
///     "confirmations": 2,
///     "required_confirmations": 12,
///     "is_expired": false,
///     "is_confirmed": false,
///     "estimated_completion": "2026-01-03T18:50:00Z"
/// }
/// `
pub async fn get_deposit_status(
    state: web::Data<AppState>,
    deposit_id: web::Path<Uuid>,
) -> Result<impl Responder, ApiError> {
    info!("Getting status for deposit: {}", deposit_id);
    
    let repo = DepositRepository::new(state.pool.clone());
    
    match repo.get_deposit_by_id(*deposit_id).await {
        Ok(Some(deposit)) => {
            #[derive(serde::Serialize)]
            struct StatusResponse {
                deposit_id: Uuid,
                status: DepositStatus,
                confirmations: i32,
                required_confirmations: i32,
                is_expired: bool,
                is_confirmed: bool,
                estimated_completion: Option<chrono::DateTime<chrono::Utc>>,
            }
            
            let estimated_completion = if deposit.confirmations > 0 {
                // تخمین زمان تکمیل بر اساس میانگین زمان بلاک
                let blocks_remaining = (deposit.required_confirmations - deposit.confirmations) as i64;
                let estimated_seconds = blocks_remaining * 3; // هر بلاک TRON ≈ 3 ثانیه
                Some(chrono::Utc::now() + chrono::Duration::seconds(estimated_seconds))
            } else {
                None
            };
            
            let response = StatusResponse {
                deposit_id: deposit.id,
                status: deposit.status,
                confirmations: deposit.confirmations,
                required_confirmations: deposit.required_confirmations,
                is_expired: deposit.is_expired(),
                is_confirmed: deposit.is_confirmed(),
                estimated_completion,
            };
            
            Ok(HttpResponse::Ok().json(response))
        }
        Ok(None) => {
            warn!("Deposit not found: {}", deposit_id);
            Err(ApiError::not_found("Deposit not found"))
        }
        Err(e) => {
            error!("Database error: {}", e);
            Err(ApiError::internal_error("Database error"))
        }
    }
}

/// ساختار query parameters برای لیست واریزها
#[derive(serde::Deserialize)]
pub struct DepositQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

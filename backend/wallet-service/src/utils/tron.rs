//! ابزارهای مربوط به شبکه TRON
//!
//! این ماژول شامل توابعی برای تعامل با شبکه TRON است

use std::str::FromStr;
use tracing::{info, warn, error};
use crate::errors::ApiError;

/// ساختار پیکربندی TRON
#[derive(Debug, Clone)]
pub struct TronConfig {
    pub node_url: String,
    pub api_key: String,
    pub network: TronNetwork,
}

/// نوع شبکه TRON
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TronNetwork {
    Mainnet,
    Testnet,
    ShastaTestnet,
}

impl TronConfig {
    /// ایجاد پیکربندی از متغیرهای محیطی
    pub fn from_env() -> Result<Self, String> {
        dotenv::dotenv().ok();
        
        let node_url = std::env::var("TRON_NODE_URL")
            .unwrap_or_else(|_| "https://api.trongrid.io".to_string());
        
        let api_key = std::env::var("TRON_API_KEY")
            .map_err(|_| "TRON_API_KEY not set in environment".to_string())?;
        
        let network_str = std::env::var("TRON_NETWORK")
            .unwrap_or_else(|_| "mainnet".to_string());
        
        let network = match network_str.to_lowercase().as_str() {
            "mainnet" => TronNetwork::Mainnet,
            "testnet" => TronNetwork::Testnet,
            "shasta" => TronNetwork::ShastaTestnet,
            _ => {
                warn!("Unknown TRON network '{}', defaulting to mainnet", network_str);
                TronNetwork::Mainnet
            }
        };
        
        Ok(Self {
            node_url,
            api_key,
            network,
        })
    }
    
    /// بررسی اینکه آیا شبکه تست است
    pub fn is_testnet(&self) -> bool {
        matches!(self.network, TronNetwork::Testnet | TronNetwork::ShastaTestnet)
    }
}

/// تولید آدرس TRON جدید
/// 
/// در نسخه واقعی، این تابع با کیف پول TRON ارتباط برقرار می‌کند
/// فعلاً یک آدرس mock تولید می‌کند
pub async fn generate_tron_address(config: &TronConfig) -> Result<String, ApiError> {
    info!("Generating new TRON address on network: {:?}", config.network);
    
    // در نسخه واقعی:
    // 1. ارتباط با TRON node
    // 2. تولید کلید خصوصی/عمومی
    // 3. تبدیل به آدرس TRON
    
    // فعلاً یک آدرس mock تولید می‌کنیم
    let prefix = if config.is_testnet() { "T" } else { "T" };
    let random_part: String = (0..33)
        .map(|_| rand::random::<char>())
        .filter(|c| c.is_alphanumeric())
        .take(33)
        .collect();
    
    let address = format!("{}{}", prefix, random_part);
    
    // اعتبارسنجی فرمت آدرس
    validate_tron_address(&address)?;
    
    info!("Generated TRON address: {}", address);
    Ok(address)
}

/// اعتبارسنجی فرمت آدرس TRON
pub fn validate_tron_address(address: &str) -> Result<(), ApiError> {
    // آدرس TRON باید:
    // 1. با 'T' شروع شود
    // 2. طول آن 34 کاراکتر باشد
    // 3. فقط حروف و اعداد داشته باشد
    
    if !address.starts_with('T') {
        return Err(ApiError::validation_error("TRON address must start with 'T'"));
    }
    
    if address.len() != 34 {
        return Err(ApiError::validation_error(
            &format!("TRON address must be 34 characters long, got {}", address.len())
        ));
    }
    
    if !address[1..].chars().all(|c| c.is_alphanumeric()) {
        return Err(ApiError::validation_error(
            "TRON address can only contain alphanumeric characters"
        ));
    }
    
    Ok(())
}

/// بررسی موجودی یک آدرس TRON
pub async fn check_tron_balance(
    config: &TronConfig,
    address: &str,
    token: Option<&str>,
) -> Result<String, ApiError> {
    info!("Checking balance for address: {}", address);
    
    // اعتبارسنجی آدرس
    validate_tron_address(address)?;
    
    // در نسخه واقعی:
    // 1. ارسال درخواست به TRON API
    // 2. دریافت موجودی
    // 3. پردازش پاسخ
    
    // فعلاً مقدار mock برمی‌گردانیم
    Ok("0.00000000".to_string())
}

/// ارسال تراکنش TRON
pub async fn send_tron_transaction(
    config: &TronConfig,
    from_address: &str,
    to_address: &str,
    amount: &str,
    private_key: &str,
) -> Result<String, ApiError> {
    info!("Sending TRON transaction: {} -> {} amount: {}", from_address, to_address, amount);
    
    // اعتبارسنجی آدرس‌ها
    validate_tron_address(from_address)?;
    validate_tron_address(to_address)?;
    
    // اعتبارسنجی مبلغ
    let amount_decimal = BigDecimal::from_str(amount)
        .map_err(|_| ApiError::validation_error("Invalid amount format"))?;
    
    if amount_decimal <= BigDecimal::from(0) {
        return Err(ApiError::validation_error("Amount must be greater than 0"));
    }
    
    // در نسخه واقعی:
    // 1. ساخت تراکنش
    // 2. امضای تراکنش
    // 3. ارسال به شبکه
    // 4. دریافت hash تراکنش
    
    // فعلاً یک hash mock برمی‌گردانیم
    let tx_hash = format!("0x{}", uuid::Uuid::new_v4().to_string().replace("-", ""));
    
    info!("Transaction sent successfully, hash: {}", tx_hash);
    Ok(tx_hash)
}

/// بررسی وضعیت تراکنش
pub async fn check_transaction_status(
    config: &TronConfig,
    transaction_hash: &str,
) -> Result<TransactionStatus, ApiError> {
    info!("Checking status for transaction: {}", transaction_hash);
    
    // در نسخه واقعی:
    // 1. درخواست به TRON API
    // 2. بررسی تأییدیه‌ها
    // 3. برگرداندن وضعیت
    
    // فعلاً وضعیت mock برمی‌گردانیم
    Ok(TransactionStatus {
        confirmed: true,
        confirmations: 12,
        block_number: Some(12345678),
        timestamp: Some(chrono::Utc::now()),
        success: true,
    })
}

/// وضعیت تراکنش
#[derive(Debug, serde::Serialize)]
pub struct TransactionStatus {
    pub confirmed: bool,
    pub confirmations: i32,
    pub block_number: Option<i64>,
    pub timestamp: Option<chrono::DateTime<chrono::Utc>>,
    pub success: bool,
}

/// تبدیل TRX به Sun (واحد پایه TRON)
pub fn trx_to_sun(trx_amount: &str) -> Result<String, ApiError> {
    let amount = BigDecimal::from_str(trx_amount)
        .map_err(|_| ApiError::validation_error("Invalid TRX amount"))?;
    
    // 1 TRX = 1,000,000 SUN
    let sun_amount = amount * BigDecimal::from(1_000_000);
    
    Ok(sun_amount.to_string())
}

/// تبدیل Sun به TRX
pub fn sun_to_trx(sun_amount: &str) -> Result<String, ApiError> {
    let amount = BigDecimal::from_str(sun_amount)
        .map_err(|_| ApiError::validation_error("Invalid SUN amount"))?;
    
    // 1 SUN = 0.000001 TRX
    let trx_amount = amount / BigDecimal::from(1_000_000);
    
    Ok(trx_amount.to_string())
}

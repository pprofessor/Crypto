//! توابع اعتبارسنجی
//!
//! این ماژول شامل توابعی برای اعتبارسنجی داده‌های ورودی است

use uuid::Uuid;
use regex::Regex;
use lazy_static::lazy_static;
use crate::errors::ApiError;

lazy_static! {
    /// regex برای اعتبارسنجی ایمیل
    static ref EMAIL_REGEX: Regex = Regex::new(
        r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"
    ).unwrap();
    
    /// regex برای اعتبارسنجی پسورد قوی
    static ref PASSWORD_REGEX: Regex = Regex::new(
        r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,}$"
    ).unwrap();
}

/// اعتبارسنجی آدرس ایمیل
pub fn validate_email(email: &str) -> Result<(), ApiError> {
    if email.is_empty() {
        return Err(ApiError::validation_error("Email cannot be empty"));
    }
    
    if email.len() > 255 {
        return Err(ApiError::validation_error("Email is too long"));
    }
    
    if !EMAIL_REGEX.is_match(email) {
        return Err(ApiError::validation_error("Invalid email format"));
    }
    
    Ok(())
}

/// اعتبارسنجی پسورد
pub fn validate_password(password: &str) -> Result<(), ApiError> {
    if password.is_empty() {
        return Err(ApiError::validation_error("Password cannot be empty"));
    }
    
    if password.len() < 8 {
        return Err(ApiError::validation_error("Password must be at least 8 characters"));
    }
    
    if password.len() > 128 {
        return Err(ApiError::validation_error("Password is too long"));
    }
    
    // بررسی پیچیدگی پسورد (اختیاری)
    // if !PASSWORD_REGEX.is_match(password) {
    //     return Err(ApiError::validation_error(
    //         "Password must contain uppercase, lowercase, number and special character"
    //     ));
    // }
    
    Ok(())
}

/// اعتبارسنجی مقدار پول
pub fn validate_amount(amount: &str, min: Option<&str>, max: Option<&str>) -> Result<(), ApiError> {
    use bigdecimal::BigDecimal;
    use std::str::FromStr;
    
    let amount_decimal = BigDecimal::from_str(amount)
        .map_err(|_| ApiError::validation_error("Invalid amount format"))?;
    
    if amount_decimal <= BigDecimal::from(0) {
        return Err(ApiError::validation_error("Amount must be greater than 0"));
    }
    
    if let Some(min_str) = min {
        let min_decimal = BigDecimal::from_str(min_str)
            .map_err(|_| ApiError::validation_error("Invalid minimum amount"))?;
        
        if amount_decimal < min_decimal {
            return Err(ApiError::validation_error(
                &format!("Amount must be at least {}", min_str)
            ));
        }
    }
    
    if let Some(max_str) = max {
        let max_decimal = BigDecimal::from_str(max_str)
            .map_err(|_| ApiError::validation_error("Invalid maximum amount"))?;
        
        if amount_decimal > max_decimal {
            return Err(ApiError::validation_error(
                &format!("Amount cannot exceed {}", max_str)
            ));
        }
    }
    
    Ok(())
}

/// اعتبارسنجی UUID
pub fn validate_uuid(uuid_str: &str) -> Result<Uuid, ApiError> {
    Uuid::parse_str(uuid_str)
        .map_err(|_| ApiError::validation_error("Invalid UUID format"))
}

/// اعتبارسنجی محدوده عددی
pub fn validate_range(value: i64, min: i64, max: i64, field_name: &str) -> Result<(), ApiError> {
    if value < min {
        return Err(ApiError::validation_error(
            &format!("{} must be at least {}", field_name, min)
        ));
    }
    
    if value > max {
        return Err(ApiError::validation_error(
            &format!("{} cannot exceed {}", field_name, max)
        ));
    }
    
    Ok(())
}

/// اعتبارسنجی طول رشته
pub fn validate_string_length(value: &str, min: usize, max: usize, field_name: &str) -> Result<(), ApiError> {
    let length = value.len();
    
    if length < min {
        return Err(ApiError::validation_error(
            &format!("{} must be at least {} characters", field_name, min)
        ));
    }
    
    if length > max {
        return Err(ApiError::validation_error(
            &format!("{} cannot exceed {} characters", field_name, max)
        ));
    }
    
    Ok(())
}

/// اعتبارسنجی آدرس بلاکچین (عمومی)
pub fn validate_blockchain_address(address: &str, blockchain: &str) -> Result<(), ApiError> {
    match blockchain.to_lowercase().as_str() {
        "tron" => {
            if !address.starts_with('T') {
                return Err(ApiError::validation_error(
                    "TRON address must start with 'T'"
                ));
            }
            
            if address.len() != 34 {
                return Err(ApiError::validation_error(
                    "TRON address must be 34 characters long"
                ));
            }
        }
        "bitcoin" => {
            if !address.starts_with('1') && !address.starts_with('3') && !address.starts_with('bc1') {
                return Err(ApiError::validation_error(
                    "Invalid Bitcoin address format"
                ));
            }
        }
        "ethereum" => {
            if !address.starts_with("0x") {
                return Err(ApiError::validation_error(
                    "Ethereum address must start with '0x'"
                ));
            }
            
            if address.len() != 42 {
                return Err(ApiError::validation_error(
                    "Ethereum address must be 42 characters long"
                ));
            }
        }
        _ => {
            return Err(ApiError::validation_error(
                &format!("Unsupported blockchain: {}", blockchain)
            ));
        }
    }
    
    Ok(())
}

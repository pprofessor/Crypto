//! توابع رمزنگاری
//!
//! این ماژول شامل توابعی برای رمزنگاری و هش کردن است

use argon2::{self, Config, ThreadMode, Variant, Version};
use rand::Rng;
use tracing::{info, warn};

/// ساختار پیکربندی برای Argon2
#[derive(Debug, Clone)]
pub struct Argon2Config {
    pub salt_length: usize,
    pub hash_length: usize,
    pub iterations: u32,
    pub memory_size: u32,
    pub parallelism: u32,
}

impl Default for Argon2Config {
    fn default() -> Self {
        Self {
            salt_length: 16,
            hash_length: 32,
            iterations: 3,
            memory_size: 4096, // 4MB
            parallelism: 1,
        }
    }
}

/// هش کردن پسورد با Argon2
pub fn hash_password(password: &str) -> Result<String, String> {
    let config = Argon2Config::default();
    
    // تولید salt تصادفی
    let salt: Vec<u8> = rand::thread_rng()
        .sample_iter(rand::distributions::Standard)
        .take(config.salt_length)
        .collect();
    
    // پیکربندی Argon2
    let argon2_config = Config {
        variant: Variant::Argon2id,
        version: Version::Version13,
        mem_cost: config.memory_size,
        time_cost: config.iterations,
        lanes: config.parallelism,
        thread_mode: ThreadMode::Parallel,
        secret: &[],
        ad: &[],
        hash_length: config.hash_length,
    };
    
    // هش کردن
    let hash = argon2::hash_encoded(password.as_bytes(), &salt, &argon2_config)
        .map_err(|e| format!("Failed to hash password: {}", e))?;
    
    info!("Password hashed successfully");
    Ok(hash)
}

/// تأیید پسورد با هش ذخیره شده
pub fn verify_password(password: &str, hashed_password: &str) -> Result<bool, String> {
    let is_valid = argon2::verify_encoded(hashed_password, password.as_bytes())
        .map_err(|e| format!("Failed to verify password: {}", e))?;
    
    if is_valid {
        info!("Password verified successfully");
    } else {
        warn!("Password verification failed");
    }
    
    Ok(is_valid)
}

/// تولید توکن تصادفی
pub fn generate_random_token(length: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789";
    
    let mut rng = rand::thread_rng();
    
    let token: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    
    token
}

/// تولید شناسه تراکنش
pub fn generate_transaction_id() -> String {
    format!("TX{}", uuid::Uuid::new_v4().to_string().replace("-", ""))
}

/// کد تأیید تصادفی
pub fn generate_verification_code(length: usize) -> String {
    let mut rng = rand::thread_rng();
    
    let code: String = (0..length)
        .map(|_| rng.gen_range(0..10).to_string())
        .collect();
    
    code
}

/// رمزنگاری رشته (برای داده‌های حساس)
pub fn encrypt_string(plaintext: &str, key: &[u8]) -> Result<Vec<u8>, String> {
    use aes_gcm::{
        aead::{Aead, KeyInit},
        Aes256Gcm, Nonce,
    };
    
    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|e| format!("Failed to create cipher: {}", e))?;
    
    // تولید nonce تصادفی
    let nonce_bytes: [u8; 12] = rand::thread_rng().gen();
    let nonce = Nonce::from_slice(&nonce_bytes);
    
    cipher.encrypt(nonce, plaintext.as_bytes())
        .map_err(|e| format!("Failed to encrypt: {}", e))
}

/// رمزگشایی رشته
pub fn decrypt_string(ciphertext: &[u8], key: &[u8]) -> Result<String, String> {
    use aes_gcm::{
        aead::{Aead, KeyInit},
        Aes256Gcm, Nonce,
    };
    
    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|e| format!("Failed to create cipher: {}", e))?;
    
    // nonce باید همراه ciphertext ذخیره شود
    // در این مثال فرض می‌کنیم ۱۲ بایت اول nonce است
    if ciphertext.len() < 12 {
        return Err("Invalid ciphertext length".to_string());
    }
    
    let nonce = Nonce::from_slice(&ciphertext[0..12]);
    let encrypted_data = &ciphertext[12..];
    
    let decrypted = cipher.decrypt(nonce, encrypted_data)
        .map_err(|e| format!("Failed to decrypt: {}", e))?;
    
    String::from_utf8(decrypted)
        .map_err(|e| format!("Failed to convert to string: {}", e))
}

/// تولید کلید تصادفی برای رمزنگاری
pub fn generate_encryption_key() -> Vec<u8> {
    let mut key = vec![0u8; 32]; // 256-bit key for AES-256
    rand::thread_rng().fill(&mut key[..]);
    key
}

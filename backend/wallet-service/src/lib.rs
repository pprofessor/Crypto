//! Wallet Service Library
//!
//! This module provides the core functionality for the cryptocurrency wallet management microservice.

// اکسپورت کردن ماژول‌های داخلی به عنوان public
pub mod api;
pub mod config;
pub mod database;
pub mod errors;
pub mod models;
pub mod repositories;
pub mod services;
pub mod utils;

// Re-export برخی از انواع پرکاربرد در سطح کریت برای راحتی استفاده
pub use config::Config;
pub use errors::ServiceError;
//! Wallet Service - سرویس مدیریت کیف پول‌های ارز دیجیتال
//! 
//! این سرویس مسئولیت‌های زیر را بر عهده دارد:
//! 1. ایجاد و مدیریت کیف پول‌های کاربران
//! 2. پیگیری موجودی USDT
//! 3. تولید آدرس‌های واریز TRON
//! 4. ارائه API برای تعامل با کیف پول‌ها

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use dotenv::dotenv;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;
use tracing_subscriber;
use tracing::info;

// Import modules
mod entities;
mod handlers;
mod database;
mod errors;
mod utils;

use handlers::{wallet_handler, health_handler};
use errors::ApiError;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    // Load environment variables
    dotenv().ok();
    info!("Wallet Service starting...");
    
    // Read configuration from environment
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");
    
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8081".to_string());
    let port = port.parse::<u16>().expect("PORT must be a valid number");
    
    // Create database connection pool
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to create database pool");
    
    info!("Database connection pool created");
    
    // Run database migrations
    info!("Running database migrations...");
    database::run_migrations(&pool).await
        .expect("Failed to run database migrations");
    
    // Create shared application state
    let app_state = web::Data::new(AppState { pool });
    
    info!("Starting HTTP server on {}:{}", host, port);
    
    // Start HTTP server
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                actix_web::http::header::CONTENT_TYPE,
                actix_web::http::header::AUTHORIZATION,
            ])
            .max_age(3600);
        
        App::new()
            .wrap(cors)
            .app_data(app_state.clone())
            .configure(configure_routes)
    })
    .bind((host.as_str(), port))?
    .run()
    .await
}

/// Application state shared across all requests
#[derive(Debug, Clone)]
pub struct AppState {
    pub pool: PgPool,
}

/// Configure API routes
fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg
        // Health check endpoint
        .service(
            web::scope("/health")
                .route("", web::get().to(health_handler::health_check))
        )
        // Wallet management endpoints
        .service(
            web::scope("/api/v1/wallets")
                .route("", web::post().to(wallet_handler::create_wallet))
                .route("/{user_id}", web::get().to(wallet_handler::get_wallet))
                .route("/{user_id}/balance", web::get().to(wallet_handler::get_balance))
        )
        // Deposit endpoints
        .service(
            web::scope("/api/v1/deposits")
                .route("", web::post().to(wallet_handler::create_deposit))
                .route("/user/{user_id}", web::get().to(wallet_handler::get_user_deposits))
                .route("/{deposit_id}", web::get().to(wallet_handler::get_deposit))
                .route("/{deposit_id}/status", web::get().to(wallet_handler::get_deposit_status))
        );
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, http};
    
    #[actix_rt::test]
    async fn test_health_check() {
        let mut app = test::init_service(
            App::new().route("/health", web::get().to(health_handler::health_check))
        ).await;
        
        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&mut app, req).await;
        
        assert_eq!(resp.status(), http::StatusCode::OK);
    }
}

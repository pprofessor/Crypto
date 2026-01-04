use actix_web::{ middleware, web, App, HttpServer };
use tracing::info;
use tracing_subscriber;

// import ماژول‌ها با استفاده از نام کریت (wallet_service)
use wallet_service::api;
use wallet_service::config::Config;
use wallet_service::repositories::WalletRepository;
use wallet_service::services::WalletService;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();
    info!("Starting Wallet Service...");

    let config = Config::new().expect("Failed to load configuration");
    info!("Configuration loaded. Server will run on: {}", config.server_address());

    // ایجاد connection pool - نوع آن به طور خودکار از بازگشت تابع infer می‌شود
    let db_pool = wallet_service::database
        ::create_pool(&config).await
        .expect("Failed to create database pool");
    info!("Database connection pool established.");

    wallet_service::database
        ::check_database(&db_pool).await
        .expect("Failed initial database check. Please ensure the 'crypto.wallets' table exists.");

    let wallet_repository = WalletRepository::new(db_pool.clone());
    let wallet_service = WalletService::new(wallet_repository);

    info!("Configuring HTTP server...");
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(web::Data::new(wallet_service.clone()))
            .wrap(middleware::Logger::default())
            .configure(api::config_routes)
    })
        .bind(config.server_address())?
        .run();

    info!("Wallet Service is running on {}", config.server_address());
    info!("Health check available at: http://{}/api/health", config.server_address());

    server.await
}

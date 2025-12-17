mod config;
mod error;
mod handlers;
mod models;
mod repositories;
mod routes;
mod services;
mod state;

use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tracing::{info, warn};

use crate::config::{create_cors_layer, load_config};
use crate::error::AppError;
use crate::repositories::{
    PgCustomerRepository, PgOrderRepository, PgProductRepository, PgSellerRepository,
};
use crate::services::{CustomerService, OrderService, ProductService, SellerService};
use crate::state::AppState;

#[tokio::main]
async fn main() -> std::result::Result<(), AppError> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let config = load_config()?;
    let cors_layer = create_cors_layer(config.cors.clone());

    info!("Connecting to database at {}...", config.database_url);

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&config.database_url)
        .await
        .map_err(AppError::DatabaseError)?;

    info!("Database connection pool created.");

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(AppError::MigrationError)?;

    let app_state = AppState {
        customer_service: CustomerService::new(Arc::new(PgCustomerRepository::new(pool.clone()))),
        seller_service: SellerService::new(Arc::new(PgSellerRepository::new(pool.clone()))),
        order_service: OrderService::new(Arc::new(PgOrderRepository::new(pool.clone()))),
        product_service: ProductService::new(Arc::new(PgProductRepository::new(pool.clone()))),
    };

    let app = crate::routes::create_router(app_state).layer(cors_layer);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    info!("Server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|e| AppError::ConfigError(format!("Failed to bind TCP listener: {}", e)))?;

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .map_err(|e| AppError::ConfigError(format!("Axum server failed: {}", e)))?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    warn!("Signal received, starting graceful shutdown...");
}

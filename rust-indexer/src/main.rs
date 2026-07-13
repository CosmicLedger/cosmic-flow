use actix_web::{web, App, HttpServer, middleware};
use tracing::{info, error};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod config;
mod db;
mod error;
mod indexer;
mod models;
mod services;

use crate::config::Config;
use crate::db::Database;
use crate::error::Result;

#[actix_web::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "stellar_chain_indexer=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("🚀 Starting Stellar Chain Indexer");

    // Load configuration
    let config = Config::from_env()?;
    info!("Configuration loaded successfully");

    // Initialize database
    let db = Database::new(&config.database_url).await?;
    info!("Database connection established");

    // Run migrations
    db.run_migrations().await?;
    info!("Database migrations completed");

    // Initialize indexer
    let indexer = indexer::Indexer::new(config.clone(), db.clone());
    
    // Start indexing in background
    tokio::spawn(async move {
        if let Err(e) = indexer.start().await {
            error!("Indexer error: {:?}", e);
        }
    });

    // Start HTTP server
    let server_addr = format!("{}:{}", config.server_host, config.server_port);
    info!("Starting HTTP server on {}", server_addr);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .wrap(api::cors())
            .configure(api::routes::configure)
    })
    .bind(&server_addr)?
    .run()
    .await
    .map_err(Into::into)
}

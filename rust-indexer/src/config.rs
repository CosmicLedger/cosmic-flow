use serde::Deserialize;
use crate::error::{Result, IndexerError};

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub horizon_url: String,
    pub server_host: String,
    pub server_port: u16,
    pub redis_url: Option<String>,
    pub enable_metrics: bool,
    pub indexer_start_ledger: Option<u64>,
    pub indexer_batch_size: usize,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenv::dotenv().ok();

        Ok(Config {
            database_url: std::env::var("DATABASE_URL")
                .map_err(|_| IndexerError::Config("DATABASE_URL not set".into()))?,
            
            horizon_url: std::env::var("HORIZON_URL")
                .unwrap_or_else(|_| "https://horizon.stellar.org".to_string()),
            
            server_host: std::env::var("SERVER_HOST")
                .unwrap_or_else(|_| "0.0.0.0".to_string()),
            
            server_port: std::env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .map_err(|_| IndexerError::Config("Invalid SERVER_PORT".into()))?,
            
            redis_url: std::env::var("REDIS_URL").ok(),
            
            enable_metrics: std::env::var("ENABLE_METRICS")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),
            
            indexer_start_ledger: std::env::var("INDEXER_START_LEDGER")
                .ok()
                .and_then(|s| s.parse().ok()),
            
            indexer_batch_size: std::env::var("INDEXER_BATCH_SIZE")
                .unwrap_or_else(|_| "100".to_string())
                .parse()
                .unwrap_or(100),
        })
    }
}

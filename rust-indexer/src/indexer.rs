use tracing::{info, warn, error};
use tokio::time::{sleep, Duration};
use crate::{config::Config, db::Database, error::Result};

pub struct Indexer {
    config: Config,
    db: Database,
    http_client: reqwest::Client,
}

impl Indexer {
    pub fn new(config: Config, db: Database) -> Self {
        let http_client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Indexer {
            config,
            db,
            http_client,
        }
    }

    pub async fn start(&self) -> Result<()> {
        info!("Starting blockchain indexer");

        let mut current_ledger = self.get_start_ledger().await?;
        
        loop {
            match self.index_ledger(current_ledger).await {
                Ok(_) => {
                    info!("Successfully indexed ledger {}", current_ledger);
                    current_ledger += 1;
                }
                Err(e) => {
                    error!("Error indexing ledger {}: {:?}", current_ledger, e);
                    sleep(Duration::from_secs(5)).await;
                }
            }

            // Small delay to avoid overwhelming the API
            sleep(Duration::from_millis(100)).await;
        }
    }

    async fn get_start_ledger(&self) -> Result<u64> {
        if let Some(start) = self.config.indexer_start_ledger {
            return Ok(start);
        }

        // Get the latest indexed ledger from database
        let latest = sqlx::query_scalar::<_, Option<i64>>(
            "SELECT MAX(sequence) FROM ledgers"
        )
        .fetch_one(self.db.pool())
        .await?;

        Ok(latest.map(|l| l as u64 + 1).unwrap_or(1))
    }

    async fn index_ledger(&self, sequence: u64) -> Result<()> {
        let url = format!("{}/ledgers/{}", self.config.horizon_url, sequence);
        
        let response = self.http_client
            .get(&url)
            .send()
            .await?;

        if !response.status().is_success() {
            warn!("Ledger {} not yet available", sequence);
            return Ok(());
        }

        let ledger_data: serde_json::Value = response.json().await?;

        // Store ledger in database
        self.store_ledger(&ledger_data).await?;

        // Index transactions for this ledger
        self.index_transactions(sequence).await?;

        Ok(())
    }

    async fn store_ledger(&self, data: &serde_json::Value) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO ledgers (
                sequence, hash, previous_hash, transaction_count,
                operation_count, closed_at, total_coins, fee_pool,
                base_fee, base_reserve, max_tx_set_size, protocol_version
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            ON CONFLICT (sequence) DO NOTHING
            "#
        )
        .bind(data["sequence"].as_i64().unwrap_or(0))
        .bind(data["hash"].as_str().unwrap_or(""))
        .bind(data["prev_hash"].as_str().unwrap_or(""))
        .bind(data["transaction_count"].as_i64().unwrap_or(0) as i32)
        .bind(data["operation_count"].as_i64().unwrap_or(0) as i32)
        .bind(data["closed_at"].as_str().unwrap_or(""))
        .bind(data["total_coins"].as_str().unwrap_or("0"))
        .bind(data["fee_pool"].as_str().unwrap_or("0"))
        .bind(data["base_fee_in_stroops"].as_i64().unwrap_or(100) as i32)
        .bind(data["base_reserve_in_stroops"].as_i64().unwrap_or(5000000) as i32)
        .bind(data["max_tx_set_size"].as_i64().unwrap_or(1000) as i32)
        .bind(data["protocol_version"].as_i64().unwrap_or(20) as i32)
        .execute(self.db.pool())
        .await?;

        Ok(())
    }

    async fn index_transactions(&self, ledger_sequence: u64) -> Result<()> {
        let url = format!(
            "{}/ledgers/{}/transactions?limit=200",
            self.config.horizon_url, ledger_sequence
        );

        let response = self.http_client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Ok(());
        }

        let data: serde_json::Value = response.json().await?;
        let transactions = data["_embedded"]["records"].as_array().unwrap_or(&vec![]);

        for tx in transactions {
            self.store_transaction(tx).await?;
        }

        Ok(())
    }

    async fn store_transaction(&self, data: &serde_json::Value) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO transactions (
                hash, ledger_sequence, source_account, fee_charged,
                operation_count, successful
            ) VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (hash) DO NOTHING
            "#
        )
        .bind(data["hash"].as_str().unwrap_or(""))
        .bind(data["ledger"].as_i64().unwrap_or(0))
        .bind(data["source_account"].as_str().unwrap_or(""))
        .bind(data["fee_charged"].as_str().unwrap_or("0").parse::<i64>().unwrap_or(0))
        .bind(data["operation_count"].as_i64().unwrap_or(0) as i32)
        .bind(data["successful"].as_bool().unwrap_or(false))
        .execute(self.db.pool())
        .await?;

        Ok(())
    }
}

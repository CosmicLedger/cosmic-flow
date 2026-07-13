use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Ledger {
    pub id: i64,
    pub sequence: i64,
    pub hash: String,
    pub previous_hash: String,
    pub transaction_count: i32,
    pub operation_count: i32,
    pub closed_at: DateTime<Utc>,
    pub total_coins: String,
    pub fee_pool: String,
    pub base_fee: i32,
    pub base_reserve: i32,
    pub max_tx_set_size: i32,
    pub protocol_version: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Transaction {
    pub id: i64,
    pub hash: String,
    pub ledger_sequence: i64,
    pub source_account: String,
    pub fee_charged: i64,
    pub operation_count: i32,
    pub successful: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Operation {
    pub id: i64,
    pub transaction_hash: String,
    pub operation_type: String,
    pub source_account: Option<String>,
    pub details: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Account {
    pub id: String,
    pub sequence: String,
    pub balance: String,
    pub last_modified_ledger: i64,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub uptime: u64,
    pub indexed_ledgers: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatsResponse {
    pub total_ledgers: i64,
    pub total_transactions: i64,
    pub total_operations: i64,
    pub total_accounts: i64,
    pub latest_ledger: Option<i64>,
}

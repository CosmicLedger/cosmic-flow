use crate::{db::Database, error::{Result, IndexerError}, models::*};
use sqlx::Row;

pub struct LedgerService;
pub struct TransactionService;
pub struct StatsService;

impl LedgerService {
    pub async fn get_latest(db: &Database, limit: i64) -> Result<Vec<Ledger>> {
        let ledgers = sqlx::query_as::<_, Ledger>(
            "SELECT * FROM ledgers ORDER BY sequence DESC LIMIT $1"
        )
        .bind(limit)
        .fetch_all(db.pool())
        .await?;

        Ok(ledgers)
    }

    pub async fn get_by_sequence(db: &Database, sequence: i64) -> Result<Ledger> {
        let ledger = sqlx::query_as::<_, Ledger>(
            "SELECT * FROM ledgers WHERE sequence = $1"
        )
        .bind(sequence)
        .fetch_optional(db.pool())
        .await?
        .ok_or_else(|| IndexerError::NotFound(format!("Ledger {} not found", sequence)))?;

        Ok(ledger)
    }

    pub async fn get_by_hash(db: &Database, hash: &str) -> Result<Ledger> {
        let ledger = sqlx::query_as::<_, Ledger>(
            "SELECT * FROM ledgers WHERE hash = $1"
        )
        .bind(hash)
        .fetch_optional(db.pool())
        .await?
        .ok_or_else(|| IndexerError::NotFound(format!("Ledger {} not found", hash)))?;

        Ok(ledger)
    }
}

impl TransactionService {
    pub async fn get_latest(db: &Database, limit: i64) -> Result<Vec<Transaction>> {
        let transactions = sqlx::query_as::<_, Transaction>(
            "SELECT * FROM transactions ORDER BY id DESC LIMIT $1"
        )
        .bind(limit)
        .fetch_all(db.pool())
        .await?;

        Ok(transactions)
    }

    pub async fn get_by_hash(db: &Database, hash: &str) -> Result<Transaction> {
        let transaction = sqlx::query_as::<_, Transaction>(
            "SELECT * FROM transactions WHERE hash = $1"
        )
        .bind(hash)
        .fetch_optional(db.pool())
        .await?
        .ok_or_else(|| IndexerError::NotFound(format!("Transaction {} not found", hash)))?;

        Ok(transaction)
    }

    pub async fn get_by_ledger(db: &Database, ledger_sequence: i64) -> Result<Vec<Transaction>> {
        let transactions = sqlx::query_as::<_, Transaction>(
            "SELECT * FROM transactions WHERE ledger_sequence = $1 ORDER BY id"
        )
        .bind(ledger_sequence)
        .fetch_all(db.pool())
        .await?;

        Ok(transactions)
    }
}

impl StatsService {
    pub async fn get_stats(db: &Database) -> Result<StatsResponse> {
        let row = sqlx::query(
            r#"
            SELECT
                (SELECT COUNT(*) FROM ledgers) as total_ledgers,
                (SELECT COUNT(*) FROM transactions) as total_transactions,
                (SELECT COUNT(*) FROM operations) as total_operations,
                (SELECT COUNT(*) FROM accounts) as total_accounts,
                (SELECT MAX(sequence) FROM ledgers) as latest_ledger
            "#
        )
        .fetch_one(db.pool())
        .await?;

        Ok(StatsResponse {
            total_ledgers: row.get("total_ledgers"),
            total_transactions: row.get("total_transactions"),
            total_operations: row.get("total_operations"),
            total_accounts: row.get("total_accounts"),
            latest_ledger: row.get("latest_ledger"),
        })
    }
}

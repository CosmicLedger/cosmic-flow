use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use crate::{
    db::Database,
    error::Result,
    models::HealthResponse,
    services::{LedgerService, TransactionService, StatsService},
};

#[derive(Deserialize)]
pub struct PaginationQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub async fn root() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "name": "Stellar Chain Indexer API",
        "version": env!("CARGO_PKG_VERSION"),
        "organization": "CosmicLedger",
        "endpoints": {
            "health": "/health",
            "stats": "/stats",
            "ledgers": "/ledgers",
            "transactions": "/transactions"
        }
    }))
}

pub async fn health() -> impl Responder {
    HttpResponse::Ok().json(HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime: 0, // TODO: Track actual uptime
        indexed_ledgers: 0, // TODO: Get from database
    })
}

pub async fn stats(db: web::Data<Database>) -> Result<impl Responder> {
    let stats = StatsService::get_stats(&db).await?;
    Ok(HttpResponse::Ok().json(stats))
}

pub async fn get_ledgers(
    db: web::Data<Database>,
    query: web::Query<PaginationQuery>,
) -> Result<impl Responder> {
    let limit = query.limit.unwrap_or(20).min(100);
    let ledgers = LedgerService::get_latest(&db, limit).await?;
    Ok(HttpResponse::Ok().json(ledgers))
}

pub async fn get_ledger_by_sequence(
    db: web::Data<Database>,
    path: web::Path<i64>,
) -> Result<impl Responder> {
    let sequence = path.into_inner();
    let ledger = LedgerService::get_by_sequence(&db, sequence).await?;
    Ok(HttpResponse::Ok().json(ledger))
}

pub async fn get_transactions(
    db: web::Data<Database>,
    query: web::Query<PaginationQuery>,
) -> Result<impl Responder> {
    let limit = query.limit.unwrap_or(20).min(100);
    let transactions = TransactionService::get_latest(&db, limit).await?;
    Ok(HttpResponse::Ok().json(transactions))
}

pub async fn get_transaction_by_hash(
    db: web::Data<Database>,
    path: web::Path<String>,
) -> Result<impl Responder> {
    let hash = path.into_inner();
    let transaction = TransactionService::get_by_hash(&db, &hash).await?;
    Ok(HttpResponse::Ok().json(transaction))
}

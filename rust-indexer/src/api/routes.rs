use actix_web::web;
use super::handlers;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
        // Health check
        .route("/health", web::get().to(handlers::health))
        .route("/", web::get().to(handlers::root))
        
        // Stats
        .route("/stats", web::get().to(handlers::stats))
        
        // Ledgers
        .service(
            web::scope("/ledgers")
                .route("", web::get().to(handlers::get_ledgers))
                .route("/{sequence}", web::get().to(handlers::get_ledger_by_sequence))
        )
        
        // Transactions
        .service(
            web::scope("/transactions")
                .route("", web::get().to(handlers::get_transactions))
                .route("/{hash}", web::get().to(handlers::get_transaction_by_hash))
        );
}

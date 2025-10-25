use std::sync::Arc;
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;
use transaction_processor::api::{start_server, AppState};
use transaction_processor::application::Mediator;
use transaction_processor::infrastructure::{
    DieselAccountBalanceRepository, DieselAccountRepository, DieselLedgerEventRepository,
};
use transaction_processor::*;

#[tokio::main]
async fn main() {
    // Initialize structured logging
    FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .init();

    info!("Transaction Processor - Event-Sourced Ledger REST API");
    info!("=====================================================");

    // Establish database connection pool
    let pool = match std::panic::catch_unwind(establish_connection_pool) {
        Ok(pool) => {
            info!("✓ Database connection pool established");
            pool
        }
        Err(e) => {
            error!("Failed to establish database connection pool: {:?}", e);
            std::process::exit(1);
        }
    };

    // Initialize repositories
    let account_repository = Arc::new(DieselAccountRepository::new(pool.clone()));
    let event_repository = Arc::new(DieselLedgerEventRepository::new(pool.clone()));
    let balance_repository = Arc::new(DieselAccountBalanceRepository::new(pool.clone()));

    // Initialize mediator with new event-sourcing repositories
    let mediator = Mediator::new(account_repository, event_repository, balance_repository);

    info!("✓ Application initialized with Event-Sourcing DDD architecture");
    info!("  - Domain layer: Entities (Account, LedgerEvent, AccountBalance)");
    info!("  - Application layer: Commands, Queries, Handlers, Mediator");
    info!("  - Infrastructure layer: Repository implementations");
    info!("  - Event-Sourcing: DEBIT/CREDIT events with balance snapshots");

    // Create app state
    let state = AppState::new(mediator);

    // Start REST API server
    info!("\n--- Starting REST API Server ---");
    let port = 3000;

    if let Err(e) = start_server(state, port).await {
        error!("Server error: {}", e);
        std::process::exit(1);
    }
}

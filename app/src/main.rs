use std::sync::Arc;
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;
use transaction_processor::application::commands::{
    CreateAccountCommand, CreateTransactionCommand,
};
use transaction_processor::application::queries::{ListAccountsQuery, ListTransactionsQuery};
use transaction_processor::application::Mediator;
use transaction_processor::infrastructure::{DieselAccountRepository, DieselTransactionRepository};
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

    info!("Transaction Processor - Ledger Application");
    info!("==========================================");

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
    let transaction_repository = Arc::new(DieselTransactionRepository::new(pool.clone()));

    // Initialize mediator
    let mediator = Mediator::new(account_repository, transaction_repository);

    info!("✓ Application initialized with DDD architecture");
    info!("  - Domain layer: Entities, Value Objects, Repositories (traits)");
    info!("  - Application layer: Commands, Queries, Handlers, Mediator");
    info!("  - Infrastructure layer: Repository implementations");

    // Example: Create a new account using command
    info!("\n--- Creating Account ---");
    let create_account_cmd =
        CreateAccountCommand::new("ACC001".to_string(), "Main Account".to_string(), 10000);

    match mediator.send_create_account(create_account_cmd).await {
        Ok(account) => {
            info!(
                "✓ Created account: {} ({})",
                account.account_name, account.account_number
            );
            info!("  Initial balance: {}", account.balance);
        }
        Err(e) => {
            error!("Error creating account: {}", e);
        }
    }

    // Example: Create a deposit transaction using command
    info!("\n--- Processing Deposit ---");
    let deposit_cmd =
        CreateTransactionCommand::new_deposit(1, 5000, Some("Initial deposit".to_string()));

    match mediator.send_create_transaction(deposit_cmd).await {
        Ok(transaction) => {
            info!("✓ Deposit processed: ID {:?}", transaction.id);
            info!("  Amount: {}", transaction.amount);
            info!("  Type: {}", transaction.transaction_type);
        }
        Err(e) => {
            error!("Error processing deposit: {}", e);
        }
    }

    // Example: Create a withdrawal transaction
    info!("\n--- Processing Withdrawal ---");
    let withdrawal_cmd =
        CreateTransactionCommand::new_withdrawal(1, 2000, Some("ATM withdrawal".to_string()));

    match mediator.send_create_transaction(withdrawal_cmd).await {
        Ok(transaction) => {
            info!("✓ Withdrawal processed: ID {:?}", transaction.id);
            info!("  Amount: {}", transaction.amount);
            info!("  Type: {}", transaction.transaction_type);
        }
        Err(e) => {
            error!("Error processing withdrawal: {}", e);
        }
    }

    // List all accounts using query
    info!("\n--- Listing All Accounts ---");
    let list_accounts_query = ListAccountsQuery::new();

    match mediator.send_list_accounts(list_accounts_query).await {
        Ok(accounts) => {
            info!("Found {} accounts:", accounts.len());
            for account in accounts {
                info!(
                    "  - {} ({}): Balance = {}",
                    account.account_name, account.account_number, account.balance
                );
            }
        }
        Err(e) => {
            error!("Error listing accounts: {}", e);
        }
    }

    // List all transactions using query
    info!("\n--- Listing All Transactions ---");
    let list_transactions_query = ListTransactionsQuery::new();

    match mediator
        .send_list_transactions(list_transactions_query)
        .await
    {
        Ok(transactions) => {
            info!("Found {} transactions:", transactions.len());
            for transaction in transactions {
                info!(
                    "  - ID {:?}: {} {} from {:?} to {:?}",
                    transaction.id,
                    transaction.transaction_type,
                    transaction.amount,
                    transaction.from_account_id,
                    transaction.to_account_id
                );
            }
        }
        Err(e) => {
            error!("Error listing transactions: {}", e);
        }
    }

    info!("\n✓ Transaction processor completed successfully!");
    info!("Architecture implemented:");
    info!("  ✓ DDD layered architecture");
    info!("  ✓ Mediator pattern for command/query handling");
    info!("  ✓ Structured logging with tracing");
    info!("  ✓ Proper error handling with custom domain errors");
}

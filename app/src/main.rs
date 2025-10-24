use transaction_processor::*;

fn main() {
    println!("Transaction Processor - Ledger Application");
    println!("==========================================\n");

    // Establish database connection pool
    let pool = establish_connection_pool();
    println!("✓ Database connection pool established");

    // Example: Create a new account
    match create_account(&pool, "ACC001".to_string(), "Main Account".to_string(), 10000) {
        Ok(account) => {
            println!("✓ Created account: {} ({})", account.account_name, account.account_number);
            println!("  Initial balance: {}", account.balance);
        }
        Err(e) => println!("Error creating account: {}", e),
    }

    // Example: Create a transaction
    match create_transaction(
        &pool,
        None,
        Some(1),
        5000,
        "deposit".to_string(),
        Some("Initial deposit".to_string()),
    ) {
        Ok(transaction) => {
            println!("✓ Created transaction: ID {}", transaction.id);
            println!("  Amount: {}", transaction.amount);
            println!("  Type: {}", transaction.transaction_type);
        }
        Err(e) => println!("Error creating transaction: {}", e),
    }

    // List all accounts
    println!("\nListing all accounts:");
    match list_accounts(&pool) {
        Ok(accounts) => {
            for account in accounts {
                println!("  - {} ({}): Balance = {}", 
                    account.account_name, 
                    account.account_number, 
                    account.balance
                );
            }
        }
        Err(e) => println!("Error listing accounts: {}", e),
    }

    // List all transactions
    println!("\nListing all transactions:");
    match list_transactions(&pool) {
        Ok(transactions) => {
            for transaction in transactions {
                println!("  - ID {}: {} {} from {:?} to {:?}", 
                    transaction.id,
                    transaction.transaction_type,
                    transaction.amount,
                    transaction.from_account_id,
                    transaction.to_account_id
                );
            }
        }
        Err(e) => println!("Error listing transactions: {}", e),
    }

    println!("\n✓ Transaction processor completed successfully!");
}

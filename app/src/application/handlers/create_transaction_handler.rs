use crate::application::commands::CreateTransactionCommand;
use crate::domain::{DomainResult, Money, Transaction, TransactionService, TransactionType};
use std::sync::Arc;
use tracing::{error, info};

pub struct CreateTransactionHandler {
    transaction_service: Arc<TransactionService>,
}

impl CreateTransactionHandler {
    pub fn new(transaction_service: Arc<TransactionService>) -> Self {
        Self {
            transaction_service,
        }
    }

    pub async fn handle(&self, command: CreateTransactionCommand) -> DomainResult<Transaction> {
        info!(
            "Creating transaction: type={}, amount={}",
            command.transaction_type, command.amount
        );

        // Validate and create value objects
        let amount = Money::new(command.amount)?;
        let transaction_type = TransactionType::from_string(&command.transaction_type)?;

        // Process based on transaction type
        let transaction = match transaction_type {
            TransactionType::Deposit => {
                let to_account_id = command.to_account_id.ok_or_else(|| {
                    error!("Deposit requires to_account_id");
                    crate::domain::DomainError::ValidationError(
                        "Deposit requires to_account_id".to_string(),
                    )
                })?;
                self.transaction_service
                    .process_deposit(to_account_id, amount, command.description)
                    .await?
            }
            TransactionType::Withdrawal => {
                let from_account_id = command.from_account_id.ok_or_else(|| {
                    error!("Withdrawal requires from_account_id");
                    crate::domain::DomainError::ValidationError(
                        "Withdrawal requires from_account_id".to_string(),
                    )
                })?;
                self.transaction_service
                    .process_withdrawal(from_account_id, amount, command.description)
                    .await?
            }
            TransactionType::Transfer => {
                let from_account_id = command.from_account_id.ok_or_else(|| {
                    error!("Transfer requires from_account_id");
                    crate::domain::DomainError::ValidationError(
                        "Transfer requires from_account_id".to_string(),
                    )
                })?;
                let to_account_id = command.to_account_id.ok_or_else(|| {
                    error!("Transfer requires to_account_id");
                    crate::domain::DomainError::ValidationError(
                        "Transfer requires to_account_id".to_string(),
                    )
                })?;
                self.transaction_service
                    .process_transfer(from_account_id, to_account_id, amount, command.description)
                    .await?
            }
        };

        info!("Transaction created successfully: id={:?}", transaction.id);
        Ok(transaction)
    }
}

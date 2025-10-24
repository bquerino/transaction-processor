use crate::application::queries::{GetTransactionsByAccountQuery, ListTransactionsQuery};
use crate::domain::{DomainResult, Transaction, TransactionRepository};
use std::sync::Arc;
use tracing::info;

pub struct ListTransactionsHandler {
    transaction_repository: Arc<dyn TransactionRepository>,
}

impl ListTransactionsHandler {
    pub fn new(transaction_repository: Arc<dyn TransactionRepository>) -> Self {
        Self {
            transaction_repository,
        }
    }

    pub async fn handle(&self, _query: ListTransactionsQuery) -> DomainResult<Vec<Transaction>> {
        info!("Listing all transactions");
        self.transaction_repository.find_all().await
    }

    pub async fn handle_by_account(
        &self,
        query: GetTransactionsByAccountQuery,
    ) -> DomainResult<Vec<Transaction>> {
        info!("Listing transactions for account: {}", query.account_id);
        self.transaction_repository
            .find_by_account_id(query.account_id)
            .await
    }
}

use crate::application::commands::{CreateAccountCommand, CreateTransactionCommand};
use crate::application::handlers::{
    CreateAccountHandler, CreateTransactionHandler, GetAccountHandler, ListAccountsHandler,
    ListTransactionsHandler,
};
use crate::application::queries::{
    GetAccountByNumberQuery, GetAccountQuery, GetTransactionsByAccountQuery, ListAccountsQuery,
    ListTransactionsQuery,
};
use crate::domain::{
    Account, AccountRepository, DomainResult, Transaction, TransactionRepository,
    TransactionService,
};
use std::sync::Arc;

/// Mediator pattern implementation for command and query dispatching
pub struct Mediator {
    create_account_handler: CreateAccountHandler,
    create_transaction_handler: CreateTransactionHandler,
    get_account_handler: GetAccountHandler,
    list_accounts_handler: ListAccountsHandler,
    list_transactions_handler: ListTransactionsHandler,
}

impl Mediator {
    pub fn new(
        account_repository: Arc<dyn AccountRepository>,
        transaction_repository: Arc<dyn TransactionRepository>,
    ) -> Self {
        let transaction_service = Arc::new(TransactionService::new(
            account_repository.clone(),
            transaction_repository.clone(),
        ));

        Self {
            create_account_handler: CreateAccountHandler::new(account_repository.clone()),
            create_transaction_handler: CreateTransactionHandler::new(transaction_service),
            get_account_handler: GetAccountHandler::new(account_repository.clone()),
            list_accounts_handler: ListAccountsHandler::new(account_repository.clone()),
            list_transactions_handler: ListTransactionsHandler::new(transaction_repository),
        }
    }

    // Command handlers
    pub async fn send_create_account(
        &self,
        command: CreateAccountCommand,
    ) -> DomainResult<Account> {
        self.create_account_handler.handle(command).await
    }

    pub async fn send_create_transaction(
        &self,
        command: CreateTransactionCommand,
    ) -> DomainResult<Transaction> {
        self.create_transaction_handler.handle(command).await
    }

    // Query handlers
    pub async fn send_get_account(&self, query: GetAccountQuery) -> DomainResult<Account> {
        self.get_account_handler.handle(query).await
    }

    pub async fn send_get_account_by_number(
        &self,
        query: GetAccountByNumberQuery,
    ) -> DomainResult<Account> {
        self.get_account_handler.handle_by_number(query).await
    }

    pub async fn send_list_accounts(&self, query: ListAccountsQuery) -> DomainResult<Vec<Account>> {
        self.list_accounts_handler.handle(query).await
    }

    pub async fn send_list_transactions(
        &self,
        query: ListTransactionsQuery,
    ) -> DomainResult<Vec<Transaction>> {
        self.list_transactions_handler.handle(query).await
    }

    pub async fn send_get_transactions_by_account(
        &self,
        query: GetTransactionsByAccountQuery,
    ) -> DomainResult<Vec<Transaction>> {
        self.list_transactions_handler
            .handle_by_account(query)
            .await
    }
}

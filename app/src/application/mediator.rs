use crate::application::commands::{
    CreateAccountCommand, CreateBalanceSnapshotCommand, CreateLedgerEventCommand,
};
use crate::application::handlers::{
    CreateAccountHandler, CreateBalanceSnapshotHandler, CreateLedgerEventHandler,
    GetAccountBalanceHandler, GetAccountHandler, ListAccountsHandler, ListLedgerEventsHandler,
};
use crate::application::queries::{
    GetAccountBalanceQuery, GetAccountByNumberQuery, GetAccountQuery, ListAccountsQuery,
    ListLedgerEventsQuery,
};
use crate::domain::{
    Account, AccountBalance, AccountBalanceRepository, AccountRepository, DomainResult,
    LedgerEvent, LedgerEventRepository,
};
use std::sync::Arc;

/// Mediator pattern implementation for command and query dispatching
pub struct Mediator {
    create_account_handler: CreateAccountHandler,
    create_ledger_event_handler: CreateLedgerEventHandler,
    create_balance_snapshot_handler: CreateBalanceSnapshotHandler,
    get_account_handler: GetAccountHandler,
    get_account_balance_handler: GetAccountBalanceHandler,
    list_accounts_handler: ListAccountsHandler,
    list_ledger_events_handler: ListLedgerEventsHandler,
}

impl Mediator {
    pub fn new(
        account_repository: Arc<dyn AccountRepository>,
        event_repository: Arc<dyn LedgerEventRepository>,
        balance_repository: Arc<dyn AccountBalanceRepository>,
    ) -> Self {
        Self {
            create_account_handler: CreateAccountHandler::new(account_repository.clone()),
            create_ledger_event_handler: CreateLedgerEventHandler::new(event_repository.clone()),
            create_balance_snapshot_handler: CreateBalanceSnapshotHandler::new(
                event_repository.clone(),
                balance_repository.clone(),
            ),
            get_account_handler: GetAccountHandler::new(account_repository.clone()),
            get_account_balance_handler: GetAccountBalanceHandler::new(
                event_repository.clone(),
                balance_repository,
            ),
            list_accounts_handler: ListAccountsHandler::new(account_repository),
            list_ledger_events_handler: ListLedgerEventsHandler::new(event_repository),
        }
    }

    // Command handlers
    pub async fn send_create_account(
        &self,
        command: CreateAccountCommand,
    ) -> DomainResult<Account> {
        self.create_account_handler.handle(command).await
    }

    pub async fn send_create_ledger_event(
        &self,
        command: CreateLedgerEventCommand,
    ) -> DomainResult<LedgerEvent> {
        self.create_ledger_event_handler.handle(command).await
    }

    pub async fn send_create_balance_snapshot(
        &self,
        command: CreateBalanceSnapshotCommand,
    ) -> DomainResult<AccountBalance> {
        self.create_balance_snapshot_handler.handle(command).await
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

    pub async fn send_get_account_balance(
        &self,
        query: GetAccountBalanceQuery,
    ) -> DomainResult<AccountBalance> {
        self.get_account_balance_handler.handle(query).await
    }

    pub async fn send_list_accounts(&self, query: ListAccountsQuery) -> DomainResult<Vec<Account>> {
        self.list_accounts_handler.handle(query).await
    }

    pub async fn send_list_ledger_events(
        &self,
        query: ListLedgerEventsQuery,
    ) -> DomainResult<Vec<LedgerEvent>> {
        self.list_ledger_events_handler.handle(query).await
    }
}

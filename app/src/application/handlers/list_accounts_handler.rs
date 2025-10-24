use crate::application::queries::ListAccountsQuery;
use crate::domain::{Account, AccountRepository, DomainResult};
use std::sync::Arc;
use tracing::info;

pub struct ListAccountsHandler {
    account_repository: Arc<dyn AccountRepository>,
}

impl ListAccountsHandler {
    pub fn new(account_repository: Arc<dyn AccountRepository>) -> Self {
        Self { account_repository }
    }

    pub async fn handle(&self, _query: ListAccountsQuery) -> DomainResult<Vec<Account>> {
        info!("Listing all accounts");
        self.account_repository.find_all().await
    }
}

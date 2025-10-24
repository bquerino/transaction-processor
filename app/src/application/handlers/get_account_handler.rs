use crate::application::queries::{GetAccountByNumberQuery, GetAccountQuery};
use crate::domain::{Account, AccountNumber, AccountRepository, DomainResult};
use std::sync::Arc;
use tracing::info;

pub struct GetAccountHandler {
    account_repository: Arc<dyn AccountRepository>,
}

impl GetAccountHandler {
    pub fn new(account_repository: Arc<dyn AccountRepository>) -> Self {
        Self { account_repository }
    }

    pub async fn handle(&self, query: GetAccountQuery) -> DomainResult<Account> {
        info!("Getting account by id: {}", query.account_id);
        self.account_repository.find_by_id(query.account_id).await
    }

    pub async fn handle_by_number(&self, query: GetAccountByNumberQuery) -> DomainResult<Account> {
        info!("Getting account by number: {}", query.account_number);
        let account_number = AccountNumber::new(query.account_number)?;
        self.account_repository
            .find_by_account_number(&account_number)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Money;
    use mockall::mock;
    use mockall::predicate::*;

    mock! {
        pub AccountRepo {}

        #[async_trait::async_trait]
        impl AccountRepository for AccountRepo {
            async fn save(&self, account: &Account) -> DomainResult<Account>;
            async fn find_by_id(&self, id: i32) -> DomainResult<Account>;
            async fn find_by_account_number(&self, account_number: &AccountNumber) -> DomainResult<Account>;
            async fn find_all(&self) -> DomainResult<Vec<Account>>;
            async fn update(&self, account: &Account) -> DomainResult<Account>;
            async fn exists_by_account_number(&self, account_number: &AccountNumber) -> DomainResult<bool>;
        }
    }

    #[tokio::test]
    async fn test_get_account_by_id() {
        let mut mock_repo = MockAccountRepo::new();

        let account_number = AccountNumber::new("ACC001".to_string()).unwrap();
        let mut account = Account::new(account_number, "Test Account".to_string());
        account.id = Some(1);

        let account_clone = account.clone();
        mock_repo
            .expect_find_by_id()
            .with(eq(1))
            .times(1)
            .returning(move |_| Ok(account_clone.clone()));

        let handler = GetAccountHandler::new(Arc::new(mock_repo));
        let query = GetAccountQuery::new(1);

        let result = handler.handle(query).await;

        assert!(result.is_ok());
        let found_account = result.unwrap();
        assert_eq!(found_account.id, Some(1));
    }
}

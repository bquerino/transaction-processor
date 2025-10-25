use crate::application::commands::CreateAccountCommand;
use crate::domain::{Account, AccountNumber, AccountRepository, DomainResult};
use std::sync::Arc;
use tracing::{error, info};

pub struct CreateAccountHandler {
    account_repository: Arc<dyn AccountRepository>,
}

impl CreateAccountHandler {
    pub fn new(account_repository: Arc<dyn AccountRepository>) -> Self {
        Self { account_repository }
    }

    pub async fn handle(&self, command: CreateAccountCommand) -> DomainResult<Account> {
        info!(
            "Creating account: number={}, name={}",
            command.account_number, command.account_name
        );

        // Validate and create value objects
        let account_number = AccountNumber::new(command.account_number)?;

        // Check if account number already exists
        if self
            .account_repository
            .exists_by_account_number(&account_number)
            .await?
        {
            error!("Account number already exists: {}", account_number);
            return Err(crate::domain::DomainError::DuplicateAccountNumber(
                account_number.value().to_string(),
            ));
        }

        // Create account entity (no balance - will be calculated from events)
        let account = Account::new(account_number, command.account_name);

        // Validate account
        account.validate()?;

        // Save account
        let saved_account = self.account_repository.save(&account).await?;

        info!("Account created successfully: id={:?}", saved_account.id);
        Ok(saved_account)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
    async fn test_create_account_success() {
        let mut mock_repo = MockAccountRepo::new();

        mock_repo
            .expect_exists_by_account_number()
            .times(1)
            .returning(|_| Ok(false));

        let account_number = AccountNumber::new("ACC001".to_string()).unwrap();
        let mut expected_account = Account::new(account_number, "Test Account".to_string());
        expected_account.id = Some(1);

        let expected_account_clone = expected_account.clone();
        mock_repo
            .expect_save()
            .times(1)
            .returning(move |_| Ok(expected_account_clone.clone()));

        let handler = CreateAccountHandler::new(Arc::new(mock_repo));
        let command = CreateAccountCommand::new("ACC001".to_string(), "Test Account".to_string());

        let result = handler.handle(command).await;

        assert!(result.is_ok());
        let account = result.unwrap();
        assert_eq!(account.id, Some(1));
    }

    #[tokio::test]
    async fn test_create_account_duplicate() {
        let mut mock_repo = MockAccountRepo::new();

        mock_repo
            .expect_exists_by_account_number()
            .times(1)
            .returning(|_| Ok(true));

        let handler = CreateAccountHandler::new(Arc::new(mock_repo));
        let command = CreateAccountCommand::new("ACC001".to_string(), "Test Account".to_string());

        let result = handler.handle(command).await;

        assert!(result.is_err());
    }
}

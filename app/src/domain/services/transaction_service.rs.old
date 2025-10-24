use crate::domain::entities::Transaction;
use crate::domain::errors::{DomainError, DomainResult};
use crate::domain::repositories::{AccountRepository, TransactionRepository};
use crate::domain::value_objects::Money;
use std::sync::Arc;
use tracing::{error, info};

pub struct TransactionService {
    account_repository: Arc<dyn AccountRepository>,
    transaction_repository: Arc<dyn TransactionRepository>,
}

impl TransactionService {
    pub fn new(
        account_repository: Arc<dyn AccountRepository>,
        transaction_repository: Arc<dyn TransactionRepository>,
    ) -> Self {
        Self {
            account_repository,
            transaction_repository,
        }
    }

    pub async fn process_deposit(
        &self,
        to_account_id: i32,
        amount: Money,
        description: Option<String>,
    ) -> DomainResult<Transaction> {
        info!(
            "Processing deposit: account_id={}, amount={}",
            to_account_id,
            amount.value()
        );

        // Get account
        let mut account = self.account_repository.find_by_id(to_account_id).await?;

        // Deposit to account
        account.deposit(amount)?;

        // Update account
        self.account_repository.update(&account).await?;

        // Create transaction record
        let transaction = Transaction::new_deposit(to_account_id, amount, description);
        let saved_transaction = self.transaction_repository.save(&transaction).await?;

        info!(
            "Deposit processed successfully: transaction_id={:?}",
            saved_transaction.id
        );
        Ok(saved_transaction)
    }

    pub async fn process_withdrawal(
        &self,
        from_account_id: i32,
        amount: Money,
        description: Option<String>,
    ) -> DomainResult<Transaction> {
        info!(
            "Processing withdrawal: account_id={}, amount={}",
            from_account_id,
            amount.value()
        );

        // Get account
        let mut account = self.account_repository.find_by_id(from_account_id).await?;

        // Check if sufficient balance
        if !account.can_withdraw(&amount) {
            error!(
                "Insufficient balance for withdrawal: account_id={}, required={}, available={}",
                from_account_id,
                amount.value(),
                account.balance.value()
            );
            return Err(DomainError::InsufficientBalance {
                required: amount.value(),
                available: account.balance.value(),
            });
        }

        // Withdraw from account
        account.withdraw(amount)?;

        // Update account
        self.account_repository.update(&account).await?;

        // Create transaction record
        let transaction = Transaction::new_withdrawal(from_account_id, amount, description);
        let saved_transaction = self.transaction_repository.save(&transaction).await?;

        info!(
            "Withdrawal processed successfully: transaction_id={:?}",
            saved_transaction.id
        );
        Ok(saved_transaction)
    }

    pub async fn process_transfer(
        &self,
        from_account_id: i32,
        to_account_id: i32,
        amount: Money,
        description: Option<String>,
    ) -> DomainResult<Transaction> {
        info!(
            "Processing transfer: from_account_id={}, to_account_id={}, amount={}",
            from_account_id,
            to_account_id,
            amount.value()
        );

        // Get both accounts
        let mut from_account = self.account_repository.find_by_id(from_account_id).await?;
        let mut to_account = self.account_repository.find_by_id(to_account_id).await?;

        // Check if sufficient balance
        if !from_account.can_withdraw(&amount) {
            error!(
                "Insufficient balance for transfer: from_account_id={}, required={}, available={}",
                from_account_id,
                amount.value(),
                from_account.balance.value()
            );
            return Err(DomainError::InsufficientBalance {
                required: amount.value(),
                available: from_account.balance.value(),
            });
        }

        // Perform transfer
        from_account.withdraw(amount)?;
        to_account.deposit(amount)?;

        // Update both accounts
        self.account_repository.update(&from_account).await?;
        self.account_repository.update(&to_account).await?;

        // Create transaction record
        let transaction =
            Transaction::new_transfer(from_account_id, to_account_id, amount, description);
        let saved_transaction = self.transaction_repository.save(&transaction).await?;

        info!(
            "Transfer processed successfully: transaction_id={:?}",
            saved_transaction.id
        );
        Ok(saved_transaction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::Account;
    use crate::domain::value_objects::AccountNumber;
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

    mock! {
        pub TransactionRepo {}

        #[async_trait::async_trait]
        impl TransactionRepository for TransactionRepo {
            async fn save(&self, transaction: &Transaction) -> DomainResult<Transaction>;
            async fn find_by_id(&self, id: i32) -> DomainResult<Transaction>;
            async fn find_all(&self) -> DomainResult<Vec<Transaction>>;
            async fn find_by_account_id(&self, account_id: i32) -> DomainResult<Vec<Transaction>>;
        }
    }

    #[tokio::test]
    async fn test_process_deposit() {
        let mut mock_account_repo = MockAccountRepo::new();
        let mut mock_transaction_repo = MockTransactionRepo::new();

        let account_number = AccountNumber::new("ACC001".to_string()).unwrap();
        let balance = Money::new(1000).unwrap();
        let mut account = Account::new(account_number, "Test Account".to_string(), balance);
        account.id = Some(1);

        let account_clone = account.clone();
        mock_account_repo
            .expect_find_by_id()
            .with(eq(1))
            .times(1)
            .returning(move |_| Ok(account_clone.clone()));

        let mut updated_account = account.clone();
        updated_account.balance = Money::new(1500).unwrap();
        let updated_account_clone = updated_account.clone();
        mock_account_repo
            .expect_update()
            .times(1)
            .returning(move |_| Ok(updated_account_clone.clone()));

        let amount = Money::new(500).unwrap();
        let transaction = Transaction::new_deposit(1, amount, Some("Test deposit".to_string()));
        let transaction_clone = transaction.clone();
        mock_transaction_repo
            .expect_save()
            .times(1)
            .returning(move |_| Ok(transaction_clone.clone()));

        let service =
            TransactionService::new(Arc::new(mock_account_repo), Arc::new(mock_transaction_repo));

        let result = service
            .process_deposit(1, amount, Some("Test deposit".to_string()))
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_process_withdrawal_insufficient_balance() {
        let mut mock_account_repo = MockAccountRepo::new();
        let mock_transaction_repo = MockTransactionRepo::new();

        let account_number = AccountNumber::new("ACC001".to_string()).unwrap();
        let balance = Money::new(100).unwrap();
        let mut account = Account::new(account_number, "Test Account".to_string(), balance);
        account.id = Some(1);

        let account_clone = account.clone();
        mock_account_repo
            .expect_find_by_id()
            .with(eq(1))
            .times(1)
            .returning(move |_| Ok(account_clone.clone()));

        let service =
            TransactionService::new(Arc::new(mock_account_repo), Arc::new(mock_transaction_repo));

        let amount = Money::new(500).unwrap();
        let result = service
            .process_withdrawal(1, amount, Some("Test withdrawal".to_string()))
            .await;

        assert!(result.is_err());
    }
}

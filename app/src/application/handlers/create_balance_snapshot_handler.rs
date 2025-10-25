use crate::application::commands::CreateBalanceSnapshotCommand;
use crate::domain::entities::AccountBalance;
use crate::domain::errors::DomainResult;
use crate::domain::repositories::{AccountBalanceRepository, LedgerEventRepository};
use crate::domain::value_objects::Money;
use std::sync::Arc;
use tracing::info;

pub struct CreateBalanceSnapshotHandler {
    event_repository: Arc<dyn LedgerEventRepository>,
    balance_repository: Arc<dyn AccountBalanceRepository>,
}

impl CreateBalanceSnapshotHandler {
    pub fn new(
        event_repository: Arc<dyn LedgerEventRepository>,
        balance_repository: Arc<dyn AccountBalanceRepository>,
    ) -> Self {
        Self {
            event_repository,
            balance_repository,
        }
    }

    pub async fn handle(
        &self,
        command: CreateBalanceSnapshotCommand,
    ) -> DomainResult<AccountBalance> {
        info!(
            "Creating balance snapshot for account_id={}",
            command.account_id
        );

        // Calculate current balance from events
        let balance_value = self
            .event_repository
            .calculate_balance(command.account_id)
            .await?;

        let balance = Money::new(balance_value)?;

        let snapshot = AccountBalance::new(command.account_id, balance);

        let saved_snapshot = self.balance_repository.save(&snapshot).await?;

        info!(
            "Balance snapshot created successfully: id={:?}, balance={}",
            saved_snapshot.id,
            saved_snapshot.balance.value()
        );

        Ok(saved_snapshot)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::repositories::account_balance_repository::MockAccountBalanceRepository;
    use crate::domain::repositories::ledger_event_repository::MockLedgerEventRepository;
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_create_balance_snapshot() {
        let mut mock_event_repo = MockLedgerEventRepository::new();
        let mut mock_balance_repo = MockAccountBalanceRepository::new();

        mock_event_repo
            .expect_calculate_balance()
            .with(eq(1))
            .once()
            .returning(|_| Ok(5000));

        mock_balance_repo
            .expect_save()
            .once()
            .returning(|snapshot| {
                let mut saved = snapshot.clone();
                saved.id = Some(1);
                Ok(saved)
            });

        let handler = CreateBalanceSnapshotHandler::new(
            Arc::new(mock_event_repo),
            Arc::new(mock_balance_repo),
        );
        let command = CreateBalanceSnapshotCommand::new(1);

        let result = handler.handle(command).await;
        assert!(result.is_ok());

        let snapshot = result.unwrap();
        assert_eq!(snapshot.id, Some(1));
        assert_eq!(snapshot.balance.value(), 5000);
    }
}

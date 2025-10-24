use crate::application::queries::GetAccountBalanceQuery;
use crate::domain::entities::AccountBalance;
use crate::domain::errors::{DomainError, DomainResult};
use crate::domain::repositories::{AccountBalanceRepository, LedgerEventRepository};
use crate::domain::value_objects::Money;
use std::sync::Arc;
use tracing::info;

pub struct GetAccountBalanceHandler {
    event_repository: Arc<dyn LedgerEventRepository>,
    balance_repository: Arc<dyn AccountBalanceRepository>,
}

impl GetAccountBalanceHandler {
    pub fn new(
        event_repository: Arc<dyn LedgerEventRepository>,
        balance_repository: Arc<dyn AccountBalanceRepository>,
    ) -> Self {
        Self {
            event_repository,
            balance_repository,
        }
    }

    pub async fn handle(&self, query: GetAccountBalanceQuery) -> DomainResult<AccountBalance> {
        info!(
            "Getting account balance: account_id={}, use_snapshot={}",
            query.account_id, query.use_snapshot
        );

        if query.use_snapshot {
            // Try to get the latest snapshot
            if let Some(snapshot) = self
                .balance_repository
                .find_latest_by_account_id(query.account_id)
                .await?
            {
                info!("Using balance snapshot: balance={}", snapshot.balance.value());
                return Ok(snapshot);
            }
        }

        // Calculate from events
        let balance_value = self
            .event_repository
            .calculate_balance(query.account_id)
            .await?;

        info!("Calculated balance from events: balance={}", balance_value);

        // Return a calculated balance (not persisted)
        Ok(AccountBalance::new(
            query.account_id,
            Money::new(balance_value)
                .map_err(|e| DomainError::ValidationError(format!("Invalid balance: {}", e)))?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::repositories::account_balance_repository::MockAccountBalanceRepository;
    use crate::domain::repositories::ledger_event_repository::MockLedgerEventRepository;
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_get_balance_from_events() {
        let mut mock_event_repo = MockLedgerEventRepository::new();
        let mock_balance_repo = MockAccountBalanceRepository::new();

        // When use_snapshot is false, balance repo should not be called
        mock_event_repo
            .expect_calculate_balance()
            .with(eq(1))
            .once()
            .returning(|_| Ok(3000));

        let handler = GetAccountBalanceHandler::new(
            Arc::new(mock_event_repo),
            Arc::new(mock_balance_repo),
        );
        let query = GetAccountBalanceQuery::new(1); // use_snapshot defaults to false

        let result = handler.handle(query).await;
        assert!(result.is_ok());

        let balance = result.unwrap();
        assert_eq!(balance.balance.value(), 3000);
    }
}

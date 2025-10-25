use crate::domain::entities::AccountBalance;
use crate::domain::errors::DomainResult;
use async_trait::async_trait;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait AccountBalanceRepository: Send + Sync {
    async fn save(&self, balance: &AccountBalance) -> DomainResult<AccountBalance>;
    async fn find_latest_by_account_id(&self, account_id: i32)
        -> DomainResult<Option<AccountBalance>>;
    async fn find_all_by_account_id(&self, account_id: i32) -> DomainResult<Vec<AccountBalance>>;
}

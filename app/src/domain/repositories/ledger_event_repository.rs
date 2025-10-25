use crate::domain::entities::LedgerEvent;
use crate::domain::errors::DomainResult;
use async_trait::async_trait;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait LedgerEventRepository: Send + Sync {
    async fn save(&self, event: &LedgerEvent) -> DomainResult<LedgerEvent>;
    async fn find_by_account_id(&self, account_id: i32) -> DomainResult<Vec<LedgerEvent>>;
    async fn find_all(&self) -> DomainResult<Vec<LedgerEvent>>;
    async fn calculate_balance(&self, account_id: i32) -> DomainResult<i64>;
}

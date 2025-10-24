use crate::domain::entities::Transaction;
use crate::domain::errors::DomainResult;
use async_trait::async_trait;

#[async_trait]
pub trait TransactionRepository: Send + Sync {
    async fn save(&self, transaction: &Transaction) -> DomainResult<Transaction>;
    async fn find_by_id(&self, id: i32) -> DomainResult<Transaction>;
    async fn find_all(&self) -> DomainResult<Vec<Transaction>>;
    async fn find_by_account_id(&self, account_id: i32) -> DomainResult<Vec<Transaction>>;
}

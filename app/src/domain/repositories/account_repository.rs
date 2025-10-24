use crate::domain::entities::Account;
use crate::domain::errors::DomainResult;
use crate::domain::value_objects::AccountNumber;
use async_trait::async_trait;

#[async_trait]
pub trait AccountRepository: Send + Sync {
    async fn save(&self, account: &Account) -> DomainResult<Account>;
    async fn find_by_id(&self, id: i32) -> DomainResult<Account>;
    async fn find_by_account_number(&self, account_number: &AccountNumber)
        -> DomainResult<Account>;
    async fn find_all(&self) -> DomainResult<Vec<Account>>;
    async fn update(&self, account: &Account) -> DomainResult<Account>;
    async fn exists_by_account_number(&self, account_number: &AccountNumber) -> DomainResult<bool>;
}

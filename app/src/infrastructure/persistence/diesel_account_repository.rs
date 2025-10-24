use crate::domain::{Account, AccountNumber, AccountRepository, DomainError, DomainResult, Money};
use crate::models;
use crate::schema;
use async_trait::async_trait;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use tracing::{error, info};

pub struct DieselAccountRepository {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl DieselAccountRepository {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AccountRepository for DieselAccountRepository {
    async fn save(&self, account: &Account) -> DomainResult<Account> {
        use schema::accounts;

        let new_account = models::NewAccount {
            account_number: account.account_number.value().to_string(),
            account_name: account.account_name.clone(),
            balance: account.balance.value(),
        };

        let mut conn = self.pool.get().map_err(|e| {
            error!("Failed to get DB connection: {}", e);
            DomainError::RepositoryError(format!("Failed to get DB connection: {}", e))
        })?;

        let saved = diesel::insert_into(accounts::table)
            .values(&new_account)
            .returning(models::Account::as_returning())
            .get_result(&mut conn)
            .map_err(|e| {
                error!("Failed to save account: {}", e);
                DomainError::RepositoryError(format!("Failed to save account: {}", e))
            })?;

        info!("Account saved to database: id={}", saved.id);

        Ok(Account {
            id: Some(saved.id),
            account_number: AccountNumber::new(saved.account_number)?,
            account_name: saved.account_name,
            balance: Money::new(saved.balance)?,
            created_at: Some(saved.created_at),
            updated_at: Some(saved.updated_at),
        })
    }

    async fn find_by_id(&self, id: i32) -> DomainResult<Account> {
        use schema::accounts::dsl;

        let mut conn = self.pool.get().map_err(|e| {
            error!("Failed to get DB connection: {}", e);
            DomainError::RepositoryError(format!("Failed to get DB connection: {}", e))
        })?;

        let account = dsl::accounts
            .find(id)
            .select(models::Account::as_select())
            .first(&mut conn)
            .map_err(|e| {
                error!("Failed to find account by id {}: {}", id, e);
                match e {
                    diesel::result::Error::NotFound => {
                        DomainError::AccountNotFound(format!("Account with id {} not found", id))
                    }
                    _ => DomainError::RepositoryError(format!("Failed to find account: {}", e)),
                }
            })?;

        Ok(Account {
            id: Some(account.id),
            account_number: AccountNumber::new(account.account_number)?,
            account_name: account.account_name,
            balance: Money::new(account.balance)?,
            created_at: Some(account.created_at),
            updated_at: Some(account.updated_at),
        })
    }

    async fn find_by_account_number(
        &self,
        account_number: &AccountNumber,
    ) -> DomainResult<Account> {
        use schema::accounts::dsl;

        let mut conn = self.pool.get().map_err(|e| {
            error!("Failed to get DB connection: {}", e);
            DomainError::RepositoryError(format!("Failed to get DB connection: {}", e))
        })?;

        let account = dsl::accounts
            .filter(dsl::account_number.eq(account_number.value()))
            .select(models::Account::as_select())
            .first(&mut conn)
            .map_err(|e| {
                error!("Failed to find account by number {}: {}", account_number, e);
                match e {
                    diesel::result::Error::NotFound => DomainError::AccountNotFound(format!(
                        "Account with number {} not found",
                        account_number
                    )),
                    _ => DomainError::RepositoryError(format!("Failed to find account: {}", e)),
                }
            })?;

        Ok(Account {
            id: Some(account.id),
            account_number: AccountNumber::new(account.account_number)?,
            account_name: account.account_name,
            balance: Money::new(account.balance)?,
            created_at: Some(account.created_at),
            updated_at: Some(account.updated_at),
        })
    }

    async fn find_all(&self) -> DomainResult<Vec<Account>> {
        use schema::accounts::dsl;

        let mut conn = self.pool.get().map_err(|e| {
            error!("Failed to get DB connection: {}", e);
            DomainError::RepositoryError(format!("Failed to get DB connection: {}", e))
        })?;

        let accounts = dsl::accounts
            .select(models::Account::as_select())
            .load(&mut conn)
            .map_err(|e| {
                error!("Failed to find all accounts: {}", e);
                DomainError::RepositoryError(format!("Failed to find all accounts: {}", e))
            })?;

        accounts
            .into_iter()
            .map(|account| {
                Ok(Account {
                    id: Some(account.id),
                    account_number: AccountNumber::new(account.account_number)?,
                    account_name: account.account_name,
                    balance: Money::new(account.balance)?,
                    created_at: Some(account.created_at),
                    updated_at: Some(account.updated_at),
                })
            })
            .collect()
    }

    async fn update(&self, account: &Account) -> DomainResult<Account> {
        use schema::accounts::dsl;

        let account_id = account.id.ok_or_else(|| {
            error!("Cannot update account without id");
            DomainError::ValidationError("Cannot update account without id".to_string())
        })?;

        let mut conn = self.pool.get().map_err(|e| {
            error!("Failed to get DB connection: {}", e);
            DomainError::RepositoryError(format!("Failed to get DB connection: {}", e))
        })?;

        let updated = diesel::update(dsl::accounts.find(account_id))
            .set((
                dsl::account_name.eq(&account.account_name),
                dsl::balance.eq(account.balance.value()),
            ))
            .returning(models::Account::as_returning())
            .get_result(&mut conn)
            .map_err(|e| {
                error!("Failed to update account: {}", e);
                DomainError::RepositoryError(format!("Failed to update account: {}", e))
            })?;

        info!("Account updated in database: id={}", updated.id);

        Ok(Account {
            id: Some(updated.id),
            account_number: AccountNumber::new(updated.account_number)?,
            account_name: updated.account_name,
            balance: Money::new(updated.balance)?,
            created_at: Some(updated.created_at),
            updated_at: Some(updated.updated_at),
        })
    }

    async fn exists_by_account_number(&self, account_number: &AccountNumber) -> DomainResult<bool> {
        use schema::accounts::dsl;

        let mut conn = self.pool.get().map_err(|e| {
            error!("Failed to get DB connection: {}", e);
            DomainError::RepositoryError(format!("Failed to get DB connection: {}", e))
        })?;

        let count: i64 = dsl::accounts
            .filter(dsl::account_number.eq(account_number.value()))
            .count()
            .get_result(&mut conn)
            .map_err(|e| {
                error!("Failed to check account existence: {}", e);
                DomainError::RepositoryError(format!("Failed to check account existence: {}", e))
            })?;

        Ok(count > 0)
    }
}

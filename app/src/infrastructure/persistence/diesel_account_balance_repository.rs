use crate::domain::entities::AccountBalance;
use crate::domain::errors::{DomainError, DomainResult};
use crate::domain::repositories::AccountBalanceRepository;
use crate::domain::value_objects::Money;
use crate::models::{AccountBalance as DbAccountBalance, NewAccountBalance};
use crate::schema::account_balances;
use async_trait::async_trait;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

pub struct DieselAccountBalanceRepository {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl DieselAccountBalanceRepository {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }

    fn to_domain(db_balance: DbAccountBalance) -> DomainResult<AccountBalance> {
        Ok(AccountBalance {
            id: Some(db_balance.id),
            account_id: db_balance.account_id,
            balance: Money::new(db_balance.balance)?,
            snapshot_at: Some(db_balance.snapshot_at),
        })
    }

    fn to_db(balance: &AccountBalance) -> NewAccountBalance {
        NewAccountBalance {
            account_id: balance.account_id,
            balance: balance.balance.value(),
        }
    }
}

#[async_trait]
impl AccountBalanceRepository for DieselAccountBalanceRepository {
    async fn save(&self, balance: &AccountBalance) -> DomainResult<AccountBalance> {
        let mut conn = self
            .pool
            .get()
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        let new_balance = Self::to_db(balance);

        let db_balance: DbAccountBalance = diesel::insert_into(account_balances::table)
            .values(&new_balance)
            .get_result(&mut conn)
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        Self::to_domain(db_balance)
    }

    async fn find_latest_by_account_id(
        &self,
        account_id: i32,
    ) -> DomainResult<Option<AccountBalance>> {
        let mut conn = self
            .pool
            .get()
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        let result: Option<DbAccountBalance> = account_balances::table
            .filter(account_balances::account_id.eq(account_id))
            .order(account_balances::snapshot_at.desc())
            .first(&mut conn)
            .optional()
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        match result {
            Some(db_balance) => Ok(Some(Self::to_domain(db_balance)?)),
            None => Ok(None),
        }
    }

    async fn find_all_by_account_id(&self, account_id: i32) -> DomainResult<Vec<AccountBalance>> {
        let mut conn = self
            .pool
            .get()
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        let db_balances: Vec<DbAccountBalance> = account_balances::table
            .filter(account_balances::account_id.eq(account_id))
            .order(account_balances::snapshot_at.desc())
            .load(&mut conn)
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        db_balances.into_iter().map(Self::to_domain).collect()
    }
}

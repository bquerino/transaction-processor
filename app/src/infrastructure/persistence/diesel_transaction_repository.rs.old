use crate::domain::{
    DomainError, DomainResult, Money, Transaction, TransactionRepository, TransactionType,
};
use crate::models;
use crate::schema;
use async_trait::async_trait;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use tracing::{error, info};

pub struct DieselTransactionRepository {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl DieselTransactionRepository {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TransactionRepository for DieselTransactionRepository {
    async fn save(&self, transaction: &Transaction) -> DomainResult<Transaction> {
        use schema::transactions;

        let new_transaction = models::NewTransaction {
            from_account_id: transaction.from_account_id,
            to_account_id: transaction.to_account_id,
            amount: transaction.amount.value(),
            transaction_type: transaction.transaction_type.as_str().to_string(),
            description: transaction.description.clone(),
        };

        let mut conn = self.pool.get().map_err(|e| {
            error!("Failed to get DB connection: {}", e);
            DomainError::RepositoryError(format!("Failed to get DB connection: {}", e))
        })?;

        let saved = diesel::insert_into(transactions::table)
            .values(&new_transaction)
            .returning(models::Transaction::as_returning())
            .get_result(&mut conn)
            .map_err(|e| {
                error!("Failed to save transaction: {}", e);
                DomainError::RepositoryError(format!("Failed to save transaction: {}", e))
            })?;

        info!("Transaction saved to database: id={}", saved.id);

        Ok(Transaction {
            id: Some(saved.id),
            from_account_id: saved.from_account_id,
            to_account_id: saved.to_account_id,
            amount: Money::new(saved.amount)?,
            transaction_type: TransactionType::from_string(&saved.transaction_type)?,
            description: saved.description,
            created_at: Some(saved.created_at),
        })
    }

    async fn find_by_id(&self, id: i32) -> DomainResult<Transaction> {
        use schema::transactions::dsl;

        let mut conn = self.pool.get().map_err(|e| {
            error!("Failed to get DB connection: {}", e);
            DomainError::RepositoryError(format!("Failed to get DB connection: {}", e))
        })?;

        let transaction = dsl::transactions
            .find(id)
            .select(models::Transaction::as_select())
            .first(&mut conn)
            .map_err(|e| {
                error!("Failed to find transaction by id {}: {}", id, e);
                match e {
                    diesel::result::Error::NotFound => DomainError::TransactionNotFound(id),
                    _ => DomainError::RepositoryError(format!("Failed to find transaction: {}", e)),
                }
            })?;

        Ok(Transaction {
            id: Some(transaction.id),
            from_account_id: transaction.from_account_id,
            to_account_id: transaction.to_account_id,
            amount: Money::new(transaction.amount)?,
            transaction_type: TransactionType::from_string(&transaction.transaction_type)?,
            description: transaction.description,
            created_at: Some(transaction.created_at),
        })
    }

    async fn find_all(&self) -> DomainResult<Vec<Transaction>> {
        use schema::transactions::dsl;

        let mut conn = self.pool.get().map_err(|e| {
            error!("Failed to get DB connection: {}", e);
            DomainError::RepositoryError(format!("Failed to get DB connection: {}", e))
        })?;

        let transactions = dsl::transactions
            .select(models::Transaction::as_select())
            .load(&mut conn)
            .map_err(|e| {
                error!("Failed to find all transactions: {}", e);
                DomainError::RepositoryError(format!("Failed to find all transactions: {}", e))
            })?;

        transactions
            .into_iter()
            .map(|transaction| {
                Ok(Transaction {
                    id: Some(transaction.id),
                    from_account_id: transaction.from_account_id,
                    to_account_id: transaction.to_account_id,
                    amount: Money::new(transaction.amount)?,
                    transaction_type: TransactionType::from_string(&transaction.transaction_type)?,
                    description: transaction.description,
                    created_at: Some(transaction.created_at),
                })
            })
            .collect()
    }

    async fn find_by_account_id(&self, account_id: i32) -> DomainResult<Vec<Transaction>> {
        use schema::transactions::dsl;

        let mut conn = self.pool.get().map_err(|e| {
            error!("Failed to get DB connection: {}", e);
            DomainError::RepositoryError(format!("Failed to get DB connection: {}", e))
        })?;

        let transactions = dsl::transactions
            .filter(
                dsl::from_account_id
                    .eq(Some(account_id))
                    .or(dsl::to_account_id.eq(Some(account_id))),
            )
            .select(models::Transaction::as_select())
            .load(&mut conn)
            .map_err(|e| {
                error!("Failed to find transactions by account id: {}", e);
                DomainError::RepositoryError(format!(
                    "Failed to find transactions by account id: {}",
                    e
                ))
            })?;

        transactions
            .into_iter()
            .map(|transaction| {
                Ok(Transaction {
                    id: Some(transaction.id),
                    from_account_id: transaction.from_account_id,
                    to_account_id: transaction.to_account_id,
                    amount: Money::new(transaction.amount)?,
                    transaction_type: TransactionType::from_string(&transaction.transaction_type)?,
                    description: transaction.description,
                    created_at: Some(transaction.created_at),
                })
            })
            .collect()
    }
}

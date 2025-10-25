use crate::domain::entities::{EventType, LedgerEvent};
use crate::domain::errors::{DomainError, DomainResult};
use crate::domain::repositories::LedgerEventRepository;
use crate::domain::value_objects::Money;
use crate::models::{LedgerEvent as DbLedgerEvent, NewLedgerEvent};
use crate::schema::ledger_events;
use async_trait::async_trait;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

pub struct DieselLedgerEventRepository {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl DieselLedgerEventRepository {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }

    fn to_domain(db_event: DbLedgerEvent) -> DomainResult<LedgerEvent> {
        Ok(LedgerEvent {
            id: Some(db_event.id),
            account_id: db_event.account_id,
            event_type: EventType::from_string(&db_event.event_type)?,
            amount: Money::new(db_event.amount)?,
            description: db_event.description,
            created_at: Some(db_event.created_at),
        })
    }

    fn to_db(event: &LedgerEvent) -> NewLedgerEvent {
        NewLedgerEvent {
            account_id: event.account_id,
            event_type: event.event_type.to_string(),
            amount: event.amount.value(),
            description: event.description.clone(),
        }
    }
}

#[async_trait]
impl LedgerEventRepository for DieselLedgerEventRepository {
    async fn save(&self, event: &LedgerEvent) -> DomainResult<LedgerEvent> {
        let mut conn = self
            .pool
            .get()
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        let new_event = Self::to_db(event);

        let db_event: DbLedgerEvent = diesel::insert_into(ledger_events::table)
            .values(&new_event)
            .get_result(&mut conn)
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        Self::to_domain(db_event)
    }

    async fn find_by_account_id(&self, account_id: i32) -> DomainResult<Vec<LedgerEvent>> {
        let mut conn = self
            .pool
            .get()
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        let db_events: Vec<DbLedgerEvent> = ledger_events::table
            .filter(ledger_events::account_id.eq(account_id))
            .order(ledger_events::created_at.asc())
            .load(&mut conn)
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        db_events.into_iter().map(Self::to_domain).collect()
    }

    async fn find_all(&self) -> DomainResult<Vec<LedgerEvent>> {
        let mut conn = self
            .pool
            .get()
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        let db_events: Vec<DbLedgerEvent> = ledger_events::table
            .order(ledger_events::created_at.desc())
            .load(&mut conn)
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        db_events.into_iter().map(Self::to_domain).collect()
    }

    async fn calculate_balance(&self, account_id: i32) -> DomainResult<i64> {
        let events = self.find_by_account_id(account_id).await?;

        let balance = events.iter().fold(0i64, |acc, event| match event.event_type {
            EventType::Credit => acc + event.amount.value(),
            EventType::Debit => acc - event.amount.value(),
        });

        Ok(balance)
    }
}

use crate::application::queries::ListLedgerEventsQuery;
use crate::domain::entities::LedgerEvent;
use crate::domain::errors::DomainResult;
use crate::domain::repositories::LedgerEventRepository;
use std::sync::Arc;
use tracing::info;

pub struct ListLedgerEventsHandler {
    event_repository: Arc<dyn LedgerEventRepository>,
}

impl ListLedgerEventsHandler {
    pub fn new(event_repository: Arc<dyn LedgerEventRepository>) -> Self {
        Self { event_repository }
    }

    pub async fn handle(&self, query: ListLedgerEventsQuery) -> DomainResult<Vec<LedgerEvent>> {
        info!("Listing ledger events: account_id={:?}", query.account_id);

        let events = if let Some(account_id) = query.account_id {
            self.event_repository.find_by_account_id(account_id).await?
        } else {
            self.event_repository.find_all().await?
        };

        info!("Found {} ledger events", events.len());

        Ok(events)
    }
}

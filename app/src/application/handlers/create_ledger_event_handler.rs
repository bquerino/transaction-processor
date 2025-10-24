use crate::application::commands::CreateLedgerEventCommand;
use crate::domain::entities::{EventType, LedgerEvent};
use crate::domain::errors::DomainResult;
use crate::domain::repositories::LedgerEventRepository;
use crate::domain::value_objects::Money;
use std::sync::Arc;
use tracing::info;

pub struct CreateLedgerEventHandler {
    event_repository: Arc<dyn LedgerEventRepository>,
}

impl CreateLedgerEventHandler {
    pub fn new(event_repository: Arc<dyn LedgerEventRepository>) -> Self {
        Self { event_repository }
    }

    pub async fn handle(&self, command: CreateLedgerEventCommand) -> DomainResult<LedgerEvent> {
        info!(
            "Creating ledger event: account_id={}, type={}, amount={}",
            command.account_id, command.event_type, command.amount
        );

        let event_type = EventType::from_string(&command.event_type)?;
        let amount = Money::new(command.amount)?;

        let event = LedgerEvent::new(
            command.account_id,
            event_type,
            amount,
            command.description,
        );

        event.validate()?;

        let saved_event = self.event_repository.save(&event).await?;

        info!(
            "Ledger event created successfully: id={:?}",
            saved_event.id
        );

        Ok(saved_event)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::repositories::ledger_event_repository::MockLedgerEventRepository;

    #[tokio::test]
    async fn test_create_debit_event() {
        let mut mock_repo = MockLedgerEventRepository::new();

        mock_repo
            .expect_save()
            .once()
            .returning(|event| {
                let mut saved = event.clone();
                saved.id = Some(1);
                Ok(saved)
            });

        let handler = CreateLedgerEventHandler::new(Arc::new(mock_repo));
        let command = CreateLedgerEventCommand::new_debit(1, 1000, Some("Test debit".to_string()));

        let result = handler.handle(command).await;
        assert!(result.is_ok());

        let event = result.unwrap();
        assert_eq!(event.id, Some(1));
        assert_eq!(event.amount.value(), 1000);
    }
}

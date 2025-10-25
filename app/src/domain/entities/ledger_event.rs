use crate::domain::errors::{DomainError, DomainResult};
use crate::domain::value_objects::Money;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EventType {
    Debit,
    Credit,
}

impl EventType {
    pub fn from_string(s: &str) -> DomainResult<Self> {
        match s.to_uppercase().as_str() {
            "DEBIT" => Ok(EventType::Debit),
            "CREDIT" => Ok(EventType::Credit),
            _ => Err(DomainError::ValidationError(format!(
                "Invalid event type: {}",
                s
            ))),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            EventType::Debit => "DEBIT".to_string(),
            EventType::Credit => "CREDIT".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerEvent {
    pub id: Option<i32>,
    pub account_id: i32,
    pub event_type: EventType,
    pub amount: Money,
    pub description: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}

impl LedgerEvent {
    pub fn new(
        account_id: i32,
        event_type: EventType,
        amount: Money,
        description: Option<String>,
    ) -> Self {
        Self {
            id: None,
            account_id,
            event_type,
            amount,
            description,
            created_at: None,
        }
    }

    pub fn new_debit(account_id: i32, amount: Money, description: Option<String>) -> Self {
        Self::new(account_id, EventType::Debit, amount, description)
    }

    pub fn new_credit(account_id: i32, amount: Money, description: Option<String>) -> Self {
        Self::new(account_id, EventType::Credit, amount, description)
    }

    pub fn validate(&self) -> DomainResult<()> {
        if self.amount.value() <= 0 {
            return Err(DomainError::ValidationError(
                "Amount must be positive".to_string(),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_debit_event() {
        let amount = Money::new(1000).unwrap();
        let event = LedgerEvent::new_debit(1, amount, Some("Test debit".to_string()));

        assert_eq!(event.account_id, 1);
        assert_eq!(event.event_type, EventType::Debit);
        assert_eq!(event.amount.value(), 1000);
    }

    #[test]
    fn test_create_credit_event() {
        let amount = Money::new(500).unwrap();
        let event = LedgerEvent::new_credit(2, amount, Some("Test credit".to_string()));

        assert_eq!(event.account_id, 2);
        assert_eq!(event.event_type, EventType::Credit);
        assert_eq!(event.amount.value(), 500);
    }

    #[test]
    fn test_event_type_from_string() {
        assert_eq!(
            EventType::from_string("DEBIT").unwrap(),
            EventType::Debit
        );
        assert_eq!(
            EventType::from_string("CREDIT").unwrap(),
            EventType::Credit
        );
        assert!(EventType::from_string("INVALID").is_err());
    }

    #[test]
    fn test_event_type_to_string() {
        assert_eq!(EventType::Debit.to_string(), "DEBIT");
        assert_eq!(EventType::Credit.to_string(), "CREDIT");
    }

    #[test]
    fn test_validate() {
        let amount = Money::new(1000).unwrap();
        let event = LedgerEvent::new_debit(1, amount, None);
        assert!(event.validate().is_ok());
    }
}

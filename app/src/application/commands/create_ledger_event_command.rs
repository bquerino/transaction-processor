use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLedgerEventCommand {
    pub account_id: i32,
    pub event_type: String, // "DEBIT" or "CREDIT"
    pub amount: i64,
    pub description: Option<String>,
}

impl CreateLedgerEventCommand {
    pub fn new(
        account_id: i32,
        event_type: String,
        amount: i64,
        description: Option<String>,
    ) -> Self {
        Self {
            account_id,
            event_type,
            amount,
            description,
        }
    }

    pub fn new_debit(account_id: i32, amount: i64, description: Option<String>) -> Self {
        Self::new(account_id, "DEBIT".to_string(), amount, description)
    }

    pub fn new_credit(account_id: i32, amount: i64, description: Option<String>) -> Self {
        Self::new(account_id, "CREDIT".to_string(), amount, description)
    }
}

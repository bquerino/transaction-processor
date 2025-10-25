use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListLedgerEventsQuery {
    pub account_id: Option<i32>,
}

impl ListLedgerEventsQuery {
    pub fn new() -> Self {
        Self { account_id: None }
    }

    pub fn for_account(account_id: i32) -> Self {
        Self {
            account_id: Some(account_id),
        }
    }
}

impl Default for ListLedgerEventsQuery {
    fn default() -> Self {
        Self::new()
    }
}

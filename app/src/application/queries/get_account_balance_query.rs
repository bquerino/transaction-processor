use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAccountBalanceQuery {
    pub account_id: i32,
    pub use_snapshot: bool, // If true, use latest snapshot; if false, calculate from events
}

impl GetAccountBalanceQuery {
    pub fn new(account_id: i32) -> Self {
        Self {
            account_id,
            use_snapshot: false,
        }
    }

    pub fn with_snapshot(account_id: i32) -> Self {
        Self {
            account_id,
            use_snapshot: true,
        }
    }
}

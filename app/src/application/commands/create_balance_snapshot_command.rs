use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBalanceSnapshotCommand {
    pub account_id: i32,
}

impl CreateBalanceSnapshotCommand {
    pub fn new(account_id: i32) -> Self {
        Self { account_id }
    }
}

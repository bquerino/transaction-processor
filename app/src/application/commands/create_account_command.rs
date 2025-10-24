use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAccountCommand {
    pub account_number: String,
    pub account_name: String,
    pub initial_balance: i64,
}

impl CreateAccountCommand {
    pub fn new(account_number: String, account_name: String, initial_balance: i64) -> Self {
        Self {
            account_number,
            account_name,
            initial_balance,
        }
    }
}

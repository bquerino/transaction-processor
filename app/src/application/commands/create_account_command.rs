use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAccountCommand {
    pub account_number: String,
    pub account_name: String,
}

impl CreateAccountCommand {
    pub fn new(account_number: String, account_name: String) -> Self {
        Self {
            account_number,
            account_name,
        }
    }
}

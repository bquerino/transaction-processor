use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAccountQuery {
    pub account_id: i32,
}

impl GetAccountQuery {
    pub fn new(account_id: i32) -> Self {
        Self { account_id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAccountByNumberQuery {
    pub account_number: String,
}

impl GetAccountByNumberQuery {
    pub fn new(account_number: String) -> Self {
        Self { account_number }
    }
}

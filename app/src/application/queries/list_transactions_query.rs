use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListTransactionsQuery;

impl ListTransactionsQuery {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ListTransactionsQuery {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTransactionsByAccountQuery {
    pub account_id: i32,
}

impl GetTransactionsByAccountQuery {
    pub fn new(account_id: i32) -> Self {
        Self { account_id }
    }
}

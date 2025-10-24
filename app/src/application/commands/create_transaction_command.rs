use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTransactionCommand {
    pub from_account_id: Option<i32>,
    pub to_account_id: Option<i32>,
    pub amount: i64,
    pub transaction_type: String,
    pub description: Option<String>,
}

impl CreateTransactionCommand {
    pub fn new(
        from_account_id: Option<i32>,
        to_account_id: Option<i32>,
        amount: i64,
        transaction_type: String,
        description: Option<String>,
    ) -> Self {
        Self {
            from_account_id,
            to_account_id,
            amount,
            transaction_type,
            description,
        }
    }

    pub fn new_deposit(to_account_id: i32, amount: i64, description: Option<String>) -> Self {
        Self::new(
            None,
            Some(to_account_id),
            amount,
            "deposit".to_string(),
            description,
        )
    }

    pub fn new_withdrawal(from_account_id: i32, amount: i64, description: Option<String>) -> Self {
        Self::new(
            Some(from_account_id),
            None,
            amount,
            "withdrawal".to_string(),
            description,
        )
    }

    pub fn new_transfer(
        from_account_id: i32,
        to_account_id: i32,
        amount: i64,
        description: Option<String>,
    ) -> Self {
        Self::new(
            Some(from_account_id),
            Some(to_account_id),
            amount,
            "transfer".to_string(),
            description,
        )
    }
}

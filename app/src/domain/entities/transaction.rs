use crate::domain::value_objects::{Money, TransactionType};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Option<i32>,
    pub from_account_id: Option<i32>,
    pub to_account_id: Option<i32>,
    pub amount: Money,
    pub transaction_type: TransactionType,
    pub description: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}

impl Transaction {
    pub fn new(
        from_account_id: Option<i32>,
        to_account_id: Option<i32>,
        amount: Money,
        transaction_type: TransactionType,
        description: Option<String>,
    ) -> Self {
        Self {
            id: None,
            from_account_id,
            to_account_id,
            amount,
            transaction_type,
            description,
            created_at: None,
        }
    }

    pub fn new_deposit(to_account_id: i32, amount: Money, description: Option<String>) -> Self {
        Self::new(
            None,
            Some(to_account_id),
            amount,
            TransactionType::Deposit,
            description,
        )
    }

    pub fn new_withdrawal(
        from_account_id: i32,
        amount: Money,
        description: Option<String>,
    ) -> Self {
        Self::new(
            Some(from_account_id),
            None,
            amount,
            TransactionType::Withdrawal,
            description,
        )
    }

    pub fn new_transfer(
        from_account_id: i32,
        to_account_id: i32,
        amount: Money,
        description: Option<String>,
    ) -> Self {
        Self::new(
            Some(from_account_id),
            Some(to_account_id),
            amount,
            TransactionType::Transfer,
            description,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_deposit() {
        let amount = Money::new(1000).unwrap();
        let transaction = Transaction::new_deposit(1, amount, Some("Initial deposit".to_string()));

        assert_eq!(transaction.to_account_id, Some(1));
        assert_eq!(transaction.from_account_id, None);
        assert_eq!(transaction.amount.value(), 1000);
        assert_eq!(transaction.transaction_type, TransactionType::Deposit);
    }

    #[test]
    fn test_create_withdrawal() {
        let amount = Money::new(500).unwrap();
        let transaction =
            Transaction::new_withdrawal(1, amount, Some("ATM withdrawal".to_string()));

        assert_eq!(transaction.from_account_id, Some(1));
        assert_eq!(transaction.to_account_id, None);
        assert_eq!(transaction.amount.value(), 500);
        assert_eq!(transaction.transaction_type, TransactionType::Withdrawal);
    }

    #[test]
    fn test_create_transfer() {
        let amount = Money::new(750).unwrap();
        let transaction = Transaction::new_transfer(1, 2, amount, Some("Payment".to_string()));

        assert_eq!(transaction.from_account_id, Some(1));
        assert_eq!(transaction.to_account_id, Some(2));
        assert_eq!(transaction.amount.value(), 750);
        assert_eq!(transaction.transaction_type, TransactionType::Transfer);
    }
}

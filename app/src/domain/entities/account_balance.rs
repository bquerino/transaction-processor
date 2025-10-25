use crate::domain::value_objects::Money;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountBalance {
    pub id: Option<i32>,
    pub account_id: i32,
    pub balance: Money,
    pub snapshot_at: Option<NaiveDateTime>,
}

impl AccountBalance {
    pub fn new(account_id: i32, balance: Money) -> Self {
        Self {
            id: None,
            account_id,
            balance,
            snapshot_at: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_account_balance() {
        let balance = Money::new(5000).unwrap();
        let account_balance = AccountBalance::new(1, balance);

        assert_eq!(account_balance.account_id, 1);
        assert_eq!(account_balance.balance.value(), 5000);
        assert!(account_balance.id.is_none());
        assert!(account_balance.snapshot_at.is_none());
    }
}

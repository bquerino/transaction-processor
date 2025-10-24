use crate::domain::errors::{DomainError, DomainResult};
use crate::domain::value_objects::{AccountNumber, Money};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: Option<i32>,
    pub account_number: AccountNumber,
    pub account_name: String,
    pub balance: Money,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl Account {
    pub fn new(
        account_number: AccountNumber,
        account_name: String,
        initial_balance: Money,
    ) -> Self {
        Self {
            id: None,
            account_number,
            account_name,
            balance: initial_balance,
            created_at: None,
            updated_at: None,
        }
    }

    pub fn deposit(&mut self, amount: Money) -> DomainResult<()> {
        self.balance = self.balance.add(&amount)?;
        Ok(())
    }

    pub fn withdraw(&mut self, amount: Money) -> DomainResult<()> {
        self.balance = self.balance.subtract(&amount)?;
        Ok(())
    }

    pub fn can_withdraw(&self, amount: &Money) -> bool {
        self.balance.value() >= amount.value()
    }

    pub fn validate(&self) -> DomainResult<()> {
        if self.account_name.is_empty() {
            return Err(DomainError::ValidationError(
                "Account name cannot be empty".to_string(),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_account() {
        let account_number = AccountNumber::new("ACC001".to_string()).unwrap();
        let balance = Money::new(1000).unwrap();
        let account = Account::new(account_number, "Test Account".to_string(), balance);

        assert_eq!(account.account_name, "Test Account");
        assert_eq!(account.balance.value(), 1000);
    }

    #[test]
    fn test_deposit() {
        let account_number = AccountNumber::new("ACC001".to_string()).unwrap();
        let balance = Money::new(1000).unwrap();
        let mut account = Account::new(account_number, "Test Account".to_string(), balance);

        let deposit_amount = Money::new(500).unwrap();
        account.deposit(deposit_amount).unwrap();

        assert_eq!(account.balance.value(), 1500);
    }

    #[test]
    fn test_withdraw_success() {
        let account_number = AccountNumber::new("ACC001".to_string()).unwrap();
        let balance = Money::new(1000).unwrap();
        let mut account = Account::new(account_number, "Test Account".to_string(), balance);

        let withdraw_amount = Money::new(500).unwrap();
        account.withdraw(withdraw_amount).unwrap();

        assert_eq!(account.balance.value(), 500);
    }

    #[test]
    fn test_withdraw_insufficient_balance() {
        let account_number = AccountNumber::new("ACC001".to_string()).unwrap();
        let balance = Money::new(100).unwrap();
        let mut account = Account::new(account_number, "Test Account".to_string(), balance);

        let withdraw_amount = Money::new(500).unwrap();
        let result = account.withdraw(withdraw_amount);

        assert!(result.is_err());
    }

    #[test]
    fn test_can_withdraw() {
        let account_number = AccountNumber::new("ACC001".to_string()).unwrap();
        let balance = Money::new(1000).unwrap();
        let account = Account::new(account_number, "Test Account".to_string(), balance);

        assert!(account.can_withdraw(&Money::new(500).unwrap()));
        assert!(!account.can_withdraw(&Money::new(1500).unwrap()));
    }

    #[test]
    fn test_validate() {
        let account_number = AccountNumber::new("ACC001".to_string()).unwrap();
        let balance = Money::new(1000).unwrap();
        let account = Account::new(account_number.clone(), "Test Account".to_string(), balance);

        assert!(account.validate().is_ok());

        let invalid_account = Account::new(account_number, "".to_string(), balance);
        assert!(invalid_account.validate().is_err());
    }
}

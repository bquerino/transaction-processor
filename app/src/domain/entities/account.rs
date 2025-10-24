use crate::domain::errors::{DomainError, DomainResult};
use crate::domain::value_objects::AccountNumber;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: Option<i32>,
    pub account_number: AccountNumber,
    pub account_name: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl Account {
    pub fn new(account_number: AccountNumber, account_name: String) -> Self {
        Self {
            id: None,
            account_number,
            account_name,
            created_at: None,
            updated_at: None,
        }
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
        let account = Account::new(account_number, "Test Account".to_string());

        assert_eq!(account.account_name, "Test Account");
        assert!(account.id.is_none());
    }

    #[test]
    fn test_validate() {
        let account_number = AccountNumber::new("ACC001".to_string()).unwrap();
        let account = Account::new(account_number.clone(), "Test Account".to_string());

        assert!(account.validate().is_ok());

        let invalid_account = Account::new(account_number, "".to_string());
        assert!(invalid_account.validate().is_err());
    }
}

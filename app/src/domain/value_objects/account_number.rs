use crate::domain::errors::{DomainError, DomainResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AccountNumber(String);

impl AccountNumber {
    pub fn new(value: String) -> DomainResult<Self> {
        if value.is_empty() {
            return Err(DomainError::InvalidAccountNumber(
                "Account number cannot be empty".to_string(),
            ));
        }

        if value.len() > 50 {
            return Err(DomainError::InvalidAccountNumber(
                "Account number too long".to_string(),
            ));
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for AccountNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

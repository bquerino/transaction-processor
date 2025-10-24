use crate::domain::errors::{DomainError, DomainResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Money(i64);

impl Money {
    pub fn new(amount: i64) -> DomainResult<Self> {
        if amount < 0 {
            return Err(DomainError::InvalidAmount(amount));
        }
        Ok(Self(amount))
    }

    pub fn zero() -> Self {
        Self(0)
    }

    pub fn value(&self) -> i64 {
        self.0
    }

    pub fn add(&self, other: &Money) -> DomainResult<Money> {
        self.0
            .checked_add(other.0)
            .map(Money)
            .ok_or_else(|| DomainError::ValidationError("Amount overflow".to_string()))
    }

    pub fn subtract(&self, other: &Money) -> DomainResult<Money> {
        if self.0 < other.0 {
            return Err(DomainError::InsufficientBalance {
                required: other.0,
                available: self.0,
            });
        }
        Ok(Money(self.0 - other.0))
    }
}

impl std::fmt::Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Account not found: {0}")]
    AccountNotFound(String),

    #[error("Transaction not found: {0}")]
    TransactionNotFound(i32),

    #[error("Insufficient balance: required {required}, available {available}")]
    InsufficientBalance { required: i64, available: i64 },

    #[error("Invalid amount: {0}")]
    InvalidAmount(i64),

    #[error("Invalid account number: {0}")]
    InvalidAccountNumber(String),

    #[error("Invalid transaction type: {0}")]
    InvalidTransactionType(String),

    #[error("Duplicate account number: {0}")]
    DuplicateAccountNumber(String),

    #[error("Repository error: {0}")]
    RepositoryError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),
}

pub type DomainResult<T> = Result<T, DomainError>;

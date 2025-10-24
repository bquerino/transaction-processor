pub mod entities;
pub mod errors;
pub mod repositories;
pub mod services;
pub mod value_objects;

pub use entities::{Account, Transaction};
pub use errors::{DomainError, DomainResult};
pub use repositories::{AccountRepository, TransactionRepository};
pub use services::TransactionService;
pub use value_objects::{AccountNumber, Money, TransactionType};

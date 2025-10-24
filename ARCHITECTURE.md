# Architecture Documentation

## Domain-Driven Design (DDD) Implementation

This application implements a **clean DDD architecture** with clear separation of concerns across four distinct layers.

## Layer Overview

```
┌─────────────────────────────────────────────────────────┐
│                     API Layer                           │
│                    (main.rs)                            │
│  - Entry point                                          │
│  - Mediator initialization                              │
│  - Structured logging setup                             │
└─────────────────────┬───────────────────────────────────┘
                      │
                      ▼
┌─────────────────────────────────────────────────────────┐
│               Application Layer                         │
│             (application/)                              │
│  ┌─────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │  Commands   │  │   Queries    │  │   Handlers   │  │
│  │  (CQRS)     │  │   (CQRS)     │  │              │  │
│  └─────────────┘  └──────────────┘  └──────────────┘  │
│  ┌──────────────────────────────────────────────────┐  │
│  │            Mediator (Dispatcher)                 │  │
│  └──────────────────────────────────────────────────┘  │
└─────────────────────┬───────────────────────────────────┘
                      │
                      ▼
┌─────────────────────────────────────────────────────────┐
│                  Domain Layer                           │
│                   (domain/)                             │
│  ┌─────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │  Entities   │  │Value Objects │  │  Services    │  │
│  │  (Account,  │  │  (Money,     │  │(Transaction  │  │
│  │Transaction) │  │AccountNumber)│  │  Service)    │  │
│  └─────────────┘  └──────────────┘  └──────────────┘  │
│  ┌──────────────────────────────────────────────────┐  │
│  │         Repository Interfaces                    │  │
│  └──────────────────────────────────────────────────┘  │
└─────────────────────┬───────────────────────────────────┘
                      │
                      ▼
┌─────────────────────────────────────────────────────────┐
│              Infrastructure Layer                       │
│              (infrastructure/)                          │
│  ┌──────────────────────────────────────────────────┐  │
│  │      Repository Implementations                  │  │
│  │  - DieselAccountRepository                       │  │
│  │  - DieselTransactionRepository                   │  │
│  └──────────────────────────────────────────────────┘  │
│  ┌──────────────────────────────────────────────────┐  │
│  │           Database (PostgreSQL)                  │  │
│  └──────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────┘
```

## Design Patterns

### 1. Mediator Pattern

The **Mediator** acts as a central dispatcher for all commands and queries:

```rust
// API Layer creates command
let command = CreateAccountCommand::new(...);

// Mediator dispatches to appropriate handler
let account = mediator.send_create_account(command).await?;

// Handler processes using domain services and repositories
```

**Benefits:**
- Decouples sender from receiver
- Single point of entry for all operations
- Easy to add new handlers without changing existing code

### 2. CQRS (Command Query Responsibility Segregation)

**Commands** (Write operations):
- `CreateAccountCommand`
- `CreateTransactionCommand`

**Queries** (Read operations):
- `GetAccountQuery`
- `ListAccountsQuery`
- `ListTransactionsQuery`

**Benefits:**
- Clear separation between reads and writes
- Optimized for different concerns
- Easier to scale independently

### 3. Repository Pattern

**Abstract Interfaces** (Domain Layer):
```rust
#[async_trait]
pub trait AccountRepository: Send + Sync {
    async fn save(&self, account: &Account) -> DomainResult<Account>;
    async fn find_by_id(&self, id: i32) -> DomainResult<Account>;
    // ...
}
```

**Concrete Implementations** (Infrastructure Layer):
```rust
pub struct DieselAccountRepository {
    pool: Pool<ConnectionManager<PgConnection>>,
}
```

**Benefits:**
- Domain layer independent of infrastructure
- Easy to swap implementations (e.g., different databases)
- Testable with mocks

### 4. Value Objects

Immutable objects with validation:

```rust
pub struct Money(i64);
pub struct AccountNumber(String);
pub struct TransactionType(enum);
```

**Benefits:**
- Encapsulates validation logic
- Prevents invalid states
- Self-documenting code

## Request Flow Example: Create Account

1. **API Layer** (`main.rs`)
   ```rust
   let command = CreateAccountCommand::new("ACC001", "Main", 10000);
   let result = mediator.send_create_account(command).await;
   ```
   - Logs: "Creating account: number=ACC001, name=Main, balance=10000"

2. **Application Layer** (`CreateAccountHandler`)
   ```rust
   // Validate inputs
   let account_number = AccountNumber::new(command.account_number)?;
   let balance = Money::new(command.initial_balance)?;
   
   // Check business rules
   if repository.exists_by_account_number(&account_number).await? {
       return Err(DomainError::DuplicateAccountNumber);
   }
   
   // Create entity
   let account = Account::new(account_number, command.account_name, balance);
   ```
   - Logs: "Account created successfully: id=Some(1)"

3. **Domain Layer** (`Account` entity)
   ```rust
   pub fn new(...) -> Self {
       // Business logic
       // Validation
   }
   
   pub fn validate(&self) -> DomainResult<()> {
       // Validate account invariants
   }
   ```

4. **Infrastructure Layer** (`DieselAccountRepository`)
   ```rust
   async fn save(&self, account: &Account) -> DomainResult<Account> {
       // Convert to Diesel model
       // Save to database
       // Convert back to domain entity
   }
   ```
   - Logs: "Account saved to database: id=1"

## Structured Logging

Implemented using **tracing** with proper severity levels:

```rust
use tracing::{info, error, warn};

// Info level - normal operations
info!("Processing deposit: account_id={}, amount={}", id, amount);

// Error level - failures
error!("Insufficient balance: required={}, available={}", req, avail);
```

**Log Format:**
```
2025-10-24T04:03:15.064106Z  INFO ThreadId(01) src/main.rs:23: Transaction Processor
2025-10-24T04:03:15.150223Z  INFO ThreadId(01) src/application/handlers/create_account_handler.rs:16: Creating account
2025-10-24T04:03:15.159154Z  INFO ThreadId(01) src/infrastructure/persistence/diesel_account_repository.rs:44: Account saved
```

**Features:**
- Timestamp with microsecond precision
- Thread ID
- Source file and line number
- Severity level (INFO, ERROR, WARN)
- Structured context

## Error Handling

Custom domain errors using **thiserror**:

```rust
#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Account not found: {0}")]
    AccountNotFound(String),

    #[error("Insufficient balance: required {required}, available {available}")]
    InsufficientBalance { required: i64, available: i64 },
    
    // ... more error types
}

pub type DomainResult<T> = Result<T, DomainError>;
```

**Benefits:**
- Type-safe error handling
- Rich error context
- Clear error messages
- Easy error propagation with `?` operator

## Testing Strategy

### Unit Tests (14 tests)

**Domain Entity Tests:**
- Account creation, deposit, withdrawal, validation
- Transaction creation for different types

**Handler Tests:**
- CreateAccountHandler with success and error cases
- GetAccountHandler

**Service Tests:**
- TransactionService deposit/withdrawal logic
- Business rule validation

**Mocking:**
Uses **mockall** to mock repository dependencies:

```rust
mock! {
    pub AccountRepo {}
    
    #[async_trait]
    impl AccountRepository for AccountRepo {
        async fn save(&self, account: &Account) -> DomainResult<Account>;
        // ...
    }
}
```

## Best Practices Implemented

1. ✅ **Separation of Concerns**: Each layer has a single responsibility
2. ✅ **Dependency Inversion**: Domain depends on abstractions, not implementations
3. ✅ **Encapsulation**: Business logic in domain entities
4. ✅ **Immutability**: Value objects are immutable
5. ✅ **Validation**: Input validation at boundaries
6. ✅ **Type Safety**: Strong typing with custom types
7. ✅ **Async/Await**: Non-blocking I/O operations
8. ✅ **Structured Logging**: Rich contextual logs
9. ✅ **Error Handling**: Proper error types and propagation
10. ✅ **Testing**: Comprehensive unit tests with mocking

## Technology Choices

- **Rust**: Memory safety, performance, type safety
- **Diesel**: Type-safe SQL query builder
- **Tokio**: Async runtime for scalability
- **Tracing**: Structured, async-aware logging
- **Thiserror**: Ergonomic error handling
- **Mockall**: Mock generation for testing
- **PostgreSQL**: Robust ACID-compliant database

## Future Enhancements

Potential improvements to consider:

1. **Integration Tests**: Full end-to-end tests with test database
2. **API Layer**: REST API with Axum or Actix-web
3. **Events**: Domain events for pub/sub patterns
4. **Saga Pattern**: Distributed transaction handling
5. **Metrics**: Prometheus metrics for monitoring
6. **Validation**: More sophisticated validation rules
7. **Caching**: Redis for frequently accessed data
8. **Authentication**: JWT-based auth for API
9. **Rate Limiting**: Protection against abuse
10. **Audit Log**: Track all state changes

# Transaction Processor - Ledger Application

A Rust-based ledger application using Diesel ORM and PostgreSQL for managing accounts and transactions.

## Architecture

This project follows **Domain-Driven Design (DDD)** principles with a clean, layered architecture:

```
transaction-processor/
├── app/                                      # Application code
│   ├── src/
│   │   ├── main.rs                          # API Layer - Entry point with Mediator
│   │   ├── lib.rs                           # Module exports
│   │   ├── application/                     # Application Layer
│   │   │   ├── commands/                    # Command definitions (CQRS)
│   │   │   │   ├── create_account_command.rs
│   │   │   │   └── create_transaction_command.rs
│   │   │   ├── queries/                     # Query definitions (CQRS)
│   │   │   │   ├── get_account_query.rs
│   │   │   │   ├── list_accounts_query.rs
│   │   │   │   └── list_transactions_query.rs
│   │   │   ├── handlers/                    # Command & Query handlers
│   │   │   │   ├── create_account_handler.rs
│   │   │   │   ├── create_transaction_handler.rs
│   │   │   │   ├── get_account_handler.rs
│   │   │   │   ├── list_accounts_handler.rs
│   │   │   │   └── list_transactions_handler.rs
│   │   │   └── mediator.rs                  # Mediator pattern dispatcher
│   │   ├── domain/                          # Domain Layer (Core)
│   │   │   ├── entities/                    # Domain entities (aggregates)
│   │   │   │   ├── account.rs
│   │   │   │   └── transaction.rs
│   │   │   ├── value_objects/               # Value objects
│   │   │   │   ├── account_number.rs
│   │   │   │   ├── money.rs
│   │   │   │   └── transaction_type.rs
│   │   │   ├── repositories/                # Repository interfaces
│   │   │   │   ├── account_repository.rs
│   │   │   │   └── transaction_repository.rs
│   │   │   ├── services/                    # Domain services
│   │   │   │   └── transaction_service.rs
│   │   │   └── errors.rs                    # Domain errors
│   │   ├── infrastructure/                  # Infrastructure Layer
│   │   │   └── persistence/                 # Repository implementations
│   │   │       ├── diesel_account_repository.rs
│   │   │       └── diesel_transaction_repository.rs
│   │   ├── models.rs                        # Diesel ORM models (legacy)
│   │   └── schema.rs                        # Database schema
│   ├── migrations/                          # Database migrations
│   ├── Cargo.toml                           # Rust dependencies
│   └── .env.example                         # Environment variables
└── infra/                                   # Infrastructure configuration
    ├── docker-compose.yml                   # PostgreSQL container
    └── init.sql                             # Database initialization
```

### DDD Layers

#### 1. **API Layer** (`main.rs`)
- Entry point for the application
- Uses **Mediator pattern** to dispatch commands and queries
- Implements **structured logging** with tracing
- Handles errors gracefully with proper logging

#### 2. **Application Layer** (`application/`)
- **Commands**: Write operations (CreateAccount, CreateTransaction)
- **Queries**: Read operations (GetAccount, ListAccounts, ListTransactions)
- **Handlers**: Process commands and queries
- **Mediator**: Central dispatcher routing requests to appropriate handlers
- Implements **CQRS** (Command Query Responsibility Segregation)

#### 3. **Domain Layer** (`domain/`)
- **Entities**: Core business objects (Account, Transaction)
- **Value Objects**: Immutable objects with validation (Money, AccountNumber, TransactionType)
- **Repository Interfaces**: Abstract contracts for data access
- **Domain Services**: Business logic (TransactionService for deposits, withdrawals, transfers)
- **Domain Errors**: Custom error types with proper semantics
- Contains **all business rules and validation**

#### 4. **Infrastructure Layer** (`infrastructure/`)
- **Repository Implementations**: Concrete implementations using Diesel ORM
- Database connection management
- Translates between domain entities and database models

### Design Patterns & Best Practices

✅ **Domain-Driven Design (DDD)**: Clear separation of domain, application, and infrastructure
✅ **Mediator Pattern**: Centralized command/query dispatching
✅ **CQRS**: Separate commands and queries
✅ **Repository Pattern**: Abstract data access with interfaces
✅ **Value Objects**: Immutable objects with validation
✅ **Dependency Injection**: Repositories injected into handlers
✅ **Structured Logging**: tracing with proper severities (INFO, ERROR)
✅ **Error Handling**: Custom domain errors with thiserror
✅ **Unit Testing**: 14 comprehensive tests with mocking (mockall)
✅ **Async/Await**: Full async support with tokio

### Request Flow Example

Here's how a "Create Account" request flows through the architecture:

```
1. API Layer (main.rs)
   └─> Creates CreateAccountCommand
   └─> Calls mediator.send_create_account(command)

2. Application Layer (Mediator)
   └─> Routes to CreateAccountHandler
   └─> Handler validates input
   └─> Creates domain value objects (AccountNumber, Money)
   └─> Creates domain entity (Account)
   └─> Validates business rules

3. Domain Layer
   └─> Account entity validates itself
   └─> Checks business invariants
   └─> Returns validated domain entity

4. Infrastructure Layer
   └─> DieselAccountRepository saves to database
   └─> Translates domain entity to Diesel model
   └─> Persists to PostgreSQL
   └─> Returns saved entity

5. Response
   └─> Success/Error propagates back through layers
   └─> Structured logging at each layer
   └─> Proper error handling with domain-specific errors
```

### Database Schema

#### Accounts Table
- `id`: Primary key (auto-increment)
- `account_number`: Unique account identifier
- `account_name`: Name of the account holder
- `balance`: Current balance (stored as bigint for precision)
- `created_at`: Timestamp of account creation
- `updated_at`: Timestamp of last update

#### Transactions Table
- `id`: Primary key (auto-increment)
- `from_account_id`: Source account (nullable for deposits)
- `to_account_id`: Destination account (nullable for withdrawals)
- `amount`: Transaction amount (must be positive)
- `transaction_type`: Type of transaction (e.g., deposit, withdrawal, transfer)
- `description`: Optional transaction description
- `created_at`: Timestamp of transaction

## Prerequisites

- **Rust** (1.70+): Install from [rustup.rs](https://rustup.rs/)
- **Docker** & **Docker Compose**: For running PostgreSQL
- **Diesel CLI**: Install with `cargo install diesel_cli --no-default-features --features postgres`

## How to Run Locally

### Quick Start (Recommended)

The easiest way to get started is using the provided Makefile:

```bash
# Complete setup (first time)
make setup

# Run the application
make run

# Stop PostgreSQL
make stop
```

For all available commands, run:
```bash
make help
```

### Alternative: Automated Setup Script

You can also use the setup script directly:

```bash
./setup.sh
```

This script will:
- Check prerequisites (Rust, Docker, Diesel CLI)
- Start PostgreSQL with docker compose
- Create the `.env` file
- Run database migrations
- Build the application

### Manual Setup

If you prefer to set up manually, follow these steps:

### 1. Start the PostgreSQL Database

Navigate to the `infra` directory and start the PostgreSQL container:

```bash
cd infra
docker-compose up -d
```

Verify the database is running:

```bash
docker-compose ps
```

### 2. Set Up Environment Variables

Copy the example environment file and configure it:

```bash
cd ../app
cp .env.example .env
```

The default `.env` should contain:
```
DATABASE_URL=postgres://ledger_user:ledger_password@localhost:5432/ledger_db
```

### 3. Run Database Migrations

Apply the database migrations using Diesel CLI:

```bash
diesel migration run
```

To revert the last migration (if needed):
```bash
diesel migration revert
```

### 4. Build and Run the Application

Build the application:

```bash
cargo build
```

Run the application:

```bash
cargo run
```

For production builds:
```bash
cargo build --release
./target/release/transaction-processor
```

## Development

### Running Tests

The project includes 14 comprehensive unit tests covering domain entities, handlers, and services:

```bash
cargo test
```

Or using the Makefile:
```bash
make test
```

**Test Coverage**:
- Domain entity tests (9 tests)
- Command handler tests (2 tests)
- Query handler tests (1 test)
- Domain service tests (2 tests)

Tests use **mockall** for mocking repository dependencies.

### Code Formatting

```bash
cargo fmt
```

### Linting

```bash
cargo clippy
```

### Adding New Migrations

To create a new migration:

```bash
diesel migration generate <migration_name>
```

This creates a new directory in `migrations/` with `up.sql` and `down.sql` files.

## Available Operations

The application provides the following core operations:

- **Create Account**: Create a new account with initial balance
- **Create Transaction**: Record a new transaction (deposit, withdrawal, transfer)
- **Get Account Balance**: Retrieve the current balance of an account
- **List Accounts**: Get all accounts
- **List Transactions**: Get all transactions

## Stopping the Infrastructure

To stop the PostgreSQL container:

```bash
cd infra
docker-compose down
```

To stop and remove all data:

```bash
docker-compose down -v
```

## Technology Stack

- **Language**: Rust 2021 Edition
- **Architecture**: Domain-Driven Design (DDD)
- **Patterns**: Mediator, CQRS, Repository
- **ORM**: Diesel 2.3
- **Database**: PostgreSQL 16
- **Connection Pooling**: R2D2
- **Async Runtime**: Tokio
- **Logging**: tracing + tracing-subscriber
- **Error Handling**: thiserror + anyhow
- **Testing**: mockall for mocking
- **Serialization**: Serde + Serde JSON
- **Date/Time**: Chrono

## License

See LICENSE file for details.
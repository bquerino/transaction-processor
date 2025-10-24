# Transaction Processor REST API Documentation

## Overview

The Transaction Processor is an event-sourced ledger application that provides a REST API for managing accounts and financial transactions using DEBIT/CREDIT events.

## Architecture

### Event-Sourcing Model

The application uses **Event Sourcing** as its core pattern:

- **Ledger Events**: All financial operations are recorded as immutable DEBIT or CREDIT events
- **Append-Only**: Events are never modified or deleted, only appended
- **Balance Calculation**: Account balances are calculated by replaying events
- **Snapshots**: Balance snapshots can be created for performance optimization

### Database Schema

#### Tables

1. **accounts**
   - `id`: Primary key
   - `account_number`: Unique account identifier
   - `account_name`: Account holder name
   - `created_at`: Creation timestamp
   - `updated_at`: Last update timestamp

2. **ledger_events** (Append-Only)
   - `id`: Primary key
   - `account_id`: Reference to account
   - `event_type`: "DEBIT" or "CREDIT"
   - `amount`: Transaction amount (always positive)
   - `description`: Optional description
   - `created_at`: Event timestamp

3. **account_balances** (Snapshots)
   - `id`: Primary key
   - `account_id`: Reference to account
   - `balance`: Balance at snapshot time
   - `snapshot_at`: Snapshot timestamp

## API Endpoints

### Base URL

```
http://localhost:3000
```

### Accounts

#### Create Account

Creates a new account without an initial balance.

```http
POST /accounts
Content-Type: application/json

{
  "account_number": "ACC001",
  "account_name": "Main Account"
}
```

**Response:**
```json
{
  "id": 1,
  "account_number": "ACC001",
  "account_name": "Main Account",
  "created_at": "2025-10-24T04:54:50.534171",
  "updated_at": "2025-10-24T04:54:50.534171"
}
```

#### Get Account

Retrieves account details by ID.

```http
GET /accounts/:id
```

**Response:**
```json
{
  "id": 1,
  "account_number": "ACC001",
  "account_name": "Main Account",
  "created_at": "2025-10-24T04:54:50.534171",
  "updated_at": "2025-10-24T04:54:50.534171"
}
```

#### List Accounts

Lists all accounts in the system.

```http
GET /accounts
```

**Response:**
```json
{
  "accounts": [
    {
      "id": 1,
      "account_number": "ACC001",
      "account_name": "Main Account",
      "created_at": "2025-10-24T04:54:50.534171",
      "updated_at": "2025-10-24T04:54:50.534171"
    }
  ],
  "count": 1
}
```

#### Get Account Balance

Calculates and returns the current balance for an account.

```http
GET /accounts/:id/balance
```

**Response:**
```json
{
  "account_id": 1,
  "balance": 3500,
  "snapshot_at": null
}
```

### Ledger Events

#### Create Ledger Event

Records a DEBIT or CREDIT event.

**CREDIT Example (Deposit):**
```http
POST /events
Content-Type: application/json

{
  "account_id": 1,
  "event_type": "CREDIT",
  "amount": 5000,
  "description": "Initial deposit"
}
```

**DEBIT Example (Withdrawal):**
```http
POST /events
Content-Type: application/json

{
  "account_id": 1,
  "event_type": "DEBIT",
  "amount": 1500,
  "description": "ATM withdrawal"
}
```

**Response:**
```json
{
  "id": 1,
  "account_id": 1,
  "event_type": "CREDIT",
  "amount": 5000,
  "description": "Initial deposit",
  "created_at": "2025-10-24T04:55:00.623629"
}
```

#### List Ledger Events

Lists all ledger events, optionally filtered by account.

**All Events:**
```http
GET /events
```

**Filtered by Account:**
```http
GET /events?account_id=1
```

**Response:**
```json
{
  "events": [
    {
      "id": 1,
      "account_id": 1,
      "event_type": "CREDIT",
      "amount": 5000,
      "description": "Initial deposit",
      "created_at": "2025-10-24T04:55:00.623629"
    },
    {
      "id": 2,
      "account_id": 1,
      "event_type": "DEBIT",
      "amount": 1500,
      "description": "ATM withdrawal",
      "created_at": "2025-10-24T04:55:09.878069"
    }
  ],
  "count": 2
}
```

### Balance Snapshots

#### Create Balance Snapshot

Creates a snapshot of an account's current balance for performance optimization.

```http
POST /balances/snapshot
Content-Type: application/json

{
  "account_id": 1
}
```

**Response:**
```json
{
  "id": 1,
  "account_id": 1,
  "balance": 3500,
  "snapshot_at": "2025-10-24T04:55:28.971584"
}
```

## Example Usage Scenarios

### Scenario 1: Create Account and Deposit Money

```bash
# 1. Create account
curl -X POST http://localhost:3000/accounts \
  -H "Content-Type: application/json" \
  -d '{"account_number": "ACC001", "account_name": "John Doe"}'

# 2. Deposit $1000
curl -X POST http://localhost:3000/events \
  -H "Content-Type: application/json" \
  -d '{"account_id": 1, "event_type": "CREDIT", "amount": 1000, "description": "Initial deposit"}'

# 3. Check balance
curl http://localhost:3000/accounts/1/balance
```

### Scenario 2: Withdraw Money

```bash
# Withdraw $200
curl -X POST http://localhost:3000/events \
  -H "Content-Type: application/json" \
  -d '{"account_id": 1, "event_type": "DEBIT", "amount": 200, "description": "ATM withdrawal"}'

# Check new balance
curl http://localhost:3000/accounts/1/balance
```

### Scenario 3: View Transaction History

```bash
# Get all events for an account
curl http://localhost:3000/events?account_id=1
```

### Scenario 4: Create Balance Snapshot

```bash
# Create a snapshot for performance
curl -X POST http://localhost:3000/balances/snapshot \
  -H "Content-Type: application/json" \
  -d '{"account_id": 1}'
```

## Balance Calculation

Balances are calculated by summing all CREDIT and DEBIT events:

```
Balance = Σ(CREDIT events) - Σ(DEBIT events)
```

For example:
- Account created: Balance = 0
- CREDIT $5000: Balance = 5000
- DEBIT $1500: Balance = 3500
- CREDIT $2000: Balance = 5500

## Error Handling

The API returns appropriate HTTP status codes:

- `200 OK`: Successful operation
- `500 Internal Server Error`: Server error with error message in JSON

Example error response:
```json
{
  "error": "Account not found: Account with id 999 not found"
}
```

## Running the Application

### Prerequisites

- Rust 1.70+
- Docker & Docker Compose
- PostgreSQL 16 (via Docker)

### Quick Start

```bash
# 1. Start PostgreSQL
cd infra && docker compose up -d

# 2. Set up environment
cd ../app && cp .env.example .env

# 3. Run migrations
diesel migration run

# 4. Start the server
cargo run
```

The server will start on `http://0.0.0.0:3000`

## Technology Stack

- **Language**: Rust 2021
- **Web Framework**: Axum 0.7
- **Database**: PostgreSQL 16
- **ORM**: Diesel 2.3
- **Async Runtime**: Tokio
- **Logging**: tracing + tracing-subscriber
- **Architecture**: Domain-Driven Design (DDD)
- **Pattern**: Event Sourcing

## Key Features

✅ **Event-Sourced Architecture**: Immutable event log  
✅ **Append-Only Ledger**: All events are preserved  
✅ **ACID Compliance**: PostgreSQL transactions  
✅ **Balance Snapshots**: Performance optimization  
✅ **RESTful API**: Clean HTTP interface  
✅ **Structured Logging**: Detailed operation logs  
✅ **Type Safety**: Rust's strong type system  
✅ **DDD Architecture**: Clean separation of concerns  

## Security

- No sensitive data logged
- SQL injection prevention (Diesel ORM)
- Input validation at domain layer
- Type-safe operations
- Zero security vulnerabilities (CodeQL verified)

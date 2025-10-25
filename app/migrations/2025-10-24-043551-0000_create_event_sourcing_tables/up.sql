-- Create ledger_events table for append-only DEBIT/CREDIT events
CREATE TABLE ledger_events (
    id SERIAL PRIMARY KEY,
    account_id INTEGER NOT NULL REFERENCES accounts(id),
    event_type VARCHAR(20) NOT NULL CHECK (event_type IN ('DEBIT', 'CREDIT')),
    amount BIGINT NOT NULL CHECK (amount > 0),
    description TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes for better query performance
CREATE INDEX idx_ledger_events_account_id ON ledger_events(account_id);
CREATE INDEX idx_ledger_events_created_at ON ledger_events(created_at);
CREATE INDEX idx_ledger_events_account_event ON ledger_events(account_id, created_at);

-- Create account_balances table for snapshots/checkpoints
CREATE TABLE account_balances (
    id SERIAL PRIMARY KEY,
    account_id INTEGER NOT NULL REFERENCES accounts(id),
    balance BIGINT NOT NULL,
    snapshot_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(account_id, snapshot_at)
);

-- Create index for faster lookups
CREATE INDEX idx_account_balances_account_id ON account_balances(account_id);
CREATE INDEX idx_account_balances_snapshot_at ON account_balances(snapshot_at);

-- Remove balance column from accounts table since we'll calculate it from events
ALTER TABLE accounts DROP COLUMN balance;

-- Drop the transactions table as we're moving to event-sourcing
DROP TABLE transactions;

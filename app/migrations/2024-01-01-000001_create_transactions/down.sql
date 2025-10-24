-- Drop transactions table and related objects
DROP INDEX IF EXISTS idx_transactions_from_account;
DROP INDEX IF EXISTS idx_transactions_to_account;
DROP INDEX IF EXISTS idx_transactions_created_at;
DROP TABLE IF EXISTS transactions;

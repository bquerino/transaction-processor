-- Initialize the ledger database
-- This file is automatically executed when the PostgreSQL container starts

\c ledger_db;

-- Create any additional extensions if needed
-- Example: CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- The actual schema will be created by Diesel migrations

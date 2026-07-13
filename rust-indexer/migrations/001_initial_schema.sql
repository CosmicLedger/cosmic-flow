-- Create ledgers table
CREATE TABLE IF NOT EXISTS ledgers (
    id BIGSERIAL PRIMARY KEY,
    sequence BIGINT NOT NULL UNIQUE,
    hash VARCHAR(64) NOT NULL UNIQUE,
    previous_hash VARCHAR(64) NOT NULL,
    transaction_count INTEGER NOT NULL DEFAULT 0,
    operation_count INTEGER NOT NULL DEFAULT 0,
    closed_at TIMESTAMP WITH TIME ZONE NOT NULL,
    total_coins VARCHAR(255) NOT NULL,
    fee_pool VARCHAR(255) NOT NULL,
    base_fee INTEGER NOT NULL,
    base_reserve INTEGER NOT NULL,
    max_tx_set_size INTEGER NOT NULL,
    protocol_version INTEGER NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_ledgers_sequence ON ledgers(sequence DESC);
CREATE INDEX idx_ledgers_closed_at ON ledgers(closed_at DESC);

-- Create transactions table
CREATE TABLE IF NOT EXISTS transactions (
    id BIGSERIAL PRIMARY KEY,
    hash VARCHAR(64) NOT NULL UNIQUE,
    ledger_sequence BIGINT NOT NULL REFERENCES ledgers(sequence),
    source_account VARCHAR(56) NOT NULL,
    fee_charged BIGINT NOT NULL,
    operation_count INTEGER NOT NULL DEFAULT 0,
    successful BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_transactions_hash ON transactions(hash);
CREATE INDEX idx_transactions_ledger ON transactions(ledger_sequence);
CREATE INDEX idx_transactions_source ON transactions(source_account);
CREATE INDEX idx_transactions_created_at ON transactions(created_at DESC);

-- Create operations table
CREATE TABLE IF NOT EXISTS operations (
    id BIGSERIAL PRIMARY KEY,
    transaction_hash VARCHAR(64) NOT NULL REFERENCES transactions(hash),
    operation_type VARCHAR(50) NOT NULL,
    source_account VARCHAR(56),
    details JSONB NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_operations_tx_hash ON operations(transaction_hash);
CREATE INDEX idx_operations_type ON operations(operation_type);
CREATE INDEX idx_operations_source ON operations(source_account);

-- Create accounts table
CREATE TABLE IF NOT EXISTS accounts (
    id VARCHAR(56) PRIMARY KEY,
    sequence VARCHAR(255) NOT NULL,
    balance VARCHAR(255) NOT NULL,
    last_modified_ledger BIGINT NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_accounts_last_modified ON accounts(last_modified_ledger);

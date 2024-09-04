CREATE TABLE accounts (
    pubkey TEXT PRIMARY KEY,
    lamports BIGINT NOT NULL,
    owner TEXT NOT NULL,
    executable BOOLEAN NOT NULL,
    rent_epoch BIGINT NOT NULL,
    data BYTEA NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

CREATE TABLE transactions (
    signature TEXT PRIMARY KEY,
    slot BIGINT NOT NULL,
    err TEXT,
    memo TEXT,
    block_time BIGINT,
    created_at TIMESTAMP NOT NULL
);

CREATE INDEX idx_transactions_slot ON transactions (slot);
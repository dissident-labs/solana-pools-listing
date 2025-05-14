CREATE TABLE IF NOT EXISTS meteora_pools (
    pool_address TEXT PRIMARY KEY,
    pool_name TEXT NOT NULL,
    token_a_mint TEXT NOT NULL,
    token_b_mint TEXT NOT NULL,
    token_a_symbol TEXT NOT NULL,
    token_b_symbol TEXT NOT NULL,
    liquidity DOUBLE PRECISION NOT NULL,
    tvl DOUBLE PRECISION NOT NULL,
    apr DOUBLE PRECISION NOT NULL,
    volume24h DOUBLE PRECISION NOT NULL,
    fee24h DOUBLE PRECISION NOT NULL,
    pool_price DOUBLE PRECISION NOT NULL,
    created_at_slot_timestamp BIGINT NOT NULL,
    updated_at BIGINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

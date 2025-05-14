CREATE TABLE IF NOT EXISTS vaults (
    vault_address TEXT PRIMARY KEY,
    vault_id TEXT NOT NULL,
    vault_type TEXT NOT NULL,
    performance_fee_rate DOUBLE PRECISION NOT NULL,
    tvl DOUBLE PRECISION NOT NULL,
    apr_30d DOUBLE PRECISION NOT NULL,
    apy_30d DOUBLE PRECISION NOT NULL,
    vault_lifetime_net_pnl DOUBLE PRECISION NOT NULL,
    lp_counts BIGINT NOT NULL,
    total_main_shares DOUBLE PRECISION NOT NULL,
    est_main_share_price DOUBLE PRECISION NOT NULL,
    lock_duration BIGINT NOT NULL,
    broker_id TEXT NOT NULL,
    min_withdrawal_amount DOUBLE PRECISION NOT NULL,
    supported_chains TEXT[] NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

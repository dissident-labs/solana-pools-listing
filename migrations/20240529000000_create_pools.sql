CREATE TABLE IF NOT EXISTS pools ( 
    pool_address TEXT PRIMARY KEY, 
    pool_name TEXT, 
    token_mints TEXT[], 
    token_amounts DOUBLE PRECISION[], 
    token_usd_amounts DOUBLE PRECISION[], 
    chain_id TEXT NOT NULL, 
    
    farming_apy DOUBLE PRECISION, 
    trade_apy DOUBLE PRECISION, 
    virtual_price_apy DOUBLE PRECISION, 
    daily_base_apy DOUBLE PRECISION, 
    weekly_base_apy DOUBLE PRECISION, 
    
    source TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(), 
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW() 
);



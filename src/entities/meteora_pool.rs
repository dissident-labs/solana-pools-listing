use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, FromRow, Serialize, Clone, PartialEq)]
pub struct MeteoraPool {
    pub pool_address: String,
    pub pool_name: String,
    pub token_a_mint: String,
    pub token_b_mint: String,
    pub token_a_symbol: String,
    pub token_b_symbol: String,
    pub liquidity: f64,
    pub tvl: f64,
    pub apr: f64,
    pub volume24h: f64,
    pub fee24h: f64,
    pub pool_price: f64,
    pub created_at_slot_timestamp: i64,
    pub updated_at: i64,
}

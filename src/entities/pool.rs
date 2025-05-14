use serde::Deserialize;
use sqlx::FromRow;

#[derive(Debug, Deserialize, FromRow)]
pub struct Pool {
    pub pool_address: String,
    pub pool_name: Option<String>,
    pub pool_token_mints: Option<Vec<String>>,
    pub pool_token_amounts: Option<Vec<String>>,
    pub pool_token_usd_amounts: Option<Vec<String>>,

    pub farming_apy: Option<String>,
    pub trade_apy: Option<String>,
    pub virtual_price_apy: Option<String>,
    pub daily_base_apy: Option<String>,
    pub weekly_base_apy: Option<String>,
    pub chain_id: String,

    pub source: Option<String>,
}

// Add this implementation block
impl Pool {
    pub fn parse_f64(value: &Option<String>) -> Option<f64> {
        value.as_ref().and_then(|s| s.parse().ok())
    }

    pub fn parse_f64_vec(values: &Option<Vec<String>>) -> Vec<f64> {
        values
            .as_ref()
            .map(|vec| vec.iter().filter_map(|s| s.parse().ok()).collect())
            .unwrap_or_default()
    }
}

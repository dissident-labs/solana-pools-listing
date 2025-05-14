use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, FromRow, Serialize)]
pub struct Vault {
    pub vault_address: String,
    pub vault_id: String,
    pub vault_type: String,
    pub performance_fee_rate: f64,
    pub tvl: f64,
    pub apr_30d: f64,
    pub apy_30d: f64,
    pub vault_lifetime_net_pnl: f64,
    pub lp_counts: i64,
    pub total_main_shares: f64,
    pub est_main_share_price: f64,
    pub lock_duration: i64,
    pub broker_id: String,
    pub min_withdrawal_amount: f64,
    pub supported_chains: Vec<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

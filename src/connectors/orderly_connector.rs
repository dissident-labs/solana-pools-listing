use crate::entities::vault::Vault;
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct OrderlyVault {
    vault_address: String,
    vault_id: String,
    vault_type: String,
    performance_fee_rate: f64,
    supported_chains: Vec<ChainInfo>,
    tvl: f64,
    #[serde(rename = "30d_apr")]
    apr_30d: f64,
    #[serde(rename = "30d_apy")]
    apy_30d: f64,
    vault_lifetime_net_pnl: f64,
    lp_counts: u64,
    total_main_shares: f64,
    est_main_share_price: f64,
    lock_duration: u64,
    broker_id: String,
    min_withdrawal_amount: f64,
}

#[derive(Debug, Deserialize)]
struct ChainInfo {
    chain_id: String,
    chain_name: String,
}

#[derive(Debug, Deserialize)]
struct OrderlyResponse {
    success: bool,
    timestamp: u64,
    data: OrderlyData,
}

#[derive(Debug, Deserialize)]
struct OrderlyData {
    rows: Vec<OrderlyVault>,
}

pub async fn fetch_orderly_vaults() -> Result<Vec<Vault>, Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client
        .get("https://api-sv.orderly.org/v1/public/strategy_vault/vault/info")
        .send()
        .await?
        .json::<OrderlyResponse>()
        .await?;

    let vaults = response
        .data
        .rows
        .into_iter()
        .map(|vault| Vault {
            vault_address: vault.vault_address,
            vault_id: vault.vault_id,
            vault_type: vault.vault_type,
            performance_fee_rate: vault.performance_fee_rate,
            tvl: vault.tvl,
            apr_30d: vault.apr_30d,
            apy_30d: vault.apy_30d,
            vault_lifetime_net_pnl: vault.vault_lifetime_net_pnl,
            lp_counts: vault.lp_counts as i64,
            total_main_shares: vault.total_main_shares,
            est_main_share_price: vault.est_main_share_price,
            lock_duration: vault.lock_duration as i64,
            broker_id: vault.broker_id,
            min_withdrawal_amount: vault.min_withdrawal_amount,
            supported_chains: vault
                .supported_chains
                .into_iter()
                .map(|c| c.chain_id)
                .collect(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        })
        .collect();

    Ok(vaults)
}

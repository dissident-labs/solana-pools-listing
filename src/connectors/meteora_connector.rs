use crate::entities::meteora_pool::MeteoraPool;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Deserialize)]
struct MeteoraResponse {
    data: Vec<MeteoraPool>,
    total: u32,
    pages: u32,
    current_page: u32,
}

pub async fn fetch_meteora_pools() -> Result<Vec<MeteoraPool>, Box<dyn Error>> {
    let client = Client::new();
    let mut all_pools = Vec::new();
    let mut offset = 0;
    const LIMIT: usize = 100;

    let url = format!(
        "https://www.meteora.ag/cp-amm/pools?limit={}&offset={}&order_by=tvl&order=desc",
        LIMIT, offset
    );

    let response = client
        .get(&url)
        .send()
        .await?
        .json::<MeteoraResponse>()
        .await?;

    let received = response.data.len();
    all_pools.extend(response.data);
    offset += LIMIT;

    Ok(all_pools)
}

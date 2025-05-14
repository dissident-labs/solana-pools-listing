use crate::entities::meteora_pool::MeteoraPool;
use sqlx::{postgres::PgQueryResult, PgPool};

pub enum ApyType {
    Apr,
}

impl ApyType {
    fn as_column(&self) -> &'static str {
        match self {
            ApyType::Apr => "apr",
        }
    }
}

pub async fn query_meteora_pools_by_apy(
    pool: &PgPool,
    min_apy: f64,
    token_filter: Option<String>,
) -> Result<Vec<MeteoraPool>, sqlx::Error> {
    let column = ApyType::Apr.as_column();
    let mut query = format!("SELECT * FROM meteora_pools WHERE {} >= $1", column);
    let mut params: Vec<String> = vec![];
    let mut index = 2;

    if let Some(token) = &token_filter {
        query.push_str(&format!(
            " AND (token_a_symbol = ${} OR token_b_symbol = ${})",
            index, index
        ));
        params.push(token.clone());
    }

    let row_query = sqlx::query_as::<_, MeteoraPool>(&query);

    let mut query_builder = row_query.bind(min_apy);
    for p in params {
        query_builder = query_builder.bind(p);
    }

    query_builder.fetch_all(pool).await
}

pub async fn add_meteora_pool(
    pool: &PgPool,
    mp: &MeteoraPool,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO meteora_pools (
            pool_address, pool_name, token_a_mint, token_b_mint,
            token_a_symbol, token_b_symbol, liquidity, tvl, apr,
            volume24h, fee24h, pool_price,
            created_at_slot_timestamp, updated_at
        )
        VALUES (
            $1, $2, $3, $4,
            $5, $6, $7, $8, $9,
            $10, $11, $12,
            $13, $14
        )
        ON CONFLICT (pool_address) DO UPDATE SET
            liquidity = EXCLUDED.liquidity,
            tvl = EXCLUDED.tvl,
            apr = EXCLUDED.apr,
            volume24h = EXCLUDED.volume24h,
            fee24h = EXCLUDED.fee24h,
            pool_price = EXCLUDED.pool_price,
            updated_at = EXCLUDED.updated_at
        "#,
        mp.pool_address,
        mp.pool_name,
        mp.token_a_mint,
        mp.token_b_mint,
        mp.token_a_symbol,
        mp.token_b_symbol,
        mp.liquidity,
        mp.tvl,
        mp.apr,
        mp.volume24h,
        mp.fee24h,
        mp.pool_price,
        mp.created_at_slot_timestamp,
        mp.updated_at
    )
    .execute(pool)
    .await
}

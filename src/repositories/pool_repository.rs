use crate::entities::pool::Pool;
use sqlx::PgPool;

pub enum ApyType {
    Farming,
    Trade,
    VirtualPrice,
    DailyBase,
    WeeklyBase,
}

impl ApyType {
    fn as_column(&self) -> &'static str {
        match self {
            ApyType::Farming => "farming_apy",
            ApyType::Trade => "trade_apy",
            ApyType::VirtualPrice => "virtual_price_apy",
            ApyType::DailyBase => "daily_base_apy",
            ApyType::WeeklyBase => "weekly_base_apy",
        }
    }
}

pub async fn query_pools_by_apy(
    pool: &PgPool,
    apy_type: ApyType,
    min_apy: f64,
    token_filter: Option<String>,
    token_pair_filter: Option<(String, String)>,
) -> Result<Vec<Pool>, sqlx::Error> {
    let column = apy_type.as_column();

    let mut query = format!("SELECT * FROM pools WHERE {} >= $1", column);
    let mut params: Vec<String> = vec![];
    let mut index = 2;

    if let Some(token) = &token_filter {
        query.push_str(&format!(" AND ${} = ANY(token_mints)", index));
        params.push(token.clone());
        index += 1;
    }

    if let Some((token_a, token_b)) = &token_pair_filter {
        query.push_str(&format!(
            " AND ${} = ANY(token_mints) AND ${} = ANY(token_mints)",
            index,
            index + 1
        ));
        params.push(token_a.clone());
        params.push(token_b.clone());
    }

    let row_query = sqlx::query_as::<_, Pool>(&query);

    // Dynamically bind the parameters
    let mut query = row_query.bind(min_apy);
    for p in params {
        query = query.bind(p);
    }

    let result = query.fetch_all(pool).await?;
    Ok(result)
}

pub async fn add_pool(pool: &PgPool, p: &Pool) -> Result<(), sqlx::Error> {
    let token_amounts = Pool::parse_f64_vec(&p.pool_token_amounts);
    let usd_amounts = Pool::parse_f64_vec(&p.pool_token_usd_amounts);

    sqlx::query!(
        r#"
        INSERT INTO pools (
            pool_address, pool_name, token_mints, token_amounts, token_usd_amounts, chain_id,
            farming_apy, trade_apy, virtual_price_apy, daily_base_apy, weekly_base_apy,
            created_at, updated_at
        )
        VALUES (
            $1, $2, $3, $4, $5, $6,
            $7, $8, $9, $10, $11,
            NOW(), NOW()
        )
        ON CONFLICT (pool_address) DO UPDATE SET
            token_amounts = EXCLUDED.token_amounts,
            token_usd_amounts = EXCLUDED.token_usd_amounts,
            farming_apy = EXCLUDED.farming_apy,
            trade_apy = EXCLUDED.trade_apy,
            virtual_price_apy = EXCLUDED.virtual_price_apy,
            daily_base_apy = EXCLUDED.daily_base_apy,
            weekly_base_apy = EXCLUDED.weekly_base_apy,
            updated_at = NOW()
        "#,
        p.pool_address,
        p.pool_name,
        &p.pool_token_mints.clone().unwrap_or_default(),
        &token_amounts,
        &usd_amounts,
        "solana",
        Pool::parse_f64(&p.farming_apy),
        Pool::parse_f64(&p.trade_apy),
        Pool::parse_f64(&p.virtual_price_apy),
        Pool::parse_f64(&p.daily_base_apy),
        Pool::parse_f64(&p.weekly_base_apy)
    )
    .execute(pool)
    .await?;

    Ok(())
}

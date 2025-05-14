use crate::entities::vault::Vault;
use sqlx::{postgres::PgQueryResult, PgPool};

pub enum ApyType {
    ThirtyDayApr,
    ThirtyDayApy,
}

impl ApyType {
    fn as_column(&self) -> &'static str {
        match self {
            ApyType::ThirtyDayApr => "apr_30d",
            ApyType::ThirtyDayApy => "apy_30d",
        }
    }
}

pub async fn query_vaults_by_apy(
    pool: &PgPool,
    apy_type: ApyType,
    min_apy: f64,
    chain_filter: Option<String>,
) -> Result<Vec<Vault>, sqlx::Error> {
    let column = apy_type.as_column();
    let mut query = format!("SELECT * FROM vaults WHERE {} >= $1", column);
    let mut params: Vec<String> = vec![];
    let mut index = 2;

    if let Some(chain) = &chain_filter {
        query.push_str(&format!(" AND ${} = ANY(supported_chains)", index));
        params.push(chain.clone());
    }

    let row_query = sqlx::query_as::<_, Vault>(&query);

    let mut query_builder = row_query.bind(min_apy);
    for p in params {
        query_builder = query_builder.bind(p);
    }

    query_builder.fetch_all(pool).await
}

pub async fn add_vault(pool: &PgPool, vault: &Vault) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO vaults (
            vault_address, vault_id, vault_type, performance_fee_rate, tvl,
            apr_30d, apy_30d, vault_lifetime_net_pnl, lp_counts,
            total_main_shares, est_main_share_price, lock_duration,
            broker_id, min_withdrawal_amount, supported_chains,
            created_at, updated_at
        )
        VALUES (
            $1, $2, $3, $4, $5,
            $6, $7, $8, $9,
            $10, $11, $12,
            $13, $14, $15,
            NOW(), NOW()
        )
        ON CONFLICT (vault_address) DO UPDATE SET
            tvl = EXCLUDED.tvl,
            apr_30d = EXCLUDED.apr_30d,
            apy_30d = EXCLUDED.apy_30d,
            vault_lifetime_net_pnl = EXCLUDED.vault_lifetime_net_pnl,
            lp_counts = EXCLUDED.lp_counts,
            total_main_shares = EXCLUDED.total_main_shares,
            est_main_share_price = EXCLUDED.est_main_share_price,
            updated_at = NOW()
        "#,
        vault.vault_address,
        vault.vault_id,
        vault.vault_type,
        vault.performance_fee_rate,
        vault.tvl,
        vault.apr_30d,
        vault.apy_30d,
        vault.vault_lifetime_net_pnl,
        vault.lp_counts,
        vault.total_main_shares,
        vault.est_main_share_price,
        vault.lock_duration,
        vault.broker_id,
        vault.min_withdrawal_amount,
        &vault.supported_chains
    )
    .execute(pool)
    .await
}

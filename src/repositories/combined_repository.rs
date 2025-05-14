use crate::entities::financial_product::FinancialProduct;

pub async fn query_all_by_apr(
    pool: &sqlx::PgPool,
) -> Result<Vec<FinancialProduct>, Box<dyn std::error::Error>> {
    let meteora_pools = crate::repositories::meteora_pool_repository::query_meteora_pools_by_apy(pool, 0.0, None)
        .await?
        .into_iter()
        .map(FinancialProduct::MeteoraPool);

    let vaults = crate::repositories::vault_repository::query_vaults_by_apy(
        pool,
        crate::repositories::vault_repository::ApyType::ThirtyDayApy,
        0.0,
        None,
    )
    .await?
    .into_iter()
    .map(FinancialProduct::Vault);

    let mut combined: Vec<FinancialProduct> = meteora_pools.chain(vaults).collect();
    combined.sort_by(|a, b| b.apr().partial_cmp(&a.apr()).unwrap_or(std::cmp::Ordering::Equal));
    
    Ok(combined)
}
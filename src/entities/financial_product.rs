#[derive(Debug)]
pub enum FinancialProduct {
    MeteoraPool(crate::entities::meteora_pool::MeteoraPool),
    Vault(crate::entities::vault::Vault),
}

impl FinancialProduct {
    pub fn apr(&self) -> f64 {
        match self {
            FinancialProduct::MeteoraPool(p) => p.apr,
            FinancialProduct::Vault(v) => v.apy_30d,
        }
    }

    pub fn product_type(&self) -> &'static str {
        match self {
            FinancialProduct::MeteoraPool(_) => "meteora_pool",
            FinancialProduct::Vault(_) => "orderly-vault",
        }
    }
}

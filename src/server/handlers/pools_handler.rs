use crate::database::DatabaseTrait;
use crate::server::error::api_error::ApiError;
use crate::server::state::app_state::AppState;
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use tracing::debug;
use validator::Validate;

#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct FinancialProductResponse {
    pub type_: String,
    pub data: serde_json::Value,
}

#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct PoolsResponse {
    pub data: Vec<FinancialProductResponse>,
}

pub async fn get_pools_handler(
    State(state): State<AppState>,
) -> Result<Json<PoolsResponse>, ApiError> {
    let db_conn = state.db_conn;
    let pool = db_conn.get_pool();
    let products = crate::repositories::combined_repository::query_all_by_apr(pool)
        .await
        .unwrap();

    let response = PoolsResponse {
        data: products
            .into_iter()
            .map(|p| match p {
                crate::entities::financial_product::FinancialProduct::MeteoraPool(p) => {
                    FinancialProductResponse {
                        type_: "meteora_pool".into(),
                        data: serde_json::to_value(p).unwrap(),
                    }
                }
                crate::entities::financial_product::FinancialProduct::Vault(v) => {
                    FinancialProductResponse {
                        type_: "vault".into(),
                        data: serde_json::to_value(v).unwrap(),
                    }
                }
            })
            .collect::<Vec<_>>(),
    };

    Ok(Json(response))
}

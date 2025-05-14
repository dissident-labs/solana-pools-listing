use axum::{extract::State, response::Json, routing::get, Router};
use serde::Deserialize;
use sqlx::PgPool;
use std::net::SocketAddr;
use std::sync::Arc;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ServerSettings {
    pub host: String,
    pub port: i32,
    pub cors_origin: Option<String>,
    pub allow_origins: Vec<String>,
}

#[derive(Clone)]
pub struct StateParams {
    pub pool: Arc<PgPool>,
}

pub async fn run_server(pool: PgPool, port: u32) -> Result<(), Box<dyn std::error::Error>> {
    let state = StateParams {
        pool: Arc::new(pool),
    };

    let app = Router::new()
        .route("/pools", get(get_pools))
        .with_state(state.clone());

    let addr = SocketAddr::from(([0, 0, 0, 0], port as u16));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn get_pools(
    State(pool): State<PgPool>,
) -> Result<Json<serde_json::Value>, Box<dyn std::error::Error>> {
    let products = crate::repositories::combined_repository::query_all_by_apr(&pool).await?;

    let response = serde_json::json!({
        "data": products.into_iter().map(|p| match p {
            crate::entities::financial_product::FinancialProduct::MeteoraPool(p) => {
                serde_json::json!({
                    "type": "meteora_pool",
                    "data": p
                })
            }
            crate::entities::financial_product::FinancialProduct::Vault(v) => {
                serde_json::json!({
                    "type": "vault",
                    "data": v
                })
            }
        }).collect::<Vec<_>>()
    });

    Ok(Json(response))
}

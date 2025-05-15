use crate::server::{handlers::pools_handler, state::app_state::AppState};
use axum::{routing::get, Router};

pub fn routes() -> Router<AppState> {
    Router::new().route("/pools", get(pools_handler::get_pools_handler))
}

use crate::server::{routes::pools, state::app_state::AppState};

use axum::{extract::MatchedPath, http::Request, middleware, routing::get, Router};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::{info, info_span, Span};

use crate::database::Database;
use std::sync::Arc;

pub struct AppRouter {
    pub router: Router,
}

impl AppRouter {
    pub fn new(app_state: &AppState) -> Self {
        let router_no_session = pools::routes();

        let merged_router = {
            let state = app_state.clone();

            router_no_session
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(|request: &Request<_>| {
                            let matched_path = request
                                .extensions()
                                .get::<MatchedPath>()
                                .map(MatchedPath::as_str);

                            info_span!(
                                "http_request",
                                method = ?request.method(),
                                matched_path,
                                uri = %request.uri(),
                                status = tracing::field::Empty,
                                latency_on_response = tracing::field::Empty,
                                latency_on_body_chunk = tracing::field::Empty,
                                stream_duration_on_eos = tracing::field::Empty,
                                latency_on_failure = tracing::field::Empty,
                            )
                        })
                        .on_request(|_request: &Request<_>, _span: &Span| {
                            tracing::info!("on_request");
                        }),
                )
                .with_state(state)
        };

        let router = Router::new().nest("/api/v1", merged_router);

        Self { router }
    }
}

impl Default for AppRouter {
    fn default() -> Self {
        Self::new(&AppState::default())
    }
}

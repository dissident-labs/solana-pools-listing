use crate::database::Database;
use std::collections::HashMap;
use std::sync::Arc;

use http::header::CONTENT_TYPE;
use http::{HeaderValue, Method};
use serde::Deserialize;
use tokio::sync::{broadcast, Mutex};
use tower_http::cors::{AllowOrigin, CorsLayer};

use crate::server::listener::AppListener;
use crate::server::router::AppRouter;
use crate::server::state::app_state::AppState;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ServerSettings {
    pub host: String,
    pub port: i32,
    pub cors_origin: Option<String>,
    pub allow_origins: Vec<String>,
}

#[derive(Clone)]
pub struct StateParams {
    pub database: Arc<Database>,
}

pub struct Server {
    app_router: AppRouter,
    app_listener: AppListener,
    port: u16,
    allow_origins: Vec<String>,
}

impl Server {
    pub async fn build(server_settings: ServerSettings, state_params: &StateParams) -> Self {
        let handler_state = AppState {
            db_conn: state_params.database.clone(),
        };
        let app_router = AppRouter::new(&handler_state);
        let app_listener = AppListener::new(server_settings.host, server_settings.port)
            .await
            .expect("failed to bind port");

        let port = app_listener.listener.local_addr().unwrap().port();
        let allow_origins = server_settings.allow_origins.clone();

        Self {
            app_router,
            app_listener,
            port,
            allow_origins,
        }
    }

    pub async fn run(self) {
        tracing::info!("Starting Server");

        let cors = if self.allow_origins.contains(&"*".to_string()) {
            CorsLayer::new().allow_origin(AllowOrigin::any())
        } else {
            let origins = self
                .allow_origins
                .iter()
                .filter_map(|o| o.parse::<HeaderValue>().ok())
                .collect::<Vec<_>>();
            CorsLayer::new().allow_origin(AllowOrigin::list(origins))
        }
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .expose_headers([CONTENT_TYPE])
        .allow_headers([CONTENT_TYPE]);

        // let service = self.app_router.router.layer(cors).into_make_service();

        axum::serve(
            self.app_listener.listener,
            self.app_router.router.layer(cors),
        )
        .await
        .unwrap();
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}

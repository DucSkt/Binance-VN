use crate::api::routes::ApiRouter;
use crate::api::state::AppState;
use crate::api::user;
use crate::config::settings::Settings;
use crate::utils::error::ErrorResponse;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{routing::get, Json, Router};
use colored::{Color, ColoredString, Colorize};
use fxhash::FxHasher;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::hash::{Hash, Hasher};
use std::net::TcpListener;
use std::sync::Arc;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tower_http::timeout::TimeoutLayer;
use tower_http::trace;
use tower_http::trace::TraceLayer;
use tracing::log::__private_api::log;
use tracing::{info, Level};
use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter};

mod api;
mod macros;
mod utils;

mod config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new("info"))
        .with_target(false) // Hide log target (module name)
        .with_level(true) // Show log levels
        .init();
    info!("Starting server...");

    let settings = Settings::new()?;
    clg!("settings", settings);
    let app_state = AppState::new(settings.clone()).await;
    // todo db_pool
    let app = Router::new()
        .nest("/api/v1", ApiRouter::new(app_state.clone()).into())
        .fallback(handler_404)
        .layer((
            TraceLayer::new_for_http(),
            TimeoutLayer::new(Duration::from_secs(10)),
        ))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::TRACE)),
        )
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_private_network(true)
                .allow_headers(Any),
        )
        .with_state(app_state)
        .route("/healthz", get(healthz));

    let addr = format!("{}:{}", "localhost", "3000");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("Listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn index() -> &'static str {
    info!("Handling request at /");
    "Hello, world!"
}

async fn healthz() -> &'static str {
    "OK"
}

async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(ErrorResponse::new("nothing here".to_string())),
    )
}

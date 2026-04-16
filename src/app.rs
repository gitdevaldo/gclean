use std::time::Duration;

use axum::error_handling::HandleErrorLayer;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde_json::json;
use tower::timeout::TimeoutLayer;
use tower::BoxError;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};
use tower_http::trace::TraceLayer;

use crate::config::AppConfig;
use crate::handlers::{health, validate_email};
use crate::state::AppState;

pub fn create_router(state: AppState, config: &AppConfig) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/v1/validate-email", post(validate_email))
        .with_state(state)
        .layer(
            ServiceBuilder::new()
                .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
                .layer(PropagateRequestIdLayer::x_request_id())
                .layer(HandleErrorLayer::new(handle_middleware_error))
                .layer(TimeoutLayer::new(Duration::from_secs(
                    config.request_timeout_seconds,
                )))
                .layer(CompressionLayer::new())
                .layer(TraceLayer::new_for_http()),
        )
}

async fn handle_middleware_error(error: BoxError) -> impl IntoResponse {
    if error.is::<tower::timeout::error::Elapsed>() {
        return (
            StatusCode::REQUEST_TIMEOUT,
            Json(json!({ "error": "request timed out" })),
        );
    }

    tracing::error!(%error, "request middleware failed");
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({ "error": "internal server error" })),
    )
}

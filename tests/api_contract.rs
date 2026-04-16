use axum::body::{to_bytes, Body};
use axum::http::{Request, StatusCode};
use serde_json::Value;
use tower::util::ServiceExt;

use email_validator_api::app::create_router;
use email_validator_api::config::AppConfig;
use email_validator_api::models::EmailValidationResponse;
use email_validator_api::service::EmailValidationService;
use email_validator_api::state::AppState;

#[tokio::test]
async fn validate_email_returns_email_and_bool() {
    let app = create_router(AppState::new(EmailValidationService), &AppConfig::default());
    let request = Request::builder()
        .method("POST")
        .uri("/v1/validate-email")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"email":"user@example.com"}"#))
        .expect("request should be valid");

    let response = app
        .oneshot(request)
        .await
        .expect("response should be available");
    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("body should be readable");
    let payload: EmailValidationResponse =
        serde_json::from_slice(&body).expect("payload should match response schema");

    assert_eq!(payload.email, "user@example.com");
}

#[tokio::test]
async fn validate_email_rejects_empty_email() {
    let app = create_router(AppState::new(EmailValidationService), &AppConfig::default());
    let request = Request::builder()
        .method("POST")
        .uri("/v1/validate-email")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"email":"   "}"#))
        .expect("request should be valid");

    let response = app
        .oneshot(request)
        .await
        .expect("response should be available");
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("body should be readable");
    let payload: Value = serde_json::from_slice(&body).expect("error payload should be json");
    assert_eq!(payload["error"], "email must not be empty");
}

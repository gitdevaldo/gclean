use axum::extract::State;
use axum::Json;

use crate::error::{ApiError, ApiResult};
use crate::models::{EmailValidationRequest, EmailValidationResponse, HealthResponse};
use crate::state::AppState;

pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

pub async fn validate_email(
    State(state): State<AppState>,
    Json(payload): Json<EmailValidationRequest>,
) -> ApiResult<Json<EmailValidationResponse>> {
    let email = payload.email.trim().to_owned();
    if email.is_empty() {
        return Err(ApiError::BadRequest("email must not be empty".to_string()));
    }

    let valid = state.validator.validate(&email);
    Ok(Json(EmailValidationResponse { email, valid }))
}

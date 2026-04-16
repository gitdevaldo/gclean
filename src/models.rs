use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EmailValidationRequest {
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct EmailValidationResponse {
    pub email: String,
    pub valid: bool,
}

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
}

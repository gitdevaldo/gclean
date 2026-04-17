use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ActorInput {
    #[serde(default)]
    pub emails: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct ValidationRequest {
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct ValidationApiEnvelope {
    pub results: Vec<ValidationApiResult>,
}

#[derive(Debug, Deserialize)]
pub struct ValidationApiResult {
    pub email: String,
    pub status: String,
    pub valid: bool,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ValidationApiResponse {
    Envelope(ValidationApiEnvelope),
    LegacySuccess { email: String, valid: bool },
    Error { error: String },
}

#[derive(Debug, Serialize)]
pub struct DatasetResult {
    pub email: String,
    pub status: String,
}

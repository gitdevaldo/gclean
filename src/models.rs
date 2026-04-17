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
    #[serde(default)]
    pub flags: ValidationFlags,
}

#[derive(Debug, Deserialize, Default)]
pub struct ValidationFlags {
    #[serde(default)]
    pub disposable: Option<bool>,
    #[serde(default)]
    pub role_based: Option<bool>,
    #[serde(default)]
    pub catch_all: Option<bool>,
    #[serde(default)]
    pub spam_trap: Option<bool>,
    #[serde(default)]
    pub pwned_signal: Option<bool>,
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
    pub valid: String,
    pub flags_disposable: Option<String>,
    pub flags_role_based: Option<String>,
    pub flags_catch_all: Option<String>,
    pub flags_spam_trap: Option<String>,
    pub flags_pwned_signal: Option<String>,
}

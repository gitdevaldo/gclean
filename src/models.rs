use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ActorInput {
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub emails: Vec<String>,
    #[serde(default, rename = "apiToken")]
    pub api_token: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ValidationRequest {
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum ValidationResponse {
    Success { email: String, valid: bool },
    Error { error: String },
}

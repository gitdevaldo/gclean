use std::time::Duration;

use reqwest::Client;

use crate::config::{REQUEST_TIMEOUT_SECONDS, VALIDATION_API_URL};
use crate::error::ActorError;
use crate::models::{DatasetResult, ValidationApiResponse, ValidationRequest};

#[derive(Clone)]
pub struct EmailValidationService {
    client: Client,
}

impl EmailValidationService {
    pub fn new() -> Result<Self, ActorError> {
        let client = Client::builder()
            .timeout(Duration::from_secs(REQUEST_TIMEOUT_SECONDS))
            .build()?;
        Ok(Self { client })
    }

    pub async fn validate(
        &self,
        email: String,
        api_token: &str,
    ) -> Result<DatasetResult, ActorError> {
        let response = self
            .client
            .post(VALIDATION_API_URL)
            .bearer_auth(api_token)
            .json(&ValidationRequest { email })
            .send()
            .await?;

        let payload = response.json::<ValidationApiResponse>().await?;
        map_dataset_result(payload)
    }
}

fn map_dataset_result(payload: ValidationApiResponse) -> Result<DatasetResult, ActorError> {
    match payload {
        ValidationApiResponse::Envelope(mut envelope) => {
            let Some(first) = envelope.results.drain(..).next() else {
                return Ok(DatasetResult {
                    email: String::new(),
                    status: "error".to_string(),
                });
            };

            Ok(DatasetResult {
                email: first.email,
                status: normalize_status(&first.status, first.valid),
            })
        }
        ValidationApiResponse::LegacySuccess { email, valid } => Ok(DatasetResult {
            email,
            status: valid.to_string(),
        }),
        ValidationApiResponse::Error { .. } => Ok(DatasetResult {
            email: String::new(),
            status: "error".to_string(),
        }),
    }
}

fn normalize_status(status: &str, valid: bool) -> String {
    match status.trim().to_ascii_lowercase().as_str() {
        "true" => "true".to_string(),
        "false" => "false".to_string(),
        "error" => "error".to_string(),
        _ => valid.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::{map_dataset_result, normalize_status};
    use crate::models::{ValidationApiEnvelope, ValidationApiResponse, ValidationApiResult};

    #[test]
    fn normalize_status_accepts_true_false_error() {
        assert_eq!(normalize_status("true", false), "true".to_string());
        assert_eq!(normalize_status("false", true), "false".to_string());
        assert_eq!(normalize_status("error", true), "error".to_string());
    }

    #[test]
    fn normalize_status_falls_back_to_valid_boolean() {
        assert_eq!(normalize_status("unknown", true), "true".to_string());
        assert_eq!(normalize_status("unknown", false), "false".to_string());
    }

    #[test]
    fn map_dataset_result_reads_first_envelope_result() {
        let payload = ValidationApiResponse::Envelope(ValidationApiEnvelope {
            results: vec![ValidationApiResult {
                email: "user@example.com".to_string(),
                status: "false".to_string(),
                valid: false,
            }],
        });

        let mapped = map_dataset_result(payload).expect("mapping should succeed");
        assert_eq!(mapped.email, "user@example.com".to_string());
        assert_eq!(mapped.status, "false".to_string());
    }
}

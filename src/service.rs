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
                return Ok(empty_error_result());
            };

            Ok(DatasetResult {
                email: first.email,
                status: first.status,
                valid: first.valid.to_string(),
                flags_disposable: bool_to_text(first.flags.disposable),
                flags_role_based: bool_to_text(first.flags.role_based),
                flags_catch_all: bool_to_text(first.flags.catch_all),
                flags_spam_trap: bool_to_text(first.flags.spam_trap),
                flags_pwned_signal: bool_to_text(first.flags.pwned_signal),
            })
        }
        ValidationApiResponse::LegacySuccess { email, valid } => Ok(DatasetResult {
            email,
            status: valid.to_string(),
            valid: valid.to_string(),
            flags_disposable: "null".to_string(),
            flags_role_based: "null".to_string(),
            flags_catch_all: "null".to_string(),
            flags_spam_trap: "null".to_string(),
            flags_pwned_signal: "null".to_string(),
        }),
        ValidationApiResponse::Error { .. } => Ok(empty_error_result()),
    }
}

fn bool_to_text(value: Option<bool>) -> String {
    match value {
        Some(v) => v.to_string(),
        None => "null".to_string(),
    }
}

fn empty_error_result() -> DatasetResult {
    DatasetResult {
        email: String::new(),
        status: "error".to_string(),
        valid: "error".to_string(),
        flags_disposable: "null".to_string(),
        flags_role_based: "null".to_string(),
        flags_catch_all: "null".to_string(),
        flags_spam_trap: "null".to_string(),
        flags_pwned_signal: "null".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::map_dataset_result;
    use crate::models::{ValidationApiEnvelope, ValidationApiResponse, ValidationApiResult};

    #[test]
    fn map_dataset_result_reads_first_envelope_result() {
        let payload = ValidationApiResponse::Envelope(ValidationApiEnvelope {
            results: vec![ValidationApiResult {
                email: "user@example.com".to_string(),
                status: "false".to_string(),
                valid: false,
                flags: Default::default(),
            }],
        });

        let mapped = map_dataset_result(payload).expect("mapping should succeed");
        assert_eq!(mapped.email, "user@example.com".to_string());
        assert_eq!(mapped.status, "false".to_string());
        assert_eq!(mapped.valid, "false".to_string());
        assert_eq!(mapped.flags_disposable, "null".to_string());
    }

    #[test]
    fn map_dataset_result_preserves_status_value_exactly() {
        let payload = ValidationApiResponse::Envelope(ValidationApiEnvelope {
            results: vec![ValidationApiResult {
                email: "user@example.com".to_string(),
                status: "invalid".to_string(),
                valid: false,
                flags: Default::default(),
            }],
        });

        let mapped = map_dataset_result(payload).expect("mapping should succeed");
        assert_eq!(mapped.status, "invalid".to_string());
    }
}

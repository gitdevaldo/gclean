use std::time::Duration;

use reqwest::Client;

use crate::config::{REQUEST_TIMEOUT_SECONDS, VALIDATION_API_URL};
use crate::error::ActorError;
use crate::models::{DatasetResult, ValidationApiResponse, ValidationCheck, ValidationRequest};

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

            let (check_format_status, check_format_message, check_format_detail) =
                check_triplet(first.checks.format.as_ref());
            let (
                check_domain_existence_status,
                check_domain_existence_message,
                check_domain_existence_detail,
            ) = check_triplet(first.checks.domain_existence.as_ref());
            let (check_mx_records_status, check_mx_records_message, check_mx_records_detail) =
                check_triplet(first.checks.mx_records.as_ref());
            let (
                check_disposable_temp_mail_status,
                check_disposable_temp_mail_message,
                check_disposable_temp_mail_detail,
            ) = check_triplet(first.checks.disposable_temp_mail.as_ref());
            let (check_role_based_status, check_role_based_message, check_role_based_detail) =
                check_triplet(first.checks.role_based.as_ref());
            let (
                check_smtp_verification_status,
                check_smtp_verification_message,
                check_smtp_verification_detail,
            ) = check_triplet(first.checks.smtp_verification.as_ref());
            let (check_pwned_check_status, check_pwned_check_message, check_pwned_check_detail) =
                check_triplet(first.checks.pwned_check.as_ref());
            let (
                check_catch_all_detection_status,
                check_catch_all_detection_message,
                check_catch_all_detection_detail,
            ) = check_triplet(first.checks.catch_all_detection.as_ref());
            let (
                check_spam_trap_detection_status,
                check_spam_trap_detection_message,
                check_spam_trap_detection_detail,
            ) = check_triplet(first.checks.spam_trap_detection.as_ref());

            Ok(DatasetResult {
                email: first.email,
                status: normalize_status(&first.status, first.valid),
                valid: first.valid.to_string(),
                flags_disposable: bool_to_text(first.flags.disposable),
                flags_role_based: bool_to_text(first.flags.role_based),
                flags_catch_all: bool_to_text(first.flags.catch_all),
                flags_spam_trap: bool_to_text(first.flags.spam_trap),
                flags_pwned_signal: bool_to_text(first.flags.pwned_signal),
                check_format_status,
                check_format_message,
                check_format_detail,
                check_domain_existence_status,
                check_domain_existence_message,
                check_domain_existence_detail,
                check_mx_records_status,
                check_mx_records_message,
                check_mx_records_detail,
                check_disposable_temp_mail_status,
                check_disposable_temp_mail_message,
                check_disposable_temp_mail_detail,
                check_role_based_status,
                check_role_based_message,
                check_role_based_detail,
                check_smtp_verification_status,
                check_smtp_verification_message,
                check_smtp_verification_detail,
                check_pwned_check_status,
                check_pwned_check_message,
                check_pwned_check_detail,
                check_catch_all_detection_status,
                check_catch_all_detection_message,
                check_catch_all_detection_detail,
                check_spam_trap_detection_status,
                check_spam_trap_detection_message,
                check_spam_trap_detection_detail,
            })
        }
        ValidationApiResponse::LegacySuccess { email, valid } => Ok(DatasetResult {
            email,
            status: valid.to_string(),
            valid: valid.to_string(),
            flags_disposable: None,
            flags_role_based: None,
            flags_catch_all: None,
            flags_spam_trap: None,
            flags_pwned_signal: None,
            check_format_status: None,
            check_format_message: None,
            check_format_detail: None,
            check_domain_existence_status: None,
            check_domain_existence_message: None,
            check_domain_existence_detail: None,
            check_mx_records_status: None,
            check_mx_records_message: None,
            check_mx_records_detail: None,
            check_disposable_temp_mail_status: None,
            check_disposable_temp_mail_message: None,
            check_disposable_temp_mail_detail: None,
            check_role_based_status: None,
            check_role_based_message: None,
            check_role_based_detail: None,
            check_smtp_verification_status: None,
            check_smtp_verification_message: None,
            check_smtp_verification_detail: None,
            check_pwned_check_status: None,
            check_pwned_check_message: None,
            check_pwned_check_detail: None,
            check_catch_all_detection_status: None,
            check_catch_all_detection_message: None,
            check_catch_all_detection_detail: None,
            check_spam_trap_detection_status: None,
            check_spam_trap_detection_message: None,
            check_spam_trap_detection_detail: None,
        }),
        ValidationApiResponse::Error { .. } => Ok(empty_error_result()),
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

fn bool_to_text(value: Option<bool>) -> Option<String> {
    value.map(|v| v.to_string())
}

fn check_triplet(
    check: Option<&ValidationCheck>,
) -> (Option<String>, Option<String>, Option<String>) {
    match check {
        Some(value) => (
            Some(value.status.clone()),
            Some(value.message.clone()),
            value.detail.clone(),
        ),
        None => (None, None, None),
    }
}

fn empty_error_result() -> DatasetResult {
    DatasetResult {
        email: String::new(),
        status: "error".to_string(),
        valid: "error".to_string(),
        flags_disposable: None,
        flags_role_based: None,
        flags_catch_all: None,
        flags_spam_trap: None,
        flags_pwned_signal: None,
        check_format_status: None,
        check_format_message: None,
        check_format_detail: None,
        check_domain_existence_status: None,
        check_domain_existence_message: None,
        check_domain_existence_detail: None,
        check_mx_records_status: None,
        check_mx_records_message: None,
        check_mx_records_detail: None,
        check_disposable_temp_mail_status: None,
        check_disposable_temp_mail_message: None,
        check_disposable_temp_mail_detail: None,
        check_role_based_status: None,
        check_role_based_message: None,
        check_role_based_detail: None,
        check_smtp_verification_status: None,
        check_smtp_verification_message: None,
        check_smtp_verification_detail: None,
        check_pwned_check_status: None,
        check_pwned_check_message: None,
        check_pwned_check_detail: None,
        check_catch_all_detection_status: None,
        check_catch_all_detection_message: None,
        check_catch_all_detection_detail: None,
        check_spam_trap_detection_status: None,
        check_spam_trap_detection_message: None,
        check_spam_trap_detection_detail: None,
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
                flags: Default::default(),
                checks: Default::default(),
            }],
        });

        let mapped = map_dataset_result(payload).expect("mapping should succeed");
        assert_eq!(mapped.email, "user@example.com".to_string());
        assert_eq!(mapped.status, "false".to_string());
        assert_eq!(mapped.valid, "false".to_string());
    }
}

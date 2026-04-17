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
    #[serde(default)]
    pub checks: ValidationChecks,
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

#[derive(Debug, Deserialize, Default)]
pub struct ValidationChecks {
    #[serde(default)]
    pub format: Option<ValidationCheck>,
    #[serde(default)]
    pub domain_existence: Option<ValidationCheck>,
    #[serde(default)]
    pub mx_records: Option<ValidationCheck>,
    #[serde(default)]
    pub disposable_temp_mail: Option<ValidationCheck>,
    #[serde(default)]
    pub role_based: Option<ValidationCheck>,
    #[serde(default)]
    pub smtp_verification: Option<ValidationCheck>,
    #[serde(default)]
    pub pwned_check: Option<ValidationCheck>,
    #[serde(default)]
    pub catch_all_detection: Option<ValidationCheck>,
    #[serde(default)]
    pub spam_trap_detection: Option<ValidationCheck>,
}

#[derive(Debug, Deserialize)]
pub struct ValidationCheck {
    pub status: String,
    pub message: String,
    #[serde(default)]
    pub detail: Option<String>,
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
    pub check_format_status: Option<String>,
    pub check_format_message: Option<String>,
    pub check_format_detail: Option<String>,
    pub check_domain_existence_status: Option<String>,
    pub check_domain_existence_message: Option<String>,
    pub check_domain_existence_detail: Option<String>,
    pub check_mx_records_status: Option<String>,
    pub check_mx_records_message: Option<String>,
    pub check_mx_records_detail: Option<String>,
    pub check_disposable_temp_mail_status: Option<String>,
    pub check_disposable_temp_mail_message: Option<String>,
    pub check_disposable_temp_mail_detail: Option<String>,
    pub check_role_based_status: Option<String>,
    pub check_role_based_message: Option<String>,
    pub check_role_based_detail: Option<String>,
    pub check_smtp_verification_status: Option<String>,
    pub check_smtp_verification_message: Option<String>,
    pub check_smtp_verification_detail: Option<String>,
    pub check_pwned_check_status: Option<String>,
    pub check_pwned_check_message: Option<String>,
    pub check_pwned_check_detail: Option<String>,
    pub check_catch_all_detection_status: Option<String>,
    pub check_catch_all_detection_message: Option<String>,
    pub check_catch_all_detection_detail: Option<String>,
    pub check_spam_trap_detection_status: Option<String>,
    pub check_spam_trap_detection_message: Option<String>,
    pub check_spam_trap_detection_detail: Option<String>,
}

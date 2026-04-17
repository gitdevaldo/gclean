use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use tracing::info;

use crate::config::{default_dataset_dir, input_file_candidates};
use crate::error::ActorError;
use crate::models::{ActorInput, ValidationResponse};
use crate::service::EmailValidationService;

pub async fn run() -> Result<(), ActorError> {
    let input = load_input()?;
    let emails = normalize_emails(input)?;
    let api_token = normalize_api_token(env::var("VALIDATION_API_TOKEN").ok())?;
    let service = EmailValidationService::new()?;

    let mut results = Vec::with_capacity(emails.len());
    for email in emails {
        let result = service.validate(email, &api_token).await?;
        results.push(result);
    }

    let dataset_writer = DatasetWriter::new(default_dataset_dir())?;
    dataset_writer.append(&results)?;

    info!(items = results.len(), "actor run completed");
    let output_json = serde_json::to_string(&results).map_err(ActorError::SerializeDatasetItem)?;
    println!("{output_json}");

    Ok(())
}

fn load_input() -> Result<ActorInput, ActorError> {
    if let Ok(raw_json) = env::var("ACTOR_INPUT_JSON") {
        return Ok(serde_json::from_str::<ActorInput>(&raw_json)?);
    }

    let candidates = input_file_candidates();
    for candidate in &candidates {
        if candidate.exists() {
            let raw_json =
                fs::read_to_string(candidate).map_err(|source| ActorError::ReadInputFile {
                    path: candidate.clone(),
                    source,
                })?;
            return Ok(serde_json::from_str::<ActorInput>(&raw_json)?);
        }
    }

    let joined = candidates
        .iter()
        .map(|path| path.display().to_string())
        .collect::<Vec<_>>()
        .join(", ");

    Err(ActorError::InputNotFound(joined))
}

fn normalize_emails(input: ActorInput) -> Result<Vec<String>, ActorError> {
    let mut all_emails = Vec::new();
    if let Some(email) = input.email {
        all_emails.push(email);
    }
    all_emails.extend(input.emails);

    if all_emails.is_empty() {
        return Err(ActorError::EmptyEmails);
    }

    let emails: Vec<String> = all_emails
        .into_iter()
        .map(|email| email.trim().to_owned())
        .collect();

    Ok(emails)
}

fn normalize_api_token(raw_token: Option<String>) -> Result<String, ActorError> {
    let token = raw_token.unwrap_or_default().trim().to_owned();
    if token.is_empty() {
        return Err(ActorError::MissingApiToken);
    }

    Ok(token)
}

struct DatasetWriter {
    dataset_dir: PathBuf,
}

impl DatasetWriter {
    fn new(dataset_dir: PathBuf) -> Result<Self, ActorError> {
        fs::create_dir_all(&dataset_dir).map_err(|source| ActorError::CreateDatasetDir {
            path: dataset_dir.clone(),
            source,
        })?;
        Ok(Self { dataset_dir })
    }

    fn append(&self, items: &[ValidationResponse]) -> Result<(), ActorError> {
        let mut index = next_dataset_index(&self.dataset_dir)?;
        for item in items {
            let path = self.dataset_dir.join(format!("{index:09}.json"));
            let body = serde_json::to_vec(item).map_err(ActorError::SerializeDatasetItem)?;
            fs::write(&path, body)
                .map_err(|source| ActorError::WriteDatasetItem { path, source })?;
            index += 1;
        }
        Ok(())
    }
}

fn next_dataset_index(dataset_dir: &Path) -> Result<u64, ActorError> {
    let mut highest = 0_u64;

    let entries = fs::read_dir(dataset_dir).map_err(|source| ActorError::ScanDatasetDir {
        path: dataset_dir.to_path_buf(),
        source,
    })?;

    for entry in entries {
        let entry = entry.map_err(|source| ActorError::ScanDatasetDir {
            path: dataset_dir.to_path_buf(),
            source,
        })?;

        let entry_path = entry.path();
        let Some(stem) = entry_path.file_stem().and_then(|s| s.to_str()) else {
            continue;
        };
        let Ok(value) = stem.parse::<u64>() else {
            continue;
        };
        if value > highest {
            highest = value;
        }
    }

    Ok(highest + 1)
}

#[cfg(test)]
mod tests {
    use super::{normalize_api_token, normalize_emails};
    use crate::models::ActorInput;

    #[test]
    fn normalize_emails_trims_and_keeps_all_emails() {
        let input = ActorInput {
            email: Some(" single@example.com ".to_string()),
            emails: vec![
                " user@example.com ".to_string(),
                "".to_string(),
                "user@example.com".to_string(),
            ],
        };

        let emails = normalize_emails(input).expect("normalized emails should be returned");
        assert_eq!(
            emails,
            vec![
                "single@example.com".to_string(),
                "user@example.com".to_string(),
                "".to_string(),
                "user@example.com".to_string()
            ]
        );
    }

    #[test]
    fn normalize_emails_rejects_missing_email_input() {
        let input = ActorInput {
            email: None,
            emails: vec![],
        };

        let result = normalize_emails(input);
        assert!(result.is_err());
    }

    #[test]
    fn normalize_api_token_rejects_missing_token() {
        let result = normalize_api_token(None);
        assert!(result.is_err());
    }

    #[test]
    fn normalize_api_token_uses_trimmed_value() {
        let token = normalize_api_token(Some(" env-token ".to_string()))
            .expect("normalized token should be returned");
        assert_eq!(token, "env-token".to_string());
    }

    #[test]
    fn normalize_api_token_rejects_blank_token() {
        let result = normalize_api_token(Some("   ".to_string()));
        assert!(result.is_err());
    }
}

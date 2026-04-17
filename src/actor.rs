use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use reqwest::StatusCode;
use tokio::time::{sleep, Duration};
use tracing::info;

use crate::config::{
    actor_input_key, apify_api_base_url, default_dataset_dir, default_dataset_id,
    default_key_value_store_id, input_file_candidates, REQUEST_DELAY_SECONDS,
};
use crate::error::ActorError;
use crate::models::{ActorInput, DatasetResult};
use crate::service::EmailValidationService;

pub async fn run() -> Result<(), ActorError> {
    let input = load_input().await?;
    let emails = normalize_emails(input)?;
    let api_token = normalize_api_token(env::var("VALIDATION_API_TOKEN").ok())?;
    let service = EmailValidationService::new()?;

    let mut results = Vec::with_capacity(emails.len());
    for email in emails {
        let mut result = service.validate(email.clone(), &api_token).await?;
        if result.email.is_empty() {
            result.email = email;
        }
        results.push(result);
        sleep(Duration::from_secs(REQUEST_DELAY_SECONDS)).await;
    }

    append_results(&results).await?;

    info!(items = results.len(), "actor run completed");
    let output_json = serde_json::to_string(&results).map_err(ActorError::SerializeDatasetItem)?;
    println!("{output_json}");

    Ok(())
}

async fn load_input() -> Result<ActorInput, ActorError> {
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

    if let Some(raw_json) = load_input_from_apify_api().await? {
        return Ok(serde_json::from_str::<ActorInput>(&raw_json)?);
    }

    let joined = candidates
        .iter()
        .map(|path| path.display().to_string())
        .collect::<Vec<_>>()
        .join(", ");

    Err(ActorError::InputNotFound(joined))
}

async fn load_input_from_apify_api() -> Result<Option<String>, ActorError> {
    let token = match env::var("APIFY_TOKEN").ok() {
        Some(value) if !value.trim().is_empty() => value,
        _ => return Ok(None),
    };

    let Some(base_url) = apify_api_base_url() else {
        return Ok(None);
    };

    let store_id = default_key_value_store_id();
    let input_key = actor_input_key();
    let url = format!(
        "{}/v2/key-value-stores/{}/records/{}?disableRedirect=true",
        base_url.trim_end_matches('/'),
        store_id,
        input_key
    );

    let response = reqwest::Client::new()
        .get(&url)
        .bearer_auth(token)
        .send()
        .await
        .map_err(ActorError::FetchApifyInput)?;

    if response.status() == StatusCode::NOT_FOUND {
        return Ok(None);
    }

    if !response.status().is_success() {
        return Err(ActorError::FetchApifyInputStatus {
            url,
            status: response.status().as_u16(),
        });
    }

    let payload = response.text().await.map_err(ActorError::FetchApifyInput)?;
    Ok(Some(payload))
}

async fn append_results(items: &[DatasetResult]) -> Result<(), ActorError> {
    if append_results_to_apify_api(items).await? {
        return Ok(());
    }

    let dataset_writer = DatasetWriter::new(default_dataset_dir())?;
    dataset_writer.append(items)
}

async fn append_results_to_apify_api(items: &[DatasetResult]) -> Result<bool, ActorError> {
    let token = match env::var("APIFY_TOKEN").ok() {
        Some(value) if !value.trim().is_empty() => value,
        _ => return Ok(false),
    };

    let Some(base_url) = apify_api_base_url() else {
        return Ok(false);
    };

    let dataset_id = default_dataset_id();
    let url = format!(
        "{}/v2/datasets/{}/items",
        base_url.trim_end_matches('/'),
        dataset_id
    );

    let response = reqwest::Client::new()
        .post(&url)
        .bearer_auth(token)
        .json(items)
        .send()
        .await
        .map_err(ActorError::StoreDatasetItems)?;

    if !response.status().is_success() {
        return Err(ActorError::StoreDatasetItemsStatus {
            url,
            status: response.status().as_u16(),
        });
    }

    Ok(true)
}

fn normalize_emails(input: ActorInput) -> Result<Vec<String>, ActorError> {
    if input.emails.is_empty() {
        return Err(ActorError::EmptyEmails);
    }

    let emails: Vec<String> = input
        .emails
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

    fn append(&self, items: &[DatasetResult]) -> Result<(), ActorError> {
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
                "user@example.com".to_string(),
                "".to_string(),
                "user@example.com".to_string()
            ]
        );
    }

    #[test]
    fn normalize_emails_rejects_missing_email_input() {
        let input = ActorInput { emails: vec![] };

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

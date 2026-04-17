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
    let service = EmailValidationService::new()?;

    let mut results = Vec::with_capacity(emails.len());
    for email in emails {
        let result = service.validate(email).await?;
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
    use super::normalize_emails;
    use crate::models::ActorInput;

    #[test]
    fn normalize_emails_trims_and_keeps_all_inputs() {
        let input = ActorInput {
            emails: vec![
                " user@example.com ".to_string(),
                "".to_string(),
                "user@example.com".to_string(),
            ],
        };

        let output = normalize_emails(input).expect("normalized emails should be returned");
        assert_eq!(
            output,
            vec![
                "user@example.com".to_string(),
                "".to_string(),
                "user@example.com".to_string()
            ]
        );
    }
}

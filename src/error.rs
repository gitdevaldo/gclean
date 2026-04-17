use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ActorError {
    #[error("input not found; expected one of: {0}")]
    InputNotFound(String),
    #[error("failed to read input file {path}: {source}")]
    ReadInputFile {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("input JSON is invalid: {0}")]
    InvalidInputJson(#[from] serde_json::Error),
    #[error("failed to serialize dataset item: {0}")]
    SerializeDatasetItem(serde_json::Error),
    #[error("input must contain at least one email")]
    EmptyEmails,
    #[error("missing API token; set VALIDATION_API_TOKEN env var")]
    MissingApiToken,
    #[error("failed to fetch actor input from Apify API: {0}")]
    FetchApifyInput(reqwest::Error),
    #[error("failed to fetch actor input from Apify API ({url}): HTTP {status}")]
    FetchApifyInputStatus { url: String, status: u16 },
    #[error("failed to store dataset items via Apify API: {0}")]
    StoreDatasetItems(reqwest::Error),
    #[error("failed to store dataset items via Apify API ({url}): HTTP {status}")]
    StoreDatasetItemsStatus { url: String, status: u16 },
    #[error("request to validation API failed: {0}")]
    RequestFailed(#[from] reqwest::Error),
    #[error("failed to create dataset directory {path}: {source}")]
    CreateDatasetDir {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("failed to scan dataset directory {path}: {source}")]
    ScanDatasetDir {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("failed to write dataset item {path}: {source}")]
    WriteDatasetItem {
        path: PathBuf,
        source: std::io::Error,
    },
}

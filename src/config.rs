use std::env;
use std::path::PathBuf;

pub const VALIDATION_API_URL: &str = "https://zrbot.devaldo.workers.dev/v1/validate-email";
pub const REQUEST_TIMEOUT_SECONDS: u64 = 15;
pub const DEFAULT_LOG_LEVEL: &str = "info";

pub fn input_file_candidates() -> Vec<PathBuf> {
    let mut candidates = Vec::new();

    if let Ok(storage_dir) = env::var("APIFY_LOCAL_STORAGE_DIR") {
        candidates.push(PathBuf::from(&storage_dir).join("key_value_stores/default/INPUT.json"));
        candidates.push(PathBuf::from(storage_dir).join("key_value_stores/default/INPUT"));
    }

    candidates.push(PathBuf::from(
        "/tmp/apify_storage/key_value_stores/default/INPUT.json",
    ));
    candidates.push(PathBuf::from(
        "./storage/key_value_stores/default/INPUT.json",
    ));

    candidates
}

pub fn default_dataset_dir() -> PathBuf {
    if let Ok(storage_dir) = env::var("APIFY_LOCAL_STORAGE_DIR") {
        return PathBuf::from(storage_dir).join("datasets/default");
    }

    PathBuf::from("/tmp/apify_storage/datasets/default")
}

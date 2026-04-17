use std::env;
use std::path::PathBuf;

pub const VALIDATION_API_URL: &str = "https://zrbot.devaldo.workers.dev/v1/validate-email";
pub const REQUEST_TIMEOUT_SECONDS: u64 = 15;
pub const DEFAULT_LOG_LEVEL: &str = "info";

pub fn actor_input_key() -> String {
    env::var("ACTOR_INPUT_KEY")
        .or_else(|_| env::var("APIFY_INPUT_KEY"))
        .unwrap_or_else(|_| "INPUT".to_string())
}

pub fn default_key_value_store_id() -> String {
    env::var("ACTOR_DEFAULT_KEY_VALUE_STORE_ID")
        .or_else(|_| env::var("APIFY_DEFAULT_KEY_VALUE_STORE_ID"))
        .unwrap_or_else(|_| "default".to_string())
}

pub fn default_dataset_id() -> String {
    env::var("ACTOR_DEFAULT_DATASET_ID")
        .or_else(|_| env::var("APIFY_DEFAULT_DATASET_ID"))
        .unwrap_or_else(|_| "default".to_string())
}

pub fn apify_api_base_url() -> Option<String> {
    let value = env::var("APIFY_API_BASE_URL")
        .or_else(|_| env::var("APIFY_API_PUBLIC_BASE_URL"))
        .ok()?;

    let trimmed = value.trim();
    if trimmed.is_empty() {
        return None;
    }

    if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
        Some(trimmed.to_string())
    } else {
        Some(format!("https://{trimmed}"))
    }
}

pub fn input_file_candidates() -> Vec<PathBuf> {
    let mut candidates = Vec::new();
    let input_key = actor_input_key();
    let store_id = default_key_value_store_id();

    let store_relative_path = PathBuf::from("key_value_stores")
        .join(&store_id)
        .join(&input_key);

    if let Ok(storage_dir) = env::var("APIFY_LOCAL_STORAGE_DIR") {
        candidates.push(
            PathBuf::from(&storage_dir)
                .join(&store_relative_path)
                .with_extension("json"),
        );
        candidates.push(PathBuf::from(storage_dir).join(&store_relative_path));
    }

    candidates.push(
        PathBuf::from("/tmp/apify_storage")
            .join(&store_relative_path)
            .with_extension("json"),
    );
    candidates.push(PathBuf::from("/tmp/apify_storage").join(&store_relative_path));
    candidates.push(
        PathBuf::from("./storage")
            .join(&store_relative_path)
            .with_extension("json"),
    );
    candidates.push(PathBuf::from("./storage").join(store_relative_path));

    candidates
}

pub fn default_dataset_dir() -> PathBuf {
    let dataset_id = default_dataset_id();

    if let Ok(storage_dir) = env::var("APIFY_LOCAL_STORAGE_DIR") {
        return PathBuf::from(storage_dir).join("datasets").join(dataset_id);
    }

    PathBuf::from("/tmp/apify_storage")
        .join("datasets")
        .join(dataset_id)
}

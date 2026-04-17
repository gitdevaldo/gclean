use std::env;
use std::path::PathBuf;

pub const VALIDATION_API_URL: &str = "https://zrbot.devaldo.workers.dev/v1/validate-email";
pub const REQUEST_TIMEOUT_SECONDS: u64 = 15;
pub const DEFAULT_LOG_LEVEL: &str = "info";

pub fn input_file_candidates() -> Vec<PathBuf> {
    let mut candidates = Vec::new();
    let input_key = env::var("APIFY_INPUT_KEY").unwrap_or_else(|_| "INPUT".to_string());
    let store_id =
        env::var("APIFY_DEFAULT_KEY_VALUE_STORE_ID").unwrap_or_else(|_| "default".to_string());

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
    if let Ok(storage_dir) = env::var("APIFY_LOCAL_STORAGE_DIR") {
        return PathBuf::from(storage_dir).join("datasets/default");
    }

    PathBuf::from("/tmp/apify_storage/datasets/default")
}

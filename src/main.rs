use email_validator_api::actor;
use email_validator_api::config::DEFAULT_LOG_LEVEL;
use email_validator_api::telemetry::init_tracing;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing(DEFAULT_LOG_LEVEL);
    actor::run().await?;
    Ok(())
}

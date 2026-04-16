use tokio::net::TcpListener;
use tokio::signal;
use tracing::{error, info};

use email_validator_api::app::create_router;
use email_validator_api::config::AppConfig;
use email_validator_api::service::EmailValidationService;
use email_validator_api::state::AppState;
use email_validator_api::telemetry::init_tracing;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let config = AppConfig::from_env().map_err(std::io::Error::other)?;

    init_tracing(&config.log_level);

    let listener = TcpListener::bind(config.socket_addr()).await?;
    let app = create_router(AppState::new(EmailValidationService), &config);
    info!(address = %config.socket_addr(), "server started");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        if let Err(error) = signal::ctrl_c().await {
            error!(%error, "failed to install CTRL+C handler");
        }
    };

    #[cfg(unix)]
    let terminate = async {
        match signal::unix::signal(signal::unix::SignalKind::terminate()) {
            Ok(mut signal_stream) => {
                signal_stream.recv().await;
            }
            Err(error) => {
                error!(%error, "failed to install SIGTERM handler");
            }
        }
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("shutdown signal received");
}

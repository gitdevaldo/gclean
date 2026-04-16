use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub host: IpAddr,
    pub port: u16,
    pub request_timeout_seconds: u64,
    pub log_level: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, String> {
        let default = Self::default();

        let host = match env::var("APP_HOST") {
            Ok(value) => value
                .parse::<IpAddr>()
                .map_err(|_| "APP_HOST must be a valid IPv4/IPv6 address".to_string())?,
            Err(_) => default.host,
        };

        let port = parse_env_or_default("APP_PORT", default.port)?;
        let request_timeout_seconds = parse_env_or_default(
            "APP_REQUEST_TIMEOUT_SECONDS",
            default.request_timeout_seconds,
        )?;
        let log_level = env::var("RUST_LOG").unwrap_or(default.log_level);

        Ok(Self {
            host,
            port,
            request_timeout_seconds,
            log_level,
        })
    }

    pub fn socket_addr(&self) -> SocketAddr {
        SocketAddr::from((self.host, self.port))
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            host: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
            port: 8080,
            request_timeout_seconds: 10,
            log_level: "info".to_string(),
        }
    }
}

fn parse_env_or_default<T>(key: &str, default_value: T) -> Result<T, String>
where
    T: std::str::FromStr + Copy,
{
    match env::var(key) {
        Ok(value) => value
            .parse::<T>()
            .map_err(|_| format!("{key} has an invalid value")),
        Err(_) => Ok(default_value),
    }
}

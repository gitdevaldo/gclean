use std::time::Duration;

use reqwest::Client;

use crate::config::{REQUEST_TIMEOUT_SECONDS, VALIDATION_API_URL};
use crate::error::ActorError;
use crate::models::{ValidationRequest, ValidationResponse};

#[derive(Clone)]
pub struct EmailValidationService {
    client: Client,
}

impl EmailValidationService {
    pub fn new() -> Result<Self, ActorError> {
        let client = Client::builder()
            .timeout(Duration::from_secs(REQUEST_TIMEOUT_SECONDS))
            .build()?;
        Ok(Self { client })
    }

    pub async fn validate(
        &self,
        email: String,
        api_token: &str,
    ) -> Result<ValidationResponse, ActorError> {
        let response = self
            .client
            .post(VALIDATION_API_URL)
            .bearer_auth(api_token)
            .json(&ValidationRequest { email })
            .send()
            .await?;

        Ok(response.json::<ValidationResponse>().await?)
    }
}

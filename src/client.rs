use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::payout::PayoutResponse;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("Invalid credentials provided for martis client")]
    InvalidCredentials,

    #[error("Reqwest error: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("API request failed with status: {status} - {body}")]
    ApiError { status: u16, body: String },
}

#[derive(Debug)]
pub struct Client {
    api_key: String,
    base_url: String,
}

impl Client {
    pub fn new(api_key: &str) -> Result<Self, ClientError> {
        if api_key.is_empty() || !api_key.starts_with("sk_") {
            return Err(ClientError::InvalidCredentials);
        }

        Ok(Client { 
            api_key: api_key.to_string(),
            base_url: "http://localhost:5279/".to_string(),
        })
    }

    pub fn with_base_url(&mut self, base_url: &str) -> &mut Self {
        self.base_url = base_url.to_string();
        self
    }

    pub async fn create_payout(&self, request: CreatePayoutRequest) -> Result<PayoutResponse, ClientError> {
        let client = reqwest::Client::new();
        let url = format!("{}/v1/payouts", self.base_url.trim_end_matches('/'));
        
        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            match response.json::<PayoutResponse>().await {
                Ok(payout_response) => Ok(payout_response),
                Err(e) => Err(ClientError::RequestError(e)),
            }
        } else {
            let status = response.status().as_u16();
            let body = response.text().await.unwrap_or_else(|_| "Could not read response body".to_string());
            Err(ClientError::ApiError { status, body })
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePayoutRequest {
    pub client_reference_id: String,
    pub amount: i64,
    pub currency: String,
    pub destination: PayoutDestination,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayoutDestination {
    #[serde(rename = "type")]
    pub destination_type: String,
    pub bank_account: BankAccountDestination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankAccountDestination {
    pub bank_code: String,
    pub account_number: String,
    pub account_holder_name: String,
}
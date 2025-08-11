use crate::common::settings::Settings;
use loco_rs::prelude::*; // This imports most common Loco types including Error
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RequestToken {
    pub act: String,
    pub username: String,
    pub password: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Token {
    pub token: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenResponse {
    pub error_code: i64,
    pub error_desc: Option<String>,
    pub data: Option<Token>,
}

impl Token {
    /// Fetches authentication token from the Feeder API.
    ///
    /// This function connects to the configured Feeder service and retrieves an
    /// authentication token required for further API requests.
    ///
    /// # Errors
    ///
    /// This function will return an error in the following situations:
    /// - If the HTTP request to the Feeder API fails
    /// - If the response body cannot be parsed as JSON
    /// - If the response data structure is unexpected or missing fields
    ///
    /// # Panics
    ///
    /// This function may panic if the HTTP request succeeds but returns an invalid response
    /// when attempting to unwrap the HTTP result.
    pub async fn get(ctx: AppContext) -> Result<String, Error> {
        if let Some(settings) = &ctx.config.settings {
            let settings = Settings::from_json(settings)?;
            // println!("settings: {:#?}", settings);
            let feeder_url = settings.feeder_url;
            let feeder_username = settings.feeder_username;
            let feeder_password = settings.feeder_password;

            let request_token = RequestToken {
                act: "GetToken".to_string(),
                username: feeder_username,
                password: feeder_password,
            };

            // println!("request token: {:#?}", request_token.clone());
            // println!("request token: {:#?}", feeder_url.clone());

            let http_client: Client = Client::new();
            let http_result = http_client
                .post(feeder_url)
                .timeout(tokio::time::Duration::from_secs(5))
                .json(&request_token)
                .send()
                .await;

            // Handle the HTTP result without unwrapping
            let response = match http_result {
                Ok(res) => match res.json::<TokenResponse>().await {
                    Ok(token_response) => token_response,
                    Err(_) => return Err(Error::string("Failed to parse response JSON")),
                },
                Err(_) => return Err(Error::string("Failed to send request")),
            };

            // Check if data exists
            match response.data {
                Some(token_data) => Ok(token_data.token),
                None => Err(Error::string("Token data not found in response")),
            }
        } else {
            Err(Error::Message(
                "fail to register user because setting not loaded".to_string(),
            ))
        }
    }
}

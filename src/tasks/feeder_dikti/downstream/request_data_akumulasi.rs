use crate::common::settings::Settings;
use crate::services::feeder_dikti::requester::token::Token;
use loco_rs::prelude::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringOrInt {
    Int(i32),
    Str(String),
}

impl fmt::Display for StringOrInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StringOrInt::Int(i) => write!(f, "{}", i),
            StringOrInt::Str(s) => write!(f, "{}", s),
        }
    }
}

impl StringOrInt {
    /// Converts StringOrInt to i32
    ///
    /// # Errors
    ///
    /// Returns an error if the string variant cannot be parsed as i32
    pub fn to_i32(&self) -> Result<i32, std::num::ParseIntError> {
        match self {
            StringOrInt::Int(i) => Ok(*i),
            StringOrInt::Str(s) => s.parse::<i32>(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RequestDataAkumulasi {
    pub act: String,
    pub token: String,
    // pub filter: Option<String>,
    // pub order: Option<String>,
    // pub limit: Option<i32>,
    // pub offset: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReturnDataAkumulasi {
    pub error_code: i32,
    pub error_desc: Option<String>,
    pub(crate) data: StringOrInt,
}

impl RequestDataAkumulasi {
    /// Fetches data from the Feeder Dikti API
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - Failed to get a token from the token service
    /// - Settings are not properly loaded
    /// - HTTP request to Feeder API fails
    /// - Response parsing fails
    pub async fn get(ctx: &AppContext, action: String) -> Result<ReturnDataAkumulasi, Error> {
        let token = match Token::get(ctx.clone()).await {
            Ok(token) => token,
            Err(err) => {
                return Err(Error::Message(format!("Fail To Request: {err}")));
            }
        };

        // tracing::info!("Sending request to Feeder Dikti TOKEN with token: {}", token.clone());

        // Now complete the implementation with the token
        if let Some(settings) = &ctx.config.settings {
            let settings = Settings::from_json(settings)?;
            let feeder_url = settings.feeder_url;

            let request_data = Self {
                act: action.clone(),
                token, // filter: None,
                       // order: None,
                       // limit: None,
                       // offset: None,
            };

            // tracing::info!("Sending request to Feeder Dikti API with action: {}", action);
            // tracing::debug!("Request data: {:?}", request_data);

            let http_client: Client = Client::new();
            let response = match http_client
                .post(&feeder_url)
                .timeout(tokio::time::Duration::from_secs(10))
                .json(&request_data)
                .send()
                .await
            {
                Ok(res) => {
                    // Get the response status first
                    let _status = res.status();
                    // tracing::debug!("HTTP response status: {}", status);

                    // Get the raw response text for debugging
                    match res.text().await {
                        Ok(response_text) => {
                            // tracing::debug!("Raw response text: {}", response_text);

                            // Try to parse the response text as JSON
                            match serde_json::from_str::<ReturnDataAkumulasi>(&response_text) {
                                Ok(data) => {
                                    // tracing::info!("Successfully parsed response for action: {}", action);
                                    data
                                }
                                Err(err) => {
                                    // tracing::error!("JSON parsing failed for action: {}. Error: {}. Response: {}",
                                    //     action, err, response_text);
                                    return Err(Error::Message(format!(
                                        "Failed to parse response JSON for action '{}': {}. Response was: {}",
                                        action, err, response_text
                                    )));
                                }
                            }
                        }
                        Err(err) => {
                            return Err(Error::Message(format!(
                                "Failed to read response text: {err}"
                            )));
                        }
                    }
                }
                Err(err) => {
                    // tracing::error!("HTTP request failed for action: {}. Error: {}", action, err);
                    return Err(Error::Message(format!("Failed to send request: {err}")));
                }
            };

            Ok(response)
        } else {
            Err(Error::Message("Settings not loaded".to_string()))
        }
    }
}

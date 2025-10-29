use crate::common::settings::Settings;
use crate::services::feeder_dikti::requester::token::Token;
use loco_rs::prelude::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents a value that can be either a String or an Integer
/// Used for API responses where the data field can be either type
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringOrInt {
    Int(i32),
    Str(String),
}

impl fmt::Display for StringOrInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StringOrInt::Int(i) => write!(f, "{i}"),
            StringOrInt::Str(s) => write!(f, "{s}"),
        }
    }
}

impl StringOrInt {
    /// Converts `StringOrInt` to `i32`
    ///
    /// # Errors
    ///
    /// Returns an error if the string variant cannot be parsed as `i32`
    pub fn to_i32(&self) -> Result<i32, std::num::ParseIntError> {
        match self {
            StringOrInt::Int(i) => Ok(*i),
            StringOrInt::Str(s) => s.parse::<i32>(),
        }
    }
}

/// Input data for making a request to Feeder Dikti API
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputRequestData {
    pub act: String,
    pub filter: Option<String>,
    pub order: Option<String>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

/// Request data structure sent to Feeder Dikti API
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RequestData {
    pub act: String,
    pub token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}

/// Return data structure from Feeder Dikti API
///
/// The `jumlah` field is optional as some API endpoints return it while others don't.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReturnData<T> {
    pub error_code: i32,
    pub error_desc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jumlah: Option<i32>,
    pub data: Option<Vec<T>>,
}

/// Return data structure for scalar responses (e.g., count/akumulasi endpoints)
///
/// Used when the API returns a single scalar value instead of an array
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReturnDataScalar {
    pub error_code: i32,
    pub error_desc: Option<String>,
    pub data: StringOrInt,
}

impl RequestData {
    /// Fetches data from the Feeder Dikti API
    ///
    /// # Arguments
    /// * `ctx` - Application context containing configuration and database connection
    /// * `input` - Input request data containing action and optional parameters
    ///
    /// # Returns
    /// * `Result<ReturnData<T>, Error>` - Parsed response data or error
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - Failed to get a token from the token service
    /// - Settings are not properly loaded
    /// - HTTP request to Feeder API fails
    /// - Response parsing fails
    ///
    /// # Example
    /// ```rust
    /// let response = RequestData::get::<MyDataType>(
    ///     &ctx,
    ///     InputRequestData {
    ///         act: "GetSomeData".to_string(),
    ///         filter: Some("status='active'".to_string()),
    ///         order: Some("created_at DESC".to_string()),
    ///         limit: Some(100),
    ///         offset: Some(0),
    ///     },
    /// ).await?;
    /// ```
    pub async fn get<T>(ctx: &AppContext, input: InputRequestData) -> Result<ReturnData<T>, Error>
    where
        T: for<'de> Deserialize<'de>,
    {
        // Get authentication token
        let token = match Token::get(ctx.clone()).await {
            Ok(token) => token,
            Err(err) => {
                return Err(Error::Message(format!("Failed to get token: {err}")));
            }
        };

        // Get Feeder API URL from settings
        let feeder_url = if let Some(settings) = &ctx.config.settings {
            let settings = Settings::from_json(settings)?;
            settings.feeder_url
        } else {
            return Err(Error::Message("Settings not loaded".to_string()));
        };

        // Build request data
        let request_data = Self {
            act: input.act.clone(),
            token,
            filter: input.filter,
            order: input.order,
            limit: input.limit,
            offset: input.offset,
        };

        // tracing::info!("Sending request to Feeder Dikti API with action: {}", input.act);
        // tracing::debug!("Request data: {:?}", request_data);

        // Make HTTP request
        let http_client: Client = Client::new();
        let response = match http_client
            .post(&feeder_url)
            .timeout(tokio::time::Duration::from_secs(15))
            .json(&request_data)
            .send()
            .await
        {
            Ok(res) => {
                // Get the response status first
                let status = res.status();
                // tracing::debug!("HTTP response status: {}", status);

                if !status.is_success() {
                    return Err(Error::Message(format!(
                        "HTTP request failed with status: {}",
                        status
                    )));
                }

                // Get the raw response text for debugging
                match res.text().await {
                    Ok(response_text) => {
                        // tracing::debug!("Raw response text: {}", response_text);

                        // First try to trim whitespace and parse
                        let trimmed_text = response_text.trim();

                        // Try to parse the response text as JSON
                        match serde_json::from_str::<ReturnData<T>>(trimmed_text) {
                            Ok(data) => {
                                // tracing::info!("Successfully parsed response for action: {}", input.act);
                                data
                            }
                            Err(err) => {
                                // If parsing fails due to trailing input, try to clean the JSON
                                if err.to_string().contains("trailing") {
                                    // Try to extract valid JSON by finding matching braces
                                    // Count braces to find where the JSON actually ends
                                    let mut brace_count = 0;
                                    let mut bracket_count = 0;
                                    let mut in_string = false;
                                    let mut escape_next = false;
                                    let mut json_end: Option<usize> = None;

                                    for (i, ch) in trimmed_text.char_indices() {
                                        if escape_next {
                                            escape_next = false;
                                            continue;
                                        }

                                        match ch {
                                            '\\' if in_string => escape_next = true,
                                            '"' => in_string = !in_string,
                                            '{' if !in_string => brace_count += 1,
                                            '}' if !in_string => {
                                                brace_count -= 1;
                                                if brace_count == 0 && bracket_count == 0 {
                                                    json_end = Some(i);
                                                    break;
                                                }
                                            }
                                            '[' if !in_string => bracket_count += 1,
                                            ']' if !in_string => {
                                                bracket_count -= 1;
                                                if brace_count == 0 && bracket_count == 0 {
                                                    json_end = Some(i);
                                                    break;
                                                }
                                            }
                                            _ => {}
                                        }
                                    }

                                    if let Some(end_pos) = json_end {
                                        let clean_json = &trimmed_text[..=end_pos];
                                        match serde_json::from_str::<ReturnData<T>>(clean_json) {
                                            Ok(data) => {
                                                // tracing::info!("Successfully parsed cleaned JSON for action: {}", input.act);
                                                data
                                            }
                                            Err(_) => {
                                                // tracing::error!("JSON parsing failed for action: {}. Error: {}. Response: {}",
                                                //     input.act, err, response_text);
                                                return Err(Error::Message(format!(
                                                    "Failed to parse response JSON for action '{}': {}. Response was: {}",
                                                    input.act, err, response_text
                                                )));
                                            }
                                        }
                                    } else {
                                        // tracing::error!("Could not find valid JSON end for action: {}. Response: {}", input.act, response_text);
                                        return Err(Error::Message(format!(
                                            "Failed to parse response JSON for action '{}': {}. Response was: {}",
                                            input.act, err, response_text
                                        )));
                                    }
                                } else {
                                    // tracing::error!("JSON parsing failed for action: {}. Error: {}. Response: {}",
                                    //     input.act, err, response_text);
                                    return Err(Error::Message(format!(
                                        "Failed to parse response JSON for action '{}': {}. Response was: {}",
                                        input.act, err, response_text
                                    )));
                                }
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
                // tracing::error!("HTTP request failed for action: {}. Error: {}", input.act, err);
                return Err(Error::Message(format!("Failed to send request: {err}")));
            }
        };

        Ok(response)
    }

    /// Fetches scalar data from the Feeder Dikti API (e.g., counts, aggregations)
    ///
    /// # Arguments
    /// * `ctx` - Application context containing configuration and database connection
    /// * `input` - Input request data containing action and optional parameters
    ///
    /// # Returns
    /// * `Result<ReturnDataScalar, Error>` - Parsed response with scalar data or error
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - Failed to get a token from the token service
    /// - Settings are not properly loaded
    /// - HTTP request to Feeder API fails
    /// - Response parsing fails
    ///
    /// # Example
    /// ```rust
    /// let response = RequestData::get_scalar(
    ///     &ctx,
    ///     InputRequestData {
    ///         act: "GetJumlahMahasiswa".to_string(),
    ///         filter: Some("id_prodi='12345'".to_string()),
    ///         order: None,
    ///         limit: None,
    ///         offset: None,
    ///     },
    /// ).await?;
    ///
    /// let count = response.data.to_i32()?;
    /// ```
    pub async fn get_scalar(
        ctx: &AppContext,
        input: InputRequestData,
    ) -> Result<ReturnDataScalar, Error> {
        // Get authentication token
        let token = match Token::get(ctx.clone()).await {
            Ok(token) => token,
            Err(err) => {
                return Err(Error::Message(format!("Failed to get token: {err}")));
            }
        };

        // Get Feeder API URL from settings
        let feeder_url = if let Some(settings) = &ctx.config.settings {
            let settings = Settings::from_json(settings)?;
            settings.feeder_url
        } else {
            return Err(Error::Message("Settings not loaded".to_string()));
        };

        // Build request data
        let request_data = Self {
            act: input.act.clone(),
            token,
            filter: input.filter,
            order: input.order,
            limit: input.limit,
            offset: input.offset,
        };

        // tracing::info!("Sending request to Feeder Dikti API with action: {}", input.act);
        // tracing::debug!("Request data: {:?}", request_data);

        // Make HTTP request
        let http_client: Client = Client::new();
        let response = match http_client
            .post(&feeder_url)
            .timeout(tokio::time::Duration::from_secs(15))
            .json(&request_data)
            .send()
            .await
        {
            Ok(res) => {
                // Get the response status first
                let status = res.status();
                // tracing::debug!("HTTP response status: {}", status);

                if !status.is_success() {
                    return Err(Error::Message(format!(
                        "HTTP request failed with status: {status}",
                    )));
                }

                // Get the raw response text for debugging
                match res.text().await {
                    Ok(response_text) => {
                        // tracing::debug!("Raw response text: {}", response_text);

                        // First try to trim whitespace and parse
                        let trimmed_text = response_text.trim();

                        // Try to parse the response text as JSON
                        match serde_json::from_str::<ReturnDataScalar>(trimmed_text) {
                            Ok(data) => {
                                // tracing::info!("Successfully parsed scalar response for action: {}", input.act);
                                data
                            }
                            Err(err) => {
                                // If parsing fails due to trailing input, try to clean the JSON
                                if err.to_string().contains("trailing") {
                                    // Try to extract valid JSON by finding matching braces
                                    // Count braces to find where the JSON actually ends
                                    let mut brace_count = 0;
                                    let mut bracket_count = 0;
                                    let mut in_string = false;
                                    let mut escape_next = false;
                                    let mut json_end: Option<usize> = None;

                                    for (i, ch) in trimmed_text.char_indices() {
                                        if escape_next {
                                            escape_next = false;
                                            continue;
                                        }

                                        match ch {
                                            '\\' if in_string => escape_next = true,
                                            '"' => in_string = !in_string,
                                            '{' if !in_string => brace_count += 1,
                                            '}' if !in_string => {
                                                brace_count -= 1;
                                                if brace_count == 0 && bracket_count == 0 {
                                                    json_end = Some(i);
                                                    break;
                                                }
                                            }
                                            '[' if !in_string => bracket_count += 1,
                                            ']' if !in_string => {
                                                bracket_count -= 1;
                                                if brace_count == 0 && bracket_count == 0 {
                                                    json_end = Some(i);
                                                    break;
                                                }
                                            }
                                            _ => {}
                                        }
                                    }

                                    if let Some(end_pos) = json_end {
                                        let clean_json = &trimmed_text[..=end_pos];
                                        match serde_json::from_str::<ReturnDataScalar>(clean_json) {
                                            Ok(data) => {
                                                // tracing::info!("Successfully parsed cleaned scalar JSON for action: {}", input.act);
                                                data
                                            }
                                            Err(_) => {
                                                // tracing::error!("JSON parsing failed for action: {}. Error: {}. Response: {}",
                                                //     input.act, err, response_text);
                                                return Err(Error::Message(format!(
                                                    "Failed to parse response JSON for action '{}': {}. Response was: {}",
                                                    input.act, err, response_text
                                                )));
                                            }
                                        }
                                    } else {
                                        // tracing::error!("Could not find valid JSON end for action: {}. Response: {}", input.act, response_text);
                                        return Err(Error::Message(format!(
                                            "Failed to parse response JSON for action '{}': {}. Response was: {}",
                                            input.act, err, response_text
                                        )));
                                    }
                                } else {
                                    // tracing::error!("JSON parsing failed for action: {}. Error: {}. Response: {}",
                                    //     input.act, err, response_text);
                                    return Err(Error::Message(format!(
                                        "Failed to parse response JSON for action '{}': {}. Response was: {}",
                                        input.act, err, response_text
                                    )));
                                }
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
                // tracing::error!("HTTP request failed for action: {}. Error: {}", input.act, err);
                return Err(Error::Message(format!("Failed to send request: {err}")));
            }
        };

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_request_data_creation() {
        let input = InputRequestData {
            act: "TestAction".to_string(),
            filter: Some("test_filter".to_string()),
            order: Some("id ASC".to_string()),
            limit: Some(100),
            offset: Some(0),
        };

        assert_eq!(input.act, "TestAction");
        assert_eq!(input.filter, Some("test_filter".to_string()));
        assert_eq!(input.limit, Some(100));
    }

    #[test]
    fn test_input_request_data_with_nulls() {
        let input = InputRequestData {
            act: "TestAction".to_string(),
            filter: None,
            order: None,
            limit: None,
            offset: None,
        };

        assert_eq!(input.act, "TestAction");
        assert!(input.filter.is_none());
        assert!(input.order.is_none());
        assert!(input.limit.is_none());
        assert!(input.offset.is_none());
    }

    #[test]
    fn test_string_or_int_display() {
        let int_val = StringOrInt::Int(42);
        let str_val = StringOrInt::Str("2742".to_string());

        assert_eq!(format!("{}", int_val), "42");
        assert_eq!(format!("{}", str_val), "2742");
    }

    #[test]
    fn test_string_or_int_to_i32() {
        let int_val = StringOrInt::Int(42);
        let str_val = StringOrInt::Str("2742".to_string());
        let invalid_str = StringOrInt::Str("not_a_number".to_string());

        assert_eq!(int_val.to_i32().unwrap(), 42);
        assert_eq!(str_val.to_i32().unwrap(), 2742);
        assert!(invalid_str.to_i32().is_err());
    }

    #[test]
    fn test_return_data_scalar_deserialization() {
        // Test with integer data
        let json_int = r#"{
            "error_code": 0,
            "error_desc": "",
            "data": 2742
        }"#;
        let result: Result<ReturnDataScalar, _> = serde_json::from_str(json_int);
        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(data.error_code, 0);
        assert_eq!(data.data.to_i32().unwrap(), 2742);

        // Test with string data
        let json_str = r#"{
            "error_code": 0,
            "error_desc": "",
            "data": "2742"
        }"#;
        let result: Result<ReturnDataScalar, _> = serde_json::from_str(json_str);
        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(data.error_code, 0);
        assert_eq!(data.data.to_i32().unwrap(), 2742);
    }

    #[test]
    fn test_return_data_with_jumlah() {
        let json = r#"{
            "error_code": 0,
            "error_desc": "",
            "data": [{"id": 1}, {"id": 2}],
            "jumlah": 100
        }"#;

        #[derive(Debug, Serialize, Deserialize)]
        struct TestItem {
            id: i32,
        }

        let result: Result<ReturnData<TestItem>, _> = serde_json::from_str(json);
        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(data.error_code, 0);
        assert_eq!(data.jumlah, Some(100));
        assert!(data.data.is_some());
        assert_eq!(data.data.unwrap().len(), 2);
    }

    #[test]
    fn test_return_data_without_jumlah() {
        let json = r#"{
            "error_code": 0,
            "error_desc": "",
            "data": [{"id": 1}]
        }"#;

        #[derive(Debug, Serialize, Deserialize)]
        struct TestItem {
            id: i32,
        }

        let result: Result<ReturnData<TestItem>, _> = serde_json::from_str(json);
        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(data.error_code, 0);
        assert_eq!(data.jumlah, None);
        assert!(data.data.is_some());
    }
}

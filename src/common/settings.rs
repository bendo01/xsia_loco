use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct Settings {
    pub current_academic_year_id: String,
    pub current_student_admission_academic_year_id: String,
    pub current_institution_id: String,
    pub current_institution_code: String,
    pub app_directory: String,
    pub feeder_username: String,
    pub feeder_password: String,
    pub feeder_url: String,
    pub system_mail_name: String,
    pub system_mail_address: String,
    pub server_domain: String,
    pub is_production_midtrans_payment: bool,
}

impl Settings {
    /// Converts a JSON value into a Settings struct.
    ///
    /// # Arguments
    ///
    /// * `value` - A JSON value that should contain settings data
    ///
    /// # Returns
    ///
    /// A Result containing the Settings if successful
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The JSON value cannot be deserialized into a Settings struct
    /// - The JSON structure doesn't match the expected Settings format
    pub fn from_json(value: &serde_json::Value) -> Result<Self> {
        Ok(serde_json::from_value(value.clone())?)
    }
}

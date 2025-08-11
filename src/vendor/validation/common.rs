use chrono::NaiveDate;
use sea_orm::prelude::Date;
use serde_json::json;
use std::collections::HashMap;
use uuid::Uuid;
use validator::{ValidationError, ValidationErrors};

#[must_use]
#[allow(clippy::missing_errors_doc)]
pub fn format_validation_errors(
    errors: &ValidationErrors,
    custom_message: &str,
) -> serde_json::Value {
    let mut error_map: HashMap<String, Vec<String>> = HashMap::new();

    for (field, error) in errors.field_errors() {
        let field_errors: Vec<String> = error
            .iter()
            .map(|e| {
                e.message
                    .clone()
                    .unwrap_or_else(|| "invalid input".into())
                    .into_owned()
            })
            .collect();

        // Insert all errors as a list of messages for each field
        error_map.insert(field.to_string(), field_errors);
    }

    // Build the JSON structure with the custom message
    json!({
        "message": custom_message, // Use the custom message provided
        "errors": error_map
    })
}

#[allow(clippy::missing_errors_doc)]
pub fn validate_uuid_format(uuid_str: &str) -> Result<(), ValidationError> {
    if Uuid::parse_str(uuid_str).is_err() {
        return Err(ValidationError::new("invalid_uuid_format"));
    }
    Ok(())
}

#[allow(clippy::missing_errors_doc)]
pub fn validate_date_string(date_str: &str) -> Result<(), ValidationError> {
    match NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        Ok(date) => {
            let now = chrono::Local::now().naive_local().date();
            if date > now {
                return Err(ValidationError::new("date_in_future"));
            }
            Ok(())
        }
        Err(_) => Err(ValidationError::new("invalid_date_format")),
    }
}

#[allow(clippy::missing_errors_doc)]
pub fn validate_option_date(start_date: &Option<Date>) -> Result<(), ValidationError> {
    if start_date.is_none() {
        let mut err = ValidationError::new("required");
        err.message = Some("Tanggal mulai harus diisi".into());
        return Err(err);
    }
    Ok(())
}

#[allow(clippy::missing_errors_doc)]
pub fn validate_two_date(dates: &(Option<Date>, Option<Date>)) -> Result<(), ValidationError> {
    if let (Some(start), Some(end)) = dates {
        if end < start {
            let mut err = ValidationError::new("invalid_date_range");
            err.message = Some("Tanggal berakhir harus setelah tanggal mulai".into());
            return Err(err);
        }
    }
    Ok(())
}

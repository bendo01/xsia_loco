use crate::common::settings::Settings;
use base64::{Engine as _, engine::general_purpose::STANDARD};
use loco_rs::prelude::*;

pub struct EncodeService;

impl EncodeService {
    /// Encodes an image file to a base64 string.
    ///
    /// # Arguments
    ///
    /// * `image_path` - The relative path to the image file.
    ///
    /// # Returns
    ///
    /// A base64 encoded string of the image.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The image file cannot be read
    /// - The path is invalid
    pub fn base64_encode(ctx: &AppContext, image_path: &str) -> Result<String, Error> {
        // // Encode to base64 string
        // let encoded = STANDARD.encode(image_bytes);
        // Ok(encoded)
        if let Some(settings) = &ctx.config.settings {
            let settings = Settings::from_json(settings)?;
            let app_directory = settings.app_directory;
            let logo_path = format!("{app_directory}/{image_path}");
            let image_bytes = std::fs::read(logo_path)
                .map_err(|e| Error::Message(format!("Failed to read image: {e}")))?;
            // Encode to base64 string
            let encoded = STANDARD.encode(image_bytes);
            Ok(encoded)
        } else {
            Err(Error::Message(
                "fail to register user because setting not loaded".to_string(),
            ))
        }
    }
}

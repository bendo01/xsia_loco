use crate::services::image::encode::EncodeService;
use headless_chrome::{Browser, LaunchOptions, types::PrintToPdfOptions};
use loco_rs::prelude::*;
use std::ffi::OsStr;

/// Generates HTML content for PDF generation.
#[must_use]
pub fn generate_html_content() -> String {
    // Example: Replace this with SeaORM database query logic
    let content = "
        <div style=\"width: 100%; clear: both; margin: 0px; padding: 0px; display: block;\">
            <p>
            Lorem ipsum dolor sit amet, consectetur adipiscing elit. Mauris sollicitudin, arcu eu fermentum euismod, diam turpis convallis enim, a mollis diam felis a libero. Nam eu diam molestie, efficitur diam sed, convallis leo. Maecenas a justo velit. Sed vehicula justo at arcu rhoncus, ultricies pretium dolor faucibus. Nam odio sapien, dignissim quis facilisis quis, iaculis nec tortor. Aenean efficitur mauris nulla, sit amet iaculis libero tristique sit amet. Nulla eu ullamcorper metus. Curabitur sollicitudin molestie gravida.
            </p>
            <!-- ...more paragraphs... -->
        </div>
        ";

    format!(
        r#"
            <!DOCTYPE html>
            <html>
                <head>
                    <meta charset="utf-8">
                    <meta name="viewport" content="width=device-width, initial-scale=1">
                    <title>PDF Sample</title>

                </head>
                <body class="p-8">
                    {content}
                </body>
            </html>
        "#
    )
}

/// Prepares header and footer templates for PDF generation.
#[must_use]
pub fn prepare_templates(ctx: &AppContext) -> (String, String) {
    // Read and encode the logo image to base64
    let logo_base64 = match EncodeService::base64_encode(
        ctx,
        "public/img/favicon/default/android-chrome-512x512.png",
    ) {
        Ok(encoded) => encoded,
        Err(err) => {
            // Handle the error appropriately - here we'll just use a placeholder
            eprintln!("Failed to encode image: {err:?}");
            String::new() // Empty string as fallback
        }
    };

    // Header template with logo and title
    let header_template = format!(
        r#"
            <div style="width: 100%; clear: both; display: block;">
                <div id="pageHeader" style="width: 100%; clear: both; margin: 0px; padding: 0px; display: block; margin-top: -30px; padding-bottom: 15px;">
                    <div style="width: 100%; padding-top: 35px; margin-left: 30px; margin-right: 30px; display: block; clear: both;">
                        <div style="float:left; width:20%;">
                            <img src="data:image/png;base64, {logo_base64}" height="90" width="90" />
                        </div>
                        <div style="width: 80%;">
                            <p style="text-transform: uppercase; font-size: 24px; color: rgb(0, 128, 0); margin-top: 15px; padding-top: 0px; margin-bottom: 0px; padding-bottom: 0px; font-weight: 700; text-align: center;">
                                Yayasan Al-Agshan
                            </p>
                            <p style="text-transform: uppercase; font-size: 24px; color: rgb(0, 128, 0); margin-top: 0px; padding-top: 0px; margin-bottom: 0px; padding-bottom: 0px; font-weight: 700; text-align: center;">
                                Universitas Pancasakti
                            </p>
                            <p style="font-size: 10px; color: rgb(0 0 0); margin-top: 0px; padding-top: 0px; margin-bottom: 0px; padding-bottom: 0px; font-weight: 700; text-align: center;">
                                Jalan Andi Mangerangi 73, (0411) 871306 / 837247 Makassar
                            </p>
                        </div>
                    </div>
                </div>
                <hr style="margin-left: 30px; margin-right: 30px; border-width: 0.5px; border-style: solid; border-color: #C0C0C0;">
            </div>
        "#
    );

    // Footer template with page numbers
    let footer_template = r#"
            <div  id="pageFooter" style="width: 100%; text-align: center; font-size: 12px;">
                Halaman <span class="pageNumber"></span> dari <span class="totalPages"></span>
            </div>
        "#
    .to_string();
    (header_template, footer_template)
}

/// Renders data into an HTML template.
///
/// # Arguments
///
/// * `data` - The string data to be inserted into the template
///
/// # Returns
///
/// A string containing the rendered HTML.
///
/// # Errors
///
/// This function currently doesn't return errors, but the `Result` type is used
/// for consistency with other template rendering functions that might fail.
pub fn render_template(data: &str) -> Result<String, Error> {
    // Use your preferred templating engine (handlebars, askama, etc)
    // Here's a simple string replacement example:
    let html = format!(
        r#"
        <!-- Insert template HTML here -->
        <div class="p-8">
          {data:#?}
        </div>
    "#
    );

    Ok(html)
}

/// Generates a PDF document from HTML content with header and footer templates.
///
/// # Returns
///
/// A vector of bytes containing the generated PDF document.
///
/// # Errors
///
/// This function can fail in the following situations:
/// - If headless Chrome browser cannot be launched
/// - If a new browser tab cannot be created
/// - If navigation to the HTML content fails
/// - If PDF generation fails
pub fn generate_pdf(ctx: &AppContext) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Generate HTML content
    let html_content = generate_html_content();

    // Prepare header and footer templates
    let (header_template, footer_template) = prepare_templates(ctx);

    // Launch headless Chrome with file access enabled
    let browser = Browser::new(LaunchOptions {
        args: vec![OsStr::new("--allow-file-access-from-files")],
        ..Default::default()
    })?;

    let tab = browser.new_tab()?;

    // Navigate to content
    let content_url = format!("data:text/html,{}", urlencoding::encode(&html_content));
    tab.navigate_to(&content_url)?;

    // Print to PDF with header and footer
    let pdf_options = PrintToPdfOptions {
        display_header_footer: Some(true),
        header_template: Some(header_template),
        footer_template: Some(footer_template),
        print_background: Some(true),
        margin_top: Some(1.9), // Inches
        margin_bottom: Some(1.0),
        margin_left: Some(0.5),
        margin_right: Some(0.5),
        ..Default::default()
    };

    let pdf = tab.print_to_pdf(Some(pdf_options))?;

    Ok(pdf)
}

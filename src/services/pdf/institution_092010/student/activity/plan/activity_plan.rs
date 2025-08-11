use crate::models::academic::student::campaign::activities::data_objects::DataObject as AcademicStudentActivityDataObject;
use crate::services::image::encode::EncodeService;
use crate::services::pdf::signature::Signature;
use chrono::Local;
use headless_chrome::{Browser, LaunchOptions, types::PrintToPdfOptions};
use loco_rs::prelude::*;
use std::error::Error;
use std::ffi::OsStr;
use std::sync::OnceLock;
use tera::{Context, Tera};

// Create a static Tera instance that will be initialized once
static TERA: OnceLock<Tera> = OnceLock::new();

// Initialize Tera singleton
fn get_templates() -> &'static Tera {
    TERA.get_or_init(|| {
        let mut tera = Tera::default();

        // Embed the template directly in the binary
        // The path is relative to the project root when using the root-relative path
        let template_content = include_str!("activity_plan.html");

        // Add the template manually
        if let Err(e) = tera.add_raw_template("activity_plan.html", template_content) {
            eprintln!("Error adding template: {e}");
        }
        tera
    })
}

/// Generates HTML content for PDF generation.
#[must_use]
pub async fn generate_html_content(ctx: &AppContext, activity_id: Uuid) -> String {
    let student_activity_result =
        AcademicStudentActivityDataObject::get_by_id(ctx, activity_id, true).await;

    let student_activity = match student_activity_result {
        Ok(Some(activity)) => activity,
        Ok(None) => return "Student activity not found".to_string(),
        Err(_) => return "Error retrieving student activity from database".to_string(),
    };

    // Get the Tera instance
    let templates = get_templates();

    // Create a Signature instance
    let signature = Signature {};

    // Generate the signatures
    let student_signature = signature
        .generate_student_signature(ctx, student_activity.activity.id)
        .await;
    let cs_staff_signature = signature.generate_course_department_signature();
    let baak_signature = signature.generate_academic_bureau_signature();

    // Create a context with your data
    let mut context = Context::new();
    context.insert("student_activity", &student_activity);
    context.insert("cs_staff_signature", &cs_staff_signature);
    context.insert("student_signature", &student_signature);
    context.insert(
        "print_date",
        &Local::now().format("%d-%m-%Y %H:%M:%S").to_string(),
    );
    context.insert("baak_signature", &baak_signature);

    // Render the template
    match templates.render("activity_plan.html", &context) {
        Ok(rendered) => rendered,
        Err(e) => {
            // Log the primary error
            eprintln!("Template rendering error: {e}");

            // Get the chain of causes
            let mut source = e.source();
            while let Some(err) = source {
                eprintln!("Caused by: {err}");
                source = err.source();
            }

            // Log what we know about the context
            eprintln!("Attempting to render template with student_activity data");

            // Return formatted error
            format!("Failed to render template: {e}")
        }
    }
}

/// Prepares header and footer templates for PDF generation.
#[must_use]
pub fn prepare_templates(ctx: &AppContext) -> (String, String) {
    // Read and encode the logo image to base64
    let logo_base64 = match EncodeService::base64_encode(
        ctx,
        "public/img/favicon/092010/android-chrome-512x512.png",
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
                            <p style="text-transform: uppercase; font-size: 18px; color: rgb(0, 0, 255); margin-top: 12px; padding-top: 0px; margin-bottom: 0px; padding-bottom: 0px; font-weight: 700; text-align: center;">
                                INSTITUT TEKNOLOGI DAN KESEHATAN
                            </p>
                            <p style="text-transform: uppercase; font-size: 18px; color: rgb(255, 0, 0); margin-top: 0px; padding-top: 0px; margin-bottom: 0px; padding-bottom: 0px; font-weight: 700; text-align: center;">
                                TRI TUNAS NASIONAL
                            </p>
                            <p style="font-size: 8px; color: rgb(0 0 0); margin-top: 0px; padding-top: 0px; margin-bottom: 0px; padding-bottom: 0px; font-weight: 700; text-align: center;">
                                SK Kemendikbud Republik Indonesia Nomor 890/M/2020 Tanggal 21 September 2020
                            </p>
                            <p style="font-size: 8px; color: rgb(0 0 0); margin-top: 0px; padding-top: 0px; margin-bottom: 0px; padding-bottom: 0px; font-weight: 700; text-align: center;">
                                Jalan Tamangapa Raya No.168, Bangkala, Kec.Manggala, Kota Makassar 90235
                            </p>
                            <p style="font-size: 8px; color: rgb(0 0 0); margin-top: 0px; padding-top: 0px; margin-bottom: 0px; padding-bottom: 0px; font-weight: 700; text-align: center;">
                                Email: official@tritunas.ac.id
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
            <div  id="pageFooter" style="width: 100%; text-align: right; font-size: 8px; padding-right: 10px;">
                Halaman <span class="pageNumber"></span> dari <span class="totalPages"></span>
            </div>
        "#
    .to_string();
    (header_template, footer_template)
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
pub async fn generate_pdf(
    ctx: &AppContext,
    activity_id: Uuid,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Generate HTML content
    let html_content = generate_html_content(ctx, activity_id).await;

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
    tab.wait_until_navigated()?;

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

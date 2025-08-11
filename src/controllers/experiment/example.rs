use crate::services::pdf::html_content;
// use axum::debug_handler;
use loco_rs::prelude::*;
use std::env;

/// Returns data for function
///
/// # Errors
/// Returns an error if unable to retrieve model from database.
///
/// # Panics
/// Panics if the current directory path contains invalid UTF-8 characters.
pub async fn pdf_example(State(_ctx): State<AppContext>) -> Result<Response> {
    let current_dir = env::current_dir()?.to_str().unwrap().to_string();
    format::json(current_dir)
}

/// Returns data for function
///
/// # Errors
/// Returns an error if unable to retrieve model from database.
pub async fn generate_pdf_route(State(ctx): State<AppContext>) -> Result<Response> {
    let pdf_data = html_content::generate_pdf(&ctx).map_err(|e| Error::Message(e.to_string()))?;
    let response = Response::builder()
        .header("Content-Type", "application/pdf")
        .header("Content-Disposition", "attachment; filename=report.pdf")
        .body(pdf_data.into())
        .map_err(|e| Error::Message(e.to_string()))?;

    Ok(response)
}

pub fn routes(_ctx: &AppContext) -> Routes {
    Routes::new()
        .prefix("api/example")
        .add("/pdf_example", get(pdf_example))
        .add("/generate_pdf_route", get(generate_pdf_route))
}

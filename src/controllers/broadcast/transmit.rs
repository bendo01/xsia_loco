use crate::services::broadcast::broadcaster;
use axum::debug_handler;
use chrono::Local;
use loco_rs::prelude::*;
use serde_json::json;

#[debug_handler]
/// Returns data for function
///
/// # Errors
/// Returns an error if unable to retrieve model from database.
pub async fn index(State(_ctx): State<AppContext>) -> Result<Response> {
    broadcaster::broadcast(
        "broadcasting",
        json!({
            "message": "Hello, from broadcasting!",
            "type": "broadcast",
            "time": Local::now().to_string()
        }),
    )
    .await;
    let response = json!({
        "message": "Hello, Loco!",
        "status": 200
    });

    format::json(response)
}

pub fn routes() -> Routes {
    Routes::new().prefix("api/broadcast").add("/", get(index))
}

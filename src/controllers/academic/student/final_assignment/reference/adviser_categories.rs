#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::middleware::is_authorized::AuthorizedLayer;
use axum::debug_handler;
use loco_rs::prelude::*;

#[debug_handler]
pub async fn index(State(_ctx): State<AppContext>) -> Result<Response> {
    format::empty()
}

#[debug_handler]
pub async fn store(State(_ctx): State<AppContext>) -> Result<Response> {
    format::empty()
}

#[debug_handler]
pub async fn show(State(_ctx): State<AppContext>) -> Result<Response> {
    format::empty()
}

#[debug_handler]
pub async fn destroy(State(_ctx): State<AppContext>) -> Result<Response> {
    format::empty()
}

#[debug_handler]
pub async fn update(State(_ctx): State<AppContext>) -> Result<Response> {
    format::empty()
}

#[debug_handler]
pub async fn list(State(_ctx): State<AppContext>) -> Result<Response> {
    format::empty()
}

pub fn routes(ctx: &AppContext) -> Routes {
    Routes::new()
        .prefix("api/academic/student/final_assignment/reference/adviser_categories")
        .add("/list", get(list))
        .add(
            "/store",
            post(store).layer(AuthorizedLayer::new(
                ctx.clone(),
                "academic.campaign.reference.attend_types.index",
            )),
        )
        .add(
            "/{id}",
            get(show).layer(AuthorizedLayer::new(
                ctx.clone(),
                "academic.campaign.reference.attend_types.index",
            )),
        )
        .add(
            "/{id}",
            delete(destroy).layer(AuthorizedLayer::new(
                ctx.clone(),
                "academic.campaign.reference.attend_types.index",
            )),
        )
        .add(
            "/{id}",
            put(update).layer(AuthorizedLayer::new(
                ctx.clone(),
                "academic.campaign.reference.attend_types.index",
            )),
        )
        .add(
            "/",
            post(index).layer(AuthorizedLayer::new(
                ctx.clone(),
                "academic.campaign.reference.attend_types.index",
            )),
        )
}

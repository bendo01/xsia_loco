#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use loco_rs::prelude::*;

#[debug_handler]
pub async fn index(State(_ctx): State<AppContext>) -> Result<Response> {
    format::empty()
}

#[debug_handler]
pub async fn index_institution(
    State(_ctx): State<AppContext>,
    Path(_institution_id): Path<Uuid>,
) -> Result<Response> {
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

pub fn routes(_ctx: &AppContext) -> Routes {
    Routes::new()
        .prefix("api/institution/master/institutions")
        .add("/", get(index))
        .add("/", post(store))
        .add("/{id}", get(show))
        .add("/{id}", delete(destroy))
        .add("/{id}", post(update))
        .add("/list", get(list))
}

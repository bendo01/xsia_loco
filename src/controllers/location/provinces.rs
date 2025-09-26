#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::models::location::provinces::_entities::provinces as ReferenceModel;
use axum::{Json, debug_handler, response::Response};
use loco_openapi::prelude::*;
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect};
use serde::{Deserialize, Serialize};

#[derive(sea_orm::FromQueryResult, Clone, Debug, Serialize, Deserialize, ToSchema)]
struct SelectedData {
    id: String,
    name: String,
}

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

/// Returns data for function
///
/// # Errors
/// Returns an error if unable to retrieve model from database.
#[utoipa::path(
    get,
    path = "/api/region/provinces",
    responses(
        (status = 200, description = "Pilihan provinsi", body = [SelectedData]),
    ),
)]
pub async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    // Build and execute the query
    let datas = ReferenceModel::Entity::find()
        .filter(ReferenceModel::Column::DeletedAt.is_null())
        .order_by_asc(ReferenceModel::Column::Name)
        .select_only() // Specify that only certain columns will be selected
        .column(ReferenceModel::Column::Id) // Select the 'id' column
        .column(ReferenceModel::Column::Name) // Select the 'name' column
        .into_model::<SelectedData>() // Map results to RegencyIdName struct
        .all(&ctx.db) // Fetch all rows using the db connection
        .await?; // Await the async operation and handle errors

    // Return the results as a JSON response
    Ok(Json(datas).into_response())
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/locations/provinces")
        .add("/", get(index))
        .add("/", post(store))
        .add("/{id}", get(show))
        .add("/{id}", delete(destroy))
        .add("/{id}", post(update))
        // .add("/list", get(list))
        .add("/list", openapi(get(list), routes!(list)))
}

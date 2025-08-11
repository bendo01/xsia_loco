#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use loco_rs::prelude::*;
use axum::debug_handler;
// use chrono::Utc;
// use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder};
// use serde::{Deserialize, Serialize};
use uuid::{Uuid, uuid};

use crate::models::institution::master::units::_entities::units as InstitutionMasterUnit;

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

#[debug_handler]
pub async fn list_by_institution_course_department(
    State(ctx): State<AppContext>,
    Path(institution_id): Path<Uuid>,
) -> Result<Response> {
    let unit_type_id = uuid!("019759fd-36e8-4f43-80ed-4f687a48145d");
    let mut query = InstitutionMasterUnit::Entity::find();
    query = query.filter(InstitutionMasterUnit::Column::DeletedAt.is_null());
    let items = query
        .filter(InstitutionMasterUnit::Column::UnitTypeId.eq(unit_type_id))
        .filter(InstitutionMasterUnit::Column::IsActive.eq(true))
        .filter(InstitutionMasterUnit::Column::InstitutionId.eq(institution_id))
        .order_by_asc(InstitutionMasterUnit::Column::Code)
        .all(&ctx.db)
        .await?;
    format::json(items)
}

pub fn routes(_ctx: &AppContext) -> Routes {
    Routes::new()
        .prefix("api/institution/master/units")
        .add("/list_by_institution_course_department/{institution_id}", get(list_by_institution_course_department))
        .add("/list", get(list))
        .add("/{id}", get(show))
        .add("/{id}", delete(destroy))
        .add("/{id}", post(update))
        
        .add("/", get(index))
        .add("/", post(store))
        
}

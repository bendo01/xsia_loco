#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::middleware::is_authenticated::AuthenticatedLayer;
use crate::models::academic::student::campaign::activities::_entities::activities as ReferenceModel;
use crate::models::academic::student::campaign::activities::data_objects::DataObject as ReferenceDataObject;
use crate::services::pdf::institution_092010::student::activity::plan::activity_plan as Institution092010StudentActivityPlan;
use crate::services::pdf::institution_092010::student::activity::result::activity_result as Institution092010StudentActivityResult;
use axum::{debug_handler, extract::Path};
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder};

#[debug_handler]
pub async fn index(State(_ctx): State<AppContext>) -> Result<Response> {
    format::empty()
}

#[debug_handler]
pub async fn index_student(
    State(ctx): State<AppContext>,
    Path(student_id): Path<Uuid>,
) -> Result<Response> {
    let mut query = ReferenceModel::Entity::find();
    query = query.filter(ReferenceModel::Column::DeletedAt.is_null());
    let datas = query
        .filter(ReferenceModel::Column::StudentId.eq(student_id))
        .order_by_asc(ReferenceModel::Column::Name)
        .all(&ctx.db)
        .await?;
    let mut items = Vec::new();
    for data in datas {
        let item_object = ReferenceDataObject::get_by_id(&ctx, data.id, false).await?;
        items.push(item_object);
    }
    format::json(items)
}

#[debug_handler]
pub async fn index_unit(
    State(_ctx): State<AppContext>,
    Path(_unit_id): Path<Uuid>,
) -> Result<Response> {
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

pub async fn show_student(State(ctx): State<AppContext>, Path(id): Path<Uuid>) -> Result<Response> {
    let activity = ReferenceDataObject::get_by_id(&ctx, id, true).await?;
    format::json(activity)
}

pub async fn print_activity_plan(
    State(ctx): State<AppContext>,
    Path(activity_id): Path<Uuid>,
) -> Result<Response> {
    let pdf_data = Institution092010StudentActivityPlan::generate_pdf(&ctx, activity_id)
        .await
        .map_err(|e| Error::Message(e.to_string()))?;
    let response = Response::builder()
        .header("Content-Type", "application/pdf")
        .header("Content-Disposition", "attachment; filename=report.pdf")
        .body(pdf_data.into())
        .map_err(|e| Error::Message(e.to_string()))?;

    Ok(response)
}

pub async fn print_activity_result(
    State(ctx): State<AppContext>,
    Path(activity_id): Path<Uuid>,
) -> Result<Response> {
    let pdf_data = Institution092010StudentActivityResult::generate_pdf(&ctx, activity_id)
        .await
        .map_err(|e| Error::Message(e.to_string()))?;
    let response = Response::builder()
        .header("Content-Type", "application/pdf")
        .header("Content-Disposition", "attachment; filename=report.pdf")
        .body(pdf_data.into())
        .map_err(|e| Error::Message(e.to_string()))?;

    Ok(response)
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
        .prefix("api/academic/student/campaign/activities")
        .add("/{id}", get(show))
        .add("/{id}", delete(destroy))
        .add("/{id}", post(update))
        .add("/", get(index))
        .add("/", post(store))
        .add(
            "/index_student/{student_id}",
            get(index_student).layer(AuthenticatedLayer::new(ctx.clone())),
        )
        .add(
            "/print_activity_plan/{activity_id}",
            get(print_activity_plan),
        )
        .add(
            "/print_activity_result/{activity_id}",
            get(print_activity_result),
        )
        .add(
            "/show_student/{id}",
            get(show_student).layer(AuthenticatedLayer::new(ctx.clone())),
        )
}

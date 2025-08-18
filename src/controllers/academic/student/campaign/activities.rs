#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::middleware::is_authenticated::AuthenticatedLayer;
use crate::models::academic::student::campaign::activities::_entities::activities as ReferenceModel;
use crate::models::academic::student::campaign::activities::_entities::activities as AcademicStudentCampaignActivity;
use crate::models::academic::student::campaign::activities::data_objects::DataObject as ReferenceDataObject;
use crate::models::academic::student::master::students::_entities::students as AcademicStudentMasterStudent;
use crate::models::auth::users::_entities::users as AuthUser;
use crate::models::institution::master::units::_entities::units as InstitutionMasterUnit;
use crate::services::pdf::institution_092010::student::activity::plan::activity_plan as Institution092010StudentActivityPlan;
use crate::services::pdf::institution_092010::student::activity::result::activity_result as Institution092010StudentActivityResult;
use crate::vendor::paginate::pagination::{PaginateInput, PaginateResult};
// use crate::vendor::validation::common::format_validation_errors;
use axum::extract::Extension;
use axum::{Json, debug_handler, extract::Path};
use chrono::Utc;
use loco_rs::prelude::*;
use sea_orm::sea_query::Expr; // Import Expr to build expressions
use sea_orm::{ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};
use sea_query::extension::postgres::PgExpr;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ModelPagination {
    pagination: PaginateResult,
    data: Vec<Option<ReferenceDataObject>>,
}

#[derive(Deserialize, Serialize)]
pub struct StatusUpdateInput {
    id: Uuid,
    status_id: Uuid,
}

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
pub async fn update_status(
    State(ctx): State<AppContext>,
    Extension(user): Extension<AuthUser::Model>,
    Json(input): Json<StatusUpdateInput>,
) -> Result<Response> {
    let activity_opt = AcademicStudentCampaignActivity::Entity::find_by_id(input.id)
        .filter(AcademicStudentCampaignActivity::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await?;

    if let Some(activity) = activity_opt {
        // Create an active model from the existing item
        let mut item = activity.clone().into_active_model();
        let now = Utc::now().naive_utc();
        item.status_id = Set(input.status_id);
        item.updated_at = Set(Some(now));
        item.updated_by = Set(Some(user.id));
        let updated_item = item.update(&ctx.db).await?;
        format::json(updated_item)
    } else {
        // Return an error response when activity is not found
        Err(Error::NotFound)
    }
}

#[debug_handler]
pub async fn index_institution(
    State(ctx): State<AppContext>,
    Path(institution_id): Path<Uuid>,
    Json(paginate_input): Json<PaginateInput>,
) -> Result<Response> {
    let unit_type_id: Uuid = uuid::uuid!("019759fd-36e8-4f43-80ed-4f687a48145d");
    let unit_opt = InstitutionMasterUnit::Entity::find()
        .filter(InstitutionMasterUnit::Column::DeletedAt.is_null())
        .filter(InstitutionMasterUnit::Column::InstitutionId.eq(institution_id))
        .filter(InstitutionMasterUnit::Column::UnitTypeId.eq(unit_type_id))
        .all(&ctx.db)
        .await?;

    // Extract unit IDs from the found units
    let unit_ids: Vec<Uuid> = unit_opt.iter().map(|unit| unit.id).collect();

    // If no units found, return empty response
    if unit_ids.is_empty() {
        return format::json(Vec::<AcademicStudentCampaignActivity::Model>::new());
    }

    // Query students that belong to these units
    let students_opt = AcademicStudentMasterStudent::Entity::find()
        .filter(AcademicStudentMasterStudent::Column::DeletedAt.is_null())
        .filter(AcademicStudentMasterStudent::Column::UnitId.is_in(unit_ids))
        .all(&ctx.db)
        .await?;
    let student_ids: Vec<Uuid> = students_opt.iter().map(|student| student.id).collect();

    if student_ids.is_empty() {
        return format::json(Vec::<AcademicStudentCampaignActivity::Model>::new());
    }

    // start pagination

    let mut query = AcademicStudentCampaignActivity::Entity::find();
    query = query
        .filter(AcademicStudentCampaignActivity::Column::DeletedAt.is_null())
        .filter(AcademicStudentCampaignActivity::Column::StudentId.is_in(student_ids));

    // Apply search filters
    if let Some(search) = &paginate_input.search {
        let search_pattern = format!("%{search}%");
        query = query.filter(Condition::any().add(
            Expr::col(AcademicStudentCampaignActivity::Column::Name).ilike(search_pattern.clone()),
        ));
    }

    // Apply sorting
    if let Some(sort_by) = &paginate_input.sort_by {
        match (sort_by.as_str(), paginate_input.sort_dir.as_deref()) {
            ("name", Some("asc")) => {
                query = query.order_by_asc(AcademicStudentCampaignActivity::Column::Name);
            }
            ("name", Some("desc")) => {
                query = query.order_by_desc(AcademicStudentCampaignActivity::Column::Name);
            }
            _ => query = query.order_by_asc(AcademicStudentCampaignActivity::Column::Name),
        }
    }

    // Pagination logic
    let page = paginate_input.page;
    let per_page = paginate_input.per_page;

    let paginator = query.paginate(&ctx.db, per_page);
    let total_data = paginator.num_items().await?;
    let total_page = paginator.num_pages().await?;
    // let items = paginator.fetch_page(page - 1).await?;
    let datas = paginator.fetch_page(page - 1).await?;

    let mut items = Vec::new();
    for data in datas {
        let item_object = ReferenceDataObject::get_by_id(&ctx, data.id, false).await?;
        items.push(item_object);
    }

    // Create the pagination metadata
    let result = PaginateResult {
        search: paginate_input.search,
        sort_by: paginate_input.sort_by,
        column: paginate_input.column,
        sort_dir: paginate_input.sort_dir,
        page,
        per_page,
        total_page,
        last_page: total_page,
        total_data,
    };

    // Combine the data and pagination result
    let response = ModelPagination {
        pagination: result,
        data: items,
    };

    // Return the response as JSON
    format::json(response)
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
pub async fn show_student(State(ctx): State<AppContext>, Path(id): Path<Uuid>) -> Result<Response> {
    let activity = ReferenceDataObject::get_by_id(&ctx, id, true).await?;
    format::json(activity)
}

#[debug_handler]
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

#[debug_handler]
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
            "/index_institution/{institution_id}",
            post(index_institution).layer(AuthenticatedLayer::new(ctx.clone())),
        )
        .add(
            "/show_student/{id}",
            get(show_student).layer(AuthenticatedLayer::new(ctx.clone())),
        )
        .add(
            "/update_status",
            post(update_status).layer(AuthenticatedLayer::new(ctx.clone())),
        )
}

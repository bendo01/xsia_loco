use crate::middleware::is_authorized::AuthorizedLayer;
use crate::models::auth::roles::_entities::roles as ReferenceModel;
use crate::models::auth::roles::data_objects::DataObject as ReferenceDataObject;
use crate::models::auth::roles::validation::roles::{ModelValidation, ModelValidator};
use crate::models::auth::users::_entities::users as AuthUser;
use crate::vendor::paginate::pagination::{PaginateInput, PaginateResult};
use crate::vendor::validation::common::format_validation_errors;
use axum::{Json, debug_handler, extract::Path, http::StatusCode};
use chrono::Utc;
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::ValidationErrors;

#[derive(Deserialize, Serialize)]
pub struct ModelPagination {
    pagination: PaginateResult,
    data: Vec<ReferenceModel::Model>,
}

/// Returns data for function
///
/// # Errors
/// Returns an error if unable to retrieve model from database.
#[debug_handler]
pub async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    // format::json(Entity::find().all(&ctx.db).await?)
    let mut query = ReferenceModel::Entity::find();
    query = query.filter(ReferenceModel::Column::DeletedAt.is_null());
    let items = query
        .order_by_asc(ReferenceModel::Column::Name)
        .all(&ctx.db)
        .await?;
    format::json(items)
}

/// Returns data for function
///
/// # Errors
/// Returns an error if unable to retrieve model from database.
async fn load_item(ctx: &AppContext, id: Uuid) -> Result<ReferenceDataObject> {
    let item = ReferenceDataObject::get_by_id(ctx, id).await?;
    item.ok_or_else(|| Error::NotFound)
}

/// Returns data for function
///
/// # Errors
/// Returns an error if unable to retrieve model from database.
#[allow(clippy::match_same_arms)]
#[debug_handler]
pub async fn index(
    State(ctx): State<AppContext>,
    Json(paginate_input): Json<PaginateInput>,
) -> Result<Response> {
    let mut query =
        ReferenceModel::Entity::find().filter(ReferenceModel::Column::DeletedAt.is_null());
    if let Some(search) = &paginate_input.search {
        if let Some(column) = &paginate_input.column {
            match column.as_str() {
                "name" => query = query.filter(ReferenceModel::Column::Name.contains(search)),
                _ => query = query.filter(ReferenceModel::Column::Name.contains(search)),
            }
        }
    }

    // Apply sorting if `sort_by` and `sort_dir` are provided
    if let Some(sort_by) = &paginate_input.sort_by {
        match (sort_by.as_str(), paginate_input.sort_dir.as_deref()) {
            ("name", Some("asc")) => query = query.order_by_asc(ReferenceModel::Column::Name),
            ("name", Some("desc")) => query = query.order_by_desc(ReferenceModel::Column::Name),
            _ => query = query.order_by_asc(ReferenceModel::Column::Name),
        }
    }
    // Pagination logic
    let page = paginate_input.page;
    let per_page = paginate_input.per_page;

    let paginator = query.paginate(&ctx.db, per_page); // Use SeaORM's built-in paginator
    let total_data = paginator.num_items().await?; // Get total number of items
    let total_page = paginator.num_pages().await?; // Get total number of pages
    let items = paginator.fetch_page(page - 1).await?; // Fetch the items for the current page (0-indexed)

    // Create the pagination result
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

    // Respond with paginated results and the pagination metadata
    // Combine pagination result and the data
    let response = ModelPagination {
        pagination: result,
        data: items,
    };

    // Return the response as JSON
    format::json(response)
}

/// Returns data for function
///
/// # Errors
/// Returns an error if unable to retrieve model from database.
#[debug_handler]
pub async fn store(
    State(ctx): State<AppContext>,
    JsonValidateWithMessage(payload): JsonValidateWithMessage<ModelValidator>,
) -> Result<Response> {
    let mut validation_errors = ValidationErrors::new();
    if let Err(validation_errors) = payload.validate() {
        let error_json = format_validation_errors(&validation_errors, "Validation Failed");
        return Ok((
            StatusCode::UNPROCESSABLE_ENTITY, // Set status to 422
            Json(error_json),                 // Return JSON-formatted errors
        )
            .into_response());
    }
    if let Err(e) = payload.validate_unique_name(&ctx.db, None).await {
        validation_errors.add("name", e);
    }

    if !validation_errors.is_empty() {
        let error_json = format_validation_errors(&validation_errors, "Validation Failed");
        return Ok((StatusCode::UNPROCESSABLE_ENTITY, Json(error_json)).into_response());
    }

    // Create a new active model
    let now = Utc::now().naive_utc();
    let id = Uuid::new_v4();

    let model = ReferenceModel::ActiveModel {
        id: Set(id),
        name: Set(payload.name),
        user_id: Set(payload.user_id),
        position_type_id: Set(payload.position_type_id),
        roleable_id: Set(payload.roleable_id),
        roleable_type: Set(payload.roleable_type),
        created_at: Set(Some(now)),
        updated_at: Set(Some(now)),
        sync_at: Set(None),
        deleted_at: Set(None),
        created_by: Set(None), // TODO: Set with authenticated user ID if available
        updated_by: Set(None),
    };

    // Insert the new model into the database
    let result = model.insert(&ctx.db).await?;

    // Return the created model
    format::json(result)
}

// #[debug_handler]
// pub async fn show(Path(id): Path<Uuid>, State(ctx): State<AppContext>) -> Result<Response> {
//     let data_object = load_item(&ctx, id).await?;
//     // Ok(format::json(data_object))
//     format::json(data_object)
// }

#[debug_handler]
/// Returns data for function
///
/// # Errors
/// Returns an error if unable to retrieve model from database.
pub async fn destroy(Path(id): Path<Uuid>, State(ctx): State<AppContext>) -> Result<Response> {
    let data_object = load_item(&ctx, id).await?;
    let mut item = data_object.model.into_active_model();
    let now = Utc::now().naive_utc();
    item.deleted_at = Set(Some(now));
    item.update(&ctx.db).await?;
    format::empty()
}

/// Returns data for function
///
/// # Errors
/// Returns an error if unable to retrieve model from database.
#[debug_handler]
pub async fn update(
    Path(id): Path<Uuid>,
    State(ctx): State<AppContext>,
    JsonValidateWithMessage(payload): JsonValidateWithMessage<ModelValidator>,
) -> Result<Response> {
    let data_object = load_item(&ctx, id).await?;
    let data = &data_object.model;
    let mut validation_errors = ValidationErrors::new();
    if let Err(validation_errors) = payload.validate() {
        let error_json = format_validation_errors(&validation_errors, "Validation Failed");
        return Ok((
            StatusCode::UNPROCESSABLE_ENTITY, // Set status to 422
            Json(error_json),                 // Return JSON-formatted errors
        )
            .into_response());
    }
    if let Err(e) = payload.validate_unique_name(&ctx.db, Some(id)).await {
        validation_errors.add("name", e);
    }

    if !validation_errors.is_empty() {
        let error_json = format_validation_errors(&validation_errors, "Validation Failed");
        return Ok((StatusCode::UNPROCESSABLE_ENTITY, Json(error_json)).into_response());
    }

    // Create an active model from the existing item
    let mut item = data.clone().into_active_model();

    // Update the fields with new values
    let now = Utc::now().naive_utc();
    item.name = Set(payload.name);
    item.user_id = Set(payload.user_id);
    item.position_type_id = Set(payload.position_type_id);
    item.roleable_id = Set(payload.roleable_id);
    item.roleable_type = Set(payload.roleable_type);
    item.updated_at = Set(Some(now));
    item.updated_by = Set(None); // TODO: Set with authenticated user ID if available

    // Save the updated model
    let updated_item = item.update(&ctx.db).await?;

    // Return the updated model
    format::json(updated_item)
}

/// Returns data for function
///
/// # Errors
/// Returns an error if unable to retrieve model from database.
pub async fn user_roles(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    // Fetch the user by PID from the JWT claims
    let user = AuthUser::Model::find_by_pid(&ctx.db, &auth.claims.pid)
        .await
        .map_err(Error::from)?;

    // Fetch all active roles for the user
    let roles = ReferenceModel::Entity::find()
        .filter(ReferenceModel::Column::UserId.eq(user.id))
        .filter(ReferenceModel::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(Error::from)?;

    // Return the roles as a JSON response
    Ok(Json(roles).into_response())
}

pub fn routes(ctx: &AppContext) -> Routes {
    Routes::new()
        .prefix("api/auth/roles")
        .add("/list", get(list))
        .add("/user", get(user_roles))
        .add(
            "/store",
            post(store).layer(AuthorizedLayer::new(ctx.clone(), "auth.roles.index")),
        )
        // .add(
        //     "/{id}",
        //     get(show).layer(AuthorizedLayer::new(ctx.clone(), "auth.roles.index")),
        // )
        .add(
            "/{id}",
            delete(destroy).layer(AuthorizedLayer::new(ctx.clone(), "auth.roles.index")),
        )
        .add(
            "/{id}",
            put(update).layer(AuthorizedLayer::new(ctx.clone(), "auth.roles.index")),
        )
        .add(
            "/",
            post(index).layer(AuthorizedLayer::new(ctx.clone(), "auth.roles.index")),
        )
}

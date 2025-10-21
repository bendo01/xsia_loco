use crate::middleware::is_authorized::AuthorizedLayer;
use crate::models::academic::course::reference::groups::_entities::groups as ReferenceModel;
use crate::models::academic::course::reference::groups::data_objects::DataObject as ReferenceDataObject;
use crate::models::academic::course::reference::groups::validation::groups::{
    ModelValidation, ModelValidator,
};
use crate::vendor::paginate::pagination::{PaginateInput, PaginateResult};

use axum::{Json, debug_handler, extract::Path};
use chrono::Utc;
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
        .order_by_asc(ReferenceModel::Column::Code)
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
                "code" => {
                    let code: Result<i32, _> = search.clone().parse();
                    match code {
                        Ok(code) => {
                            query = query.filter(ReferenceModel::Column::Code.eq(code));
                        }
                        Err(e) => println!("Failed to parse: {e}"),
                    }
                }
                "alphabet_code" => {
                    query = query.order_by_asc(ReferenceModel::Column::AlphabetCode);
                }
                "name" => query = query.filter(ReferenceModel::Column::Name.contains(search)),
                _ => query = query.filter(ReferenceModel::Column::Name.contains(search)),
            }
        }
    }

    // Apply sorting if `sort_by` and `sort_dir` are provided
    if let Some(sort_by) = &paginate_input.sort_by {
        match (sort_by.as_str(), paginate_input.sort_dir.as_deref()) {
            ("code", Some("asc")) => query = query.order_by_asc(ReferenceModel::Column::Code),
            ("code", Some("desc")) => query = query.order_by_desc(ReferenceModel::Column::Code),
            ("alphabet_code", Some("asc")) => {
                query = query.order_by_asc(ReferenceModel::Column::AlphabetCode);
            }
            ("alphabet_code", Some("desc")) => {
                query = query.order_by_desc(ReferenceModel::Column::AlphabetCode);
            }
            ("name", Some("asc")) => query = query.order_by_asc(ReferenceModel::Column::Name),
            ("name", Some("desc")) => query = query.order_by_desc(ReferenceModel::Column::Name),
            _ => query = query.order_by_asc(ReferenceModel::Column::Code),
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
    Json(payload): Json<ModelValidator>,
) -> Result<Response> {
    // Additional validation for uniqueness
    payload.validate_unique_code(&ctx.db, None).await?;
    payload.validate_unique_alphabet_code(&ctx.db, None).await?;

    // Create a new active model
    let now = Utc::now().naive_utc();
    let id = Uuid::new_v4();

    let model = ReferenceModel::ActiveModel {
        id: Set(id),
        code: Set(payload.code),
        alphabet_code: Set(payload.alphabet_code),
        name: Set(payload.name),
        abbreviation: Set(payload.abbreviation),
        start_effective_date: Set(payload.start_effective_date),
        end_effective_date: Set(payload.end_effective_date),
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

#[debug_handler]
/// Returns data for function
///
/// # Errors
/// Returns an error if unable to retrieve model from database.
pub async fn show(State(_ctx): State<AppContext>) -> Result<Response> {
    format::empty()
}

/// Returns data for function
///
/// # Errors
/// Returns an error if unable to retrieve model from database.
#[debug_handler]
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
    Json(payload): Json<ModelValidator>,
) -> Result<Response> {
    // Find the existing item
    let data_object = load_item(&ctx, id).await?;
    let data = &data_object.model;

    // Check if code changed and validate uniqueness if changed
    if data.code != payload.code {
        payload.validate_unique_code(&ctx.db, Some(id)).await?;
    }

    // Check if alphabet_code changed and validate uniqueness if changed
    if data.alphabet_code != payload.alphabet_code {
        payload
            .validate_unique_alphabet_code(&ctx.db, Some(id))
            .await?;
    }

    // Create an active model from the existing item
    let mut item = data.clone().into_active_model();

    // Update the fields with new values
    let now = Utc::now().naive_utc();
    item.code = Set(payload.code);
    item.alphabet_code = Set(payload.alphabet_code);
    item.name = Set(payload.name);
    item.updated_at = Set(Some(now));
    item.updated_by = Set(None); // TODO: Set with authenticated user ID if available

    // Save the updated model
    let updated_item = item.update(&ctx.db).await?;

    // Return the updated model
    format::json(updated_item)
}

pub fn routes(ctx: &AppContext) -> Routes {
    Routes::new()
        .prefix("api/academic/course/reference/groups")
        .add("/list", get(list))
        .add(
            "/store",
            post(store).layer(AuthorizedLayer::new(
                ctx.clone(),
                "academic.course.reference.groups.index",
            )),
        )
        .add(
            "/{id}",
            get(show).layer(AuthorizedLayer::new(
                ctx.clone(),
                "academic.course.reference.groups.index",
            )),
        )
        .add(
            "/{id}",
            delete(destroy).layer(AuthorizedLayer::new(
                ctx.clone(),
                "academic.course.reference.groups.index",
            )),
        )
        .add(
            "/{id}",
            put(update).layer(AuthorizedLayer::new(
                ctx.clone(),
                "academic.course.reference.groups.index",
            )),
        )
        .add(
            "/",
            post(index).layer(AuthorizedLayer::new(
                ctx.clone(),
                "academic.course.reference.groups.index",
            )),
        )
}

use crate::models::institution::master::employees::_entities::employees as InstitutionMasterEmployee;
use crate::models::institution::master::employees::data_objects::EmployeeDataObject;
use crate::vendor::paginate::pagination::{PaginateInput, PaginateResult};
use axum::{Json, debug_handler};
use loco_rs::prelude::*;
use sea_orm::sea_query::Expr; // Import Expr to build expressions
use sea_orm::{ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};
use sea_query::extension::postgres::PgExpr;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ModelPagination {
    pagination: PaginateResult,
    data: Vec<Option<EmployeeDataObject>>,
}

/// Returns data for function
///
/// # Errors
/// Returns an error if unable to retrieve model from database.
#[allow(clippy::match_same_arms)]
#[allow(clippy::redundant_clone)]
#[debug_handler]
/// Returns data for function
///
/// # Errors
/// Returns an error if unable to retrieve model from database.
pub async fn index(
    State(ctx): State<AppContext>,
    Json(paginate_input): Json<PaginateInput>,
) -> Result<Response> {
    let mut query = InstitutionMasterEmployee::Entity::find();
    query = query.filter(InstitutionMasterEmployee::Column::DeletedAt.is_null());

    // Apply search filters
    if let Some(search) = &paginate_input.search {
        let search_pattern = format!("%{search}%");
        query = query.filter(
            Condition::any()
                .add(
                    Expr::col(InstitutionMasterEmployee::Column::Code)
                        .ilike(search_pattern.clone()),
                )
                .add(
                    Expr::col(InstitutionMasterEmployee::Column::Name)
                        .ilike(search_pattern.clone()),
                ),
        );
    }

    // Apply sorting
    if let Some(sort_by) = &paginate_input.sort_by {
        match (sort_by.as_str(), paginate_input.sort_dir.as_deref()) {
            ("code", Some("asc")) => {
                query = query.order_by_asc(InstitutionMasterEmployee::Column::Code);
            }
            ("code", Some("desc")) => {
                query = query.order_by_desc(InstitutionMasterEmployee::Column::Code);
            }
            ("name", Some("asc")) => {
                query = query.order_by_asc(InstitutionMasterEmployee::Column::Name);
            }
            ("name", Some("desc")) => {
                query = query.order_by_desc(InstitutionMasterEmployee::Column::Name);
            }
            _ => query = query.order_by_asc(InstitutionMasterEmployee::Column::Code),
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
        let item_object = EmployeeDataObject::get_by_id(&ctx, data.id).await?;
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
/// Returns data for function
///
/// # Errors
/// Returns an error if unable to retrieve model from database.
pub async fn store(State(_ctx): State<AppContext>) -> Result<Response> {
    format::empty()
}

#[debug_handler]
/// Returns data for function
///
/// # Errors
/// Returns an error if unable to retrieve model from database.
pub async fn show(Path(id): Path<Uuid>, State(ctx): State<AppContext>) -> Result<Response> {
    //format::empty()
    let employee = EmployeeDataObject::get_by_id(&ctx, id).await?;
    #[allow(clippy::option_if_let_else)]
    match employee {
        Some(data) => format::json(data),
        None => Err(Error::NotFound),
    }
}

#[debug_handler]
/// Returns data for function
///
/// # Errors
/// Returns an error if unable to retrieve model from database.
pub async fn destroy(State(_ctx): State<AppContext>) -> Result<Response> {
    format::empty()
}

#[debug_handler]
/// Returns data for function
///
/// # Errors
/// Returns an error if unable to retrieve model from database.
pub async fn update(State(_ctx): State<AppContext>) -> Result<Response> {
    format::empty()
}

#[debug_handler]
/// Returns data for function
///
/// # Errors
/// Returns an error if unable to retrieve model from database.
pub async fn list(State(_ctx): State<AppContext>) -> Result<Response> {
    format::empty()
}

pub fn routes(_ctx: &AppContext) -> Routes {
    Routes::new()
        .prefix("api/institution/master/employees")
        .add("/", get(index))
        .add("/", post(store))
        .add("/{id}", get(show))
        .add("/{id}", delete(destroy))
        .add("/{id}", post(update))
        .add("/list", get(list))
}

use crate::middleware::is_authorized::AuthorizedLayer;
use crate::models::person::reference::age_classifications::_entities::age_classifications as ReferenceModel;
use crate::models::person::reference::age_classifications::validation::age_classifications as ReferenceValidation;
use crate::vendor::paginate::pagination::{PaginateInput, PaginateResult};
use crate::vendor::validation::common::format_validation_errors;
use axum::{Json, debug_handler, http::StatusCode};
use chrono::Utc;
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct Params {
    #[validate(range(min = 1))]
    pub code: i32,
    #[validate(length(min = 1, message = "Name must be at least 1 characters long."))]
    pub alphabet_code: String,
    #[validate(length(min = 1, message = "Name must be at least 1 characters long."))]
    pub name: String,
    #[validate(range(min = 1))]
    pub minimum: i32,
    #[validate(range(min = 1))]
    pub maximum: Option<i32>,
}

impl Params {
    fn update(&self, item: &mut ReferenceModel::ActiveModel) {
        item.code = Set(self.code);
        item.alphabet_code = Set(self.alphabet_code.clone());
        item.name = Set(self.name.clone());
        item.minimum = Set(self.minimum);
        if self.maximum.is_some() {
            item.maximum = Set(self.maximum);
        }
    }
}

impl Validatable for ReferenceModel::ActiveModel {
    fn validator(&self) -> Box<dyn Validate> {
        Box::new(Params {
            code: self.code.as_ref().to_owned(),
            alphabet_code: self.alphabet_code.as_ref().to_owned(),
            name: self.name.as_ref().to_owned(),
            minimum: self.minimum.as_ref().to_owned(),
            maximum: self.maximum.as_ref().to_owned(),
        })
    }
}

#[derive(Deserialize, Serialize)]
pub struct ModelPagination {
    pagination: PaginateResult,
    data: Vec<ReferenceModel::Model>,
}

/// Returns data for function
///
/// # Errors
/// Returns an error if unable to retrieve model from database.
async fn load_item(ctx: &AppContext, id: Uuid) -> Result<ReferenceModel::Model> {
    let item = ReferenceModel::Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
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
#[allow(clippy::match_same_arms)]
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
pub async fn store(
    State(ctx): State<AppContext>,
    JsonValidateWithMessage(params): JsonValidateWithMessage<Params>,
) -> Result<Response> {
    if let Err(validation_errors) = validator::Validate::validate(&params) {
        // Use your format_validation_errors function to format the errors with a custom message
        let error_json = format_validation_errors(&validation_errors, "Validation Failed");
        // Return the error as a response with a 422 Unprocessable Entity status code
        return Ok((
            StatusCode::UNPROCESSABLE_ENTITY, // Set status to 422
            Json(error_json),                 // Return JSON-formatted errors
        )
            .into_response());
    }

    let mut validation_errors = ValidationErrors::new();

    // Check uniqueness for code, alphabet_code, and name
    if let Err(e) = ReferenceValidation::validate_unique_code(&params.code, &ctx.db, None).await {
        validation_errors.add("code", e);
    }
    if let Err(e) =
        ReferenceValidation::validate_unique_alphabet_code(&params.alphabet_code, &ctx.db, None)
            .await
    {
        validation_errors.add("alphabet_code", e);
    }
    if let Err(e) = ReferenceValidation::validate_unique_name(&params.name, &ctx.db, None).await {
        validation_errors.add("name", e);
    }

    if let Err(e) = ReferenceValidation::validate_unique_name(&params.name, &ctx.db, None).await {
        validation_errors.add("name", e);
    }

    if let Err(e) = ReferenceValidation::validate_minimum(&params.minimum) {
        validation_errors.add("minimum", e);
    }

    if let Err(e) = ReferenceValidation::validate_min_max(&params.minimum, &params.maximum) {
        validation_errors.add("minimum", e);
    }

    if !validation_errors.is_empty() {
        let error_json = format_validation_errors(&validation_errors, "Validation Failed");
        return Ok((StatusCode::UNPROCESSABLE_ENTITY, Json(error_json)).into_response());
    }

    let mut item = <ReferenceModel::ActiveModel as sea_orm::ActiveModelTrait>::default();
    params.update(&mut item);
    let item = item.insert(&ctx.db).await?;
    format::json(item)
}

/// Returns data for function
///
/// # Errors
/// Returns an error if unable to retrieve model from database.
pub async fn update(
    Path(id): Path<Uuid>,
    State(ctx): State<AppContext>,
    JsonValidateWithMessage(params): JsonValidateWithMessage<Params>,
) -> Result<Response> {
    let mut validation_errors = ValidationErrors::new();

    // Check uniqueness for code, alphabet_code, and name
    if let Err(e) = ReferenceValidation::validate_unique_code(&params.code, &ctx.db, Some(id)).await
    {
        validation_errors.add("code", e);
    }
    if let Err(e) =
        ReferenceValidation::validate_unique_alphabet_code(&params.alphabet_code, &ctx.db, Some(id))
            .await
    {
        validation_errors.add("alphabet_code", e);
    }
    if let Err(e) = ReferenceValidation::validate_unique_name(&params.name, &ctx.db, Some(id)).await
    {
        validation_errors.add("name", e);
    }

    if let Err(e) = ReferenceValidation::validate_minimum(&params.minimum) {
        validation_errors.add("minimum", e);
    }

    if let Err(e) = ReferenceValidation::validate_min_max(&params.minimum, &params.maximum) {
        validation_errors.add("minimum", e);
    }

    if !validation_errors.is_empty() {
        let error_json = format_validation_errors(&validation_errors, "Validation Failed");
        return Ok((StatusCode::UNPROCESSABLE_ENTITY, Json(error_json)).into_response());
    }

    let item = load_item(&ctx, id).await?;
    let mut item = item.into_active_model();
    params.update(&mut item);
    let item = item.update(&ctx.db).await?;
    format::json(item)
}

/// Returns data for function
///
/// # Errors
/// Returns an error if unable to retrieve model from database.
pub async fn destroy(Path(id): Path<Uuid>, State(ctx): State<AppContext>) -> Result<Response> {
    let item = load_item(&ctx, id).await?;
    let mut item = item.into_active_model();
    let now = Utc::now().naive_utc();
    item.deleted_at = Set(Some(now));
    item.update(&ctx.db).await?;
    format::empty()
}

// pub async fn show(Path(id): Path<Uuid>, State(ctx): State<AppContext>) -> Result<Response> {
//     format::json(load_item(&ctx, id).await?)
// }

pub fn routes(ctx: &AppContext) -> Routes {
    Routes::new()
        .prefix("api/person/reference/age_classifications")
        //.add("/", get(index))
        .add("/list", get(list))
        .add(
            "/store",
            post(store).layer(AuthorizedLayer::new(
                ctx.clone(),
                "person.reference.age_classifications.index",
            )),
        )
        // .add(
        //     "/{id}",
        //     get(show).layer(AuthorizedLayer::new(
        //         ctx.clone(),
        //         "person.reference.age_classifications.index",
        //     )),
        // )
        .add(
            "/{id}",
            delete(destroy).layer(AuthorizedLayer::new(
                ctx.clone(),
                "person.reference.age_classifications.index",
            )),
        )
        .add(
            "/{id}",
            put(update).layer(AuthorizedLayer::new(
                ctx.clone(),
                "person.reference.age_classifications.index",
            )),
        )
        .add(
            "/",
            post(index).layer(AuthorizedLayer::new(
                ctx.clone(),
                "person.reference.age_classifications.index",
            )),
        )
}

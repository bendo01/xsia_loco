use crate::middleware::is_authenticated::AuthenticatedLayer;
use crate::middleware::is_authorized::AuthorizedLayer;
use crate::models::academic::candidate::master::candidates::_entities::candidates as AcademicCandidateMasterCandidate;
use crate::models::academic::student::master::students::_entities::students as AcademicStudentMasterStudent;
use crate::models::auth::users::_entities::users as AuthUser;
use crate::models::person::master::family_card_members::_entities::family_card_members as PersonMasterFamilyCardMember;
use crate::models::person::master::family_cards::_entities::family_cards as PersonMasterFamilyCard;
use crate::models::person::master::individuals::_entities::individuals as ReferenceModel;
use crate::models::person::master::individuals::data_objects::DataObject as ReferenceDataObject;
use crate::models::person::master::individuals::validation::individuals::{
    ModelValidation, ModelValidator,
};
use crate::vendor::paginate::pagination::{PaginateInput, PaginateResult};
use crate::vendor::validation::common::format_validation_errors;
use axum::{Json, extract::Path, http::StatusCode};
use chrono::Utc;
use loco_rs::prelude::*;
use sea_orm::sea_query::Expr; // Import Expr to build expressions
use sea_orm::{ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};
use sea_query::extension::postgres::PgExpr;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;
use validator::ValidationErrors;

#[derive(Deserialize, Serialize)]
pub struct ModelPagination {
    pagination: PaginateResult,
    data: Vec<Option<ReferenceDataObject>>,
}

/// Returns data for function
///
/// # Errors
/// Returns an error if unable to retrieve model from database.
#[allow(clippy::match_same_arms)]
#[allow(clippy::redundant_clone)]
pub async fn index(
    State(ctx): State<AppContext>,
    Json(paginate_input): Json<PaginateInput>,
) -> Result<Response> {
    let mut query = ReferenceModel::Entity::find();
    query = query.filter(ReferenceModel::Column::DeletedAt.is_null());

    // Apply search filters
    if let Some(search) = &paginate_input.search {
        let search_pattern = format!("%{search}%");
        query = query.filter(
            Condition::any()
                .add(Expr::col(ReferenceModel::Column::Code).ilike(search_pattern.clone()))
                .add(Expr::col(ReferenceModel::Column::Name).ilike(search_pattern.clone())),
        );
    }

    // Apply sorting
    if let Some(sort_by) = &paginate_input.sort_by {
        match (sort_by.as_str(), paginate_input.sort_dir.as_deref()) {
            ("code", Some("asc")) => {
                query = query.order_by_asc(ReferenceModel::Column::Code);
            }
            ("code", Some("desc")) => {
                query = query.order_by_desc(ReferenceModel::Column::Code);
            }
            ("name", Some("asc")) => {
                query = query.order_by_asc(ReferenceModel::Column::Name);
            }
            ("name", Some("desc")) => {
                query = query.order_by_desc(ReferenceModel::Column::Name);
            }
            _ => query = query.order_by_asc(ReferenceModel::Column::Code),
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

/// Returns data for function
///
/// # Errors
/// Returns an error if unable to retrieve model from database.
pub async fn show(Path(id): Path<Uuid>, State(ctx): State<AppContext>) -> Result<Response> {
    let individual = ReferenceDataObject::get_by_id(&ctx, id, true).await?;
    #[allow(clippy::option_if_let_else)]
    match individual {
        Some(data) => format::json(data),
        None => Err(Error::NotFound),
    }
}

/// Returns data for function
///
/// # Errors
/// Returns an error if unable to retrieve model from database.
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

    if let Err(e) = payload.validate_unique_code(&ctx.db, None).await {
        validation_errors.add("code", e);
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
        code: Set(payload.code),
        name: Set(payload.name),
        birth_place: Set(payload.birth_place),
        birth_date: Set(payload.birth_date),
        gender_id: Set(payload.gender_id),
        religion_id: Set(payload.religion_id),
        identification_type_id: Set(payload.identification_type_id),
        marital_status_id: Set(payload.marital_status_id),
        profession_id: Set(payload.profession_id),
        is_special_need: Set(payload.is_special_need),
        is_social_protection_card_recipient: Set(payload.is_social_protection_card_recipient),
        is_deceased: Set(payload.is_deceased),
        occupation_id: Set(payload.occupation_id),
        education_id: Set(payload.education_id),
        income_id: Set(payload.income_id),
        front_title: Set(payload.front_title.clone()),
        last_title: Set(payload.last_title.clone()),
        age_classification_id: Set(None),
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

#[allow(clippy::too_many_lines)]
pub async fn store_candidate(
    Path(candidate_id): Path<Uuid>,
    State(ctx): State<AppContext>,
    JsonValidateWithMessage(payload): JsonValidateWithMessage<ModelValidator>,
) -> Response {
    // Fetch candidate from database
    let candidate_opt = match AcademicCandidateMasterCandidate::Entity::find()
        .filter(AcademicCandidateMasterCandidate::Column::DeletedAt.is_null())
        .filter(AcademicCandidateMasterCandidate::Column::Id.eq(candidate_id))
        .one(&ctx.db)
        .await
    {
        Ok(opt) => opt,
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": err.to_string()})),
            )
                .into_response();
        }
    };

    let Some(current_candidate) = candidate_opt else {
        return (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Candidate not found"})),
        )
            .into_response();
    };

    // Fetch associated user
    let user_opt = match AuthUser::Entity::find()
        .filter(AuthUser::Column::DeletedAt.is_null())
        .filter(AuthUser::Column::Id.eq(current_candidate.user_id))
        .one(&ctx.db)
        .await
    {
        Ok(opt) => opt,
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": err.to_string()})),
            )
                .into_response();
        }
    };

    let Some(current_user) = user_opt else {
        return (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Candidate not found"})),
        )
            .into_response();
    };

    // Business logic checks
    if current_user.individual_id != Uuid::nil() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Individual ID already set"})),
        )
            .into_response();
    }

    // Validate payload
    if let Err(errors) = payload.validate() {
        let error_json = format_validation_errors(&errors, "Validation Failed"); // Assume this function exists
        return (StatusCode::UNPROCESSABLE_ENTITY, Json(error_json)).into_response();
    }

    if let Err(e) = payload.validate_unique_code(&ctx.db, None).await {
        let mut validation_errors = ValidationErrors::new(); // Assume ValidationErrors type exists
        validation_errors.add("code", e);
        let error_json = format_validation_errors(&validation_errors, "Validation Failed");
        return (StatusCode::UNPROCESSABLE_ENTITY, Json(error_json)).into_response();
    }

    // Create and insert new model
    let now = Utc::now().naive_utc();
    let id = Uuid::new_v4();
    let model = ReferenceModel::ActiveModel {
        id: Set(id),
        code: Set(payload.code),
        name: Set(payload.name),
        birth_place: Set(payload.birth_place),
        birth_date: Set(payload.birth_date),
        gender_id: Set(payload.gender_id),
        religion_id: Set(payload.religion_id),
        identification_type_id: Set(payload.identification_type_id),
        marital_status_id: Set(payload.marital_status_id),
        profession_id: Set(payload.profession_id),
        is_special_need: Set(payload.is_special_need),
        is_social_protection_card_recipient: Set(payload.is_social_protection_card_recipient),
        is_deceased: Set(payload.is_deceased),
        occupation_id: Set(payload.occupation_id),
        education_id: Set(payload.education_id),
        income_id: Set(payload.income_id),
        front_title: Set(payload.front_title.clone()),
        last_title: Set(payload.last_title.clone()),
        age_classification_id: Set(None),
        created_at: Set(Some(now)),
        updated_at: Set(Some(now)),
        sync_at: Set(None),
        deleted_at: Set(None),
        created_by: Set(Some(current_user.id)), // TODO: Set with authenticated user ID if available
        updated_by: Set(Some(current_user.id)),
    };

    let result = match model.insert(&ctx.db).await {
        Ok(res) => {
            // update current candidate
            let mut update_candidate: AcademicCandidateMasterCandidate::ActiveModel =
                current_candidate.clone().into();
            update_candidate.individual_id = Set(res.clone().id);
            match update_candidate.save(&ctx.db).await {
                Ok(_) => res.clone(),
                Err(err) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error": err.to_string()})),
                    )
                        .into_response();
                }
            };
            // get current user
            let mut update_user: AuthUser::ActiveModel = current_user.clone().into();
            update_user.individual_id = Set(res.clone().id);
            match update_user.save(&ctx.db).await {
                Ok(_) => res.clone(),
                Err(err) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error": err.to_string()})),
                    )
                        .into_response();
                }
            }
        }
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": err.to_string()})),
            )
                .into_response();
        }
    };

    // Success response
    (StatusCode::OK, Json(result)).into_response() // Or use a format helper like format::json(result)
}

#[allow(clippy::too_many_lines)]
pub async fn store_candidate_relatives(
    Path((candidate_id, relative_type_id)): Path<(Uuid, Uuid)>,
    State(ctx): State<AppContext>,
    JsonValidateWithMessage(payload): JsonValidateWithMessage<ModelValidator>,
) -> Response {
    // Fetch candidate from database
    let candidate_opt = match AcademicCandidateMasterCandidate::Entity::find()
        .filter(AcademicCandidateMasterCandidate::Column::DeletedAt.is_null())
        .filter(AcademicCandidateMasterCandidate::Column::Id.eq(candidate_id))
        .one(&ctx.db)
        .await
    {
        Ok(opt) => opt,
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": err.to_string()})),
            )
                .into_response();
        }
    };

    let Some(current_candidate) = candidate_opt else {
        return (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Candidate not found"})),
        )
            .into_response();
    };

    // Fetch associated user
    let user_opt = match AuthUser::Entity::find()
        .filter(AuthUser::Column::DeletedAt.is_null())
        .filter(AuthUser::Column::Id.eq(current_candidate.clone().user_id))
        .one(&ctx.db)
        .await
    {
        Ok(opt) => opt,
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": err.to_string()})),
            )
                .into_response();
        }
    };

    let Some(current_user) = user_opt else {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "User not found"})),
        )
            .into_response();
    };

    // Business logic checks
    if current_user.individual_id == Uuid::nil() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Individual ID not set"})),
        )
            .into_response();
    }

    // fecth family card
    let family_card_opt = match PersonMasterFamilyCard::Entity::find()
        .filter(PersonMasterFamilyCard::Column::DeletedAt.is_null())
        .filter(
            PersonMasterFamilyCard::Column::IndividualId
                .eq(current_candidate.clone().individual_id),
        )
        .one(&ctx.db)
        .await
    {
        Ok(opt) => opt,
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": err.to_string()})),
            )
                .into_response();
        }
    };

    let Some(current_family_card) = family_card_opt else {
        return (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Family Card not found"})),
        )
            .into_response();
    };

    // Validate payload
    if let Err(errors) = payload.validate() {
        let error_json = format_validation_errors(&errors, "Validation Failed"); // Assume this function exists
        return (StatusCode::UNPROCESSABLE_ENTITY, Json(error_json)).into_response();
    }

    if let Err(e) = payload.validate_unique_code(&ctx.db, None).await {
        let mut validation_errors = ValidationErrors::new(); // Assume ValidationErrors type exists
        validation_errors.add("code", e);
        let error_json = format_validation_errors(&validation_errors, "Validation Failed");
        return (StatusCode::UNPROCESSABLE_ENTITY, Json(error_json)).into_response();
    }

    // Create and insert new model
    let now = Utc::now().naive_utc();
    let id = Uuid::new_v4();
    let model = ReferenceModel::ActiveModel {
        id: Set(id),
        code: Set(payload.code),
        name: Set(payload.name),
        birth_place: Set(payload.birth_place),
        birth_date: Set(payload.birth_date),
        gender_id: Set(payload.gender_id),
        religion_id: Set(payload.religion_id),
        identification_type_id: Set(payload.identification_type_id),
        marital_status_id: Set(payload.marital_status_id),
        profession_id: Set(payload.profession_id),
        is_special_need: Set(payload.is_special_need),
        is_social_protection_card_recipient: Set(payload.is_social_protection_card_recipient),
        is_deceased: Set(payload.is_deceased),
        occupation_id: Set(payload.occupation_id),
        education_id: Set(payload.education_id),
        income_id: Set(payload.income_id),
        front_title: Set(payload.front_title.clone()),
        last_title: Set(payload.last_title.clone()),
        age_classification_id: Set(None),
        created_at: Set(Some(now)),
        updated_at: Set(Some(now)),
        sync_at: Set(None),
        deleted_at: Set(None),
        created_by: Set(Some(current_user.id)), // TODO: Set with authenticated user ID if available
        updated_by: Set(Some(current_user.id)),
    };

    let individual_result = match model.insert(&ctx.db).await {
        Ok(res) => {
            // create family card member
            let family_card_generated_id = Uuid::new_v4();
            let family_card_member_opt = PersonMasterFamilyCardMember::ActiveModel {
                id: Set(family_card_generated_id),
                family_card_id: Set(current_family_card.clone().id),
                individual_id: Set(current_user.clone().individual_id),
                relative_id: Set(res.clone().id),
                relative_type_id: Set(relative_type_id),
                created_at: Set(Some(now)),
                updated_at: Set(Some(now)),
                sync_at: Set(None),
                deleted_at: Set(None),
                created_by: Set(Some(current_user.clone().id)),
                updated_by: Set(Some(current_user.clone().id)),
            };

            let _result_family_card_member = match family_card_member_opt.insert(&ctx.db).await {
                Ok(result_family_card_member) => result_family_card_member,
                Err(err) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error": err.to_string()})),
                    )
                        .into_response();
                }
            };

            res // Return the individual record
        }
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": err.to_string()})),
            )
                .into_response();
        }
    };

    // Success response
    (StatusCode::OK, Json(individual_result)).into_response()
}

#[allow(clippy::too_many_lines)]
pub async fn store_student_relatives(
    Path((student_id, relative_type_id)): Path<(Uuid, Uuid)>,
    State(ctx): State<AppContext>,
    JsonValidateWithMessage(payload): JsonValidateWithMessage<ModelValidator>,
) -> Response {
    // Fetch candidate from database
    let student_opt = match AcademicStudentMasterStudent::Entity::find()
        .filter(AcademicStudentMasterStudent::Column::DeletedAt.is_null())
        .filter(AcademicStudentMasterStudent::Column::Id.eq(student_id))
        .one(&ctx.db)
        .await
    {
        Ok(opt) => opt,
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": err.to_string()})),
            )
                .into_response();
        }
    };

    let Some(current_student) = student_opt else {
        return (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Student not found"})),
        )
            .into_response();
    };

    // Fetch associated user
    let user_opt = match AuthUser::Entity::find()
        .filter(AuthUser::Column::DeletedAt.is_null())
        .filter(AuthUser::Column::IndividualId.eq(current_student.clone().individual_id))
        .one(&ctx.db)
        .await
    {
        Ok(opt) => opt,
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": err.to_string()})),
            )
                .into_response();
        }
    };

    let Some(current_user) = user_opt else {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "User not found"})),
        )
            .into_response();
    };

    // Business logic checks
    if current_user.individual_id == Uuid::nil() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Individual ID not set"})),
        )
            .into_response();
    }

    // fecth family card
    let family_card_opt = match PersonMasterFamilyCard::Entity::find()
        .filter(PersonMasterFamilyCard::Column::DeletedAt.is_null())
        .filter(
            PersonMasterFamilyCard::Column::IndividualId.eq(current_student.clone().individual_id),
        )
        .one(&ctx.db)
        .await
    {
        Ok(opt) => opt,
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": err.to_string()})),
            )
                .into_response();
        }
    };

    let Some(current_family_card) = family_card_opt else {
        return (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Family Card not found"})),
        )
            .into_response();
    };

    // Validate payload
    if let Err(errors) = payload.validate() {
        let error_json = format_validation_errors(&errors, "Validation Failed"); // Assume this function exists
        return (StatusCode::UNPROCESSABLE_ENTITY, Json(error_json)).into_response();
    }

    if let Err(e) = payload.validate_unique_code(&ctx.db, None).await {
        let mut validation_errors = ValidationErrors::new(); // Assume ValidationErrors type exists
        validation_errors.add("code", e);
        let error_json = format_validation_errors(&validation_errors, "Validation Failed");
        return (StatusCode::UNPROCESSABLE_ENTITY, Json(error_json)).into_response();
    }

    // Create and insert new model
    let now = Utc::now().naive_utc();
    let id = Uuid::new_v4();
    let model = ReferenceModel::ActiveModel {
        id: Set(id),
        code: Set(payload.code),
        name: Set(payload.name),
        birth_place: Set(payload.birth_place),
        birth_date: Set(payload.birth_date),
        gender_id: Set(payload.gender_id),
        religion_id: Set(payload.religion_id),
        identification_type_id: Set(payload.identification_type_id),
        marital_status_id: Set(payload.marital_status_id),
        profession_id: Set(payload.profession_id),
        is_special_need: Set(payload.is_special_need),
        is_social_protection_card_recipient: Set(payload.is_social_protection_card_recipient),
        is_deceased: Set(payload.is_deceased),
        occupation_id: Set(payload.occupation_id),
        education_id: Set(payload.education_id),
        income_id: Set(payload.income_id),
        front_title: Set(payload.front_title.clone()),
        last_title: Set(payload.last_title.clone()),
        age_classification_id: Set(None),
        created_at: Set(Some(now)),
        updated_at: Set(Some(now)),
        sync_at: Set(None),
        deleted_at: Set(None),
        created_by: Set(Some(current_user.id)), // TODO: Set with authenticated user ID if available
        updated_by: Set(Some(current_user.id)),
    };

    let individual_result = match model.insert(&ctx.db).await {
        Ok(res) => {
            // create family card member
            let family_card_generated_id = Uuid::new_v4();
            let family_card_member_opt = PersonMasterFamilyCardMember::ActiveModel {
                id: Set(family_card_generated_id),
                family_card_id: Set(current_family_card.clone().id),
                individual_id: Set(current_user.clone().individual_id),
                relative_id: Set(res.clone().id),
                relative_type_id: Set(relative_type_id),
                created_at: Set(Some(now)),
                updated_at: Set(Some(now)),
                sync_at: Set(None),
                deleted_at: Set(None),
                created_by: Set(Some(current_user.clone().id)),
                updated_by: Set(Some(current_user.clone().id)),
            };

            let _result_family_card_member = match family_card_member_opt.insert(&ctx.db).await {
                Ok(result_family_card_member) => result_family_card_member,
                Err(err) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error": err.to_string()})),
                    )
                        .into_response();
                }
            };

            res // Return the individual record
        }
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": err.to_string()})),
            )
                .into_response();
        }
    };

    // Success response
    (StatusCode::OK, Json(individual_result)).into_response()
}

pub fn routes(ctx: &AppContext) -> Routes {
    Routes::new()
        .prefix("api/person/master/individuals")
        // .add("/list", get(list))
        .add(
            "/store/candidate/{candidate_id}/relative_type/{relative_type_id}",
            post(store_candidate_relatives).layer(AuthenticatedLayer::new(ctx.clone())),
        )
        .add(
            "/store/candidate/{candidate_id}",
            post(store_candidate).layer(AuthenticatedLayer::new(ctx.clone())),
        )
        .add(
            "/store/student/{student_id}/relative_type/{relative_type_id}",
            post(store_student_relatives).layer(AuthenticatedLayer::new(ctx.clone())),
        )
        .add(
            "/store",
            post(store).layer(AuthorizedLayer::new(
                ctx.clone(),
                "person.master.individuals.store",
            )),
        )
        .add(
            "/{id}",
            get(show).layer(AuthorizedLayer::new(
                ctx.clone(),
                "person.master.individuals.show",
            )),
        )
        // .add(
        //     "/{id}",
        //     delete(destroy).layer(AuthorizedLayer::new(
        //         ctx.clone(),
        //         "person.master.individuals.destroy",
        //     )),
        // )
        // .add(
        //     "/{id}",
        //     put(update).layer(AuthorizedLayer::new(
        //         ctx.clone(),
        //         "person.master.individuals.update",
        //     )),
        // )
        .add(
            "/",
            post(index).layer(AuthorizedLayer::new(
                ctx.clone(),
                "person.master.individuals.index",
            )),
        )
}

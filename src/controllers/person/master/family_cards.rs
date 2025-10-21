use crate::middleware::is_authenticated::AuthenticatedLayer;
use crate::models::academic::candidate::master::candidates::_entities::candidates as AcademicCandidateMasterCandidate;
use crate::models::auth::users::_entities::users as AuthUser;
use crate::models::person::master::family_cards::_entities::family_cards as ReferenceModel;
use crate::models::person::master::family_cards::validation::family_cards::{
    ModelValidation, ModelValidator,
};
use crate::vendor::validation::common::format_validation_errors;
use axum::{Json, extract::Path, http::StatusCode};
use chrono::Utc;
use loco_rs::prelude::*;
// use serde::{Deserialize, Serialize};
use crate::models::academic::student::master::students::_entities::students as AcademicStudentMasterStudent;
use serde_json::json;
use uuid::Uuid;
use validator::ValidationErrors;

/// Returns data for function
///
/// # Errors
/// Returns an error if unable to retrieve model from database.
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

    // Validate payload
    if let Err(errors) = validator::Validate::validate(&payload) {
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
        individual_id: Set(current_user.clone().individual_id),
        created_at: Set(Some(now)),
        updated_at: Set(Some(now)),
        sync_at: Set(None),
        deleted_at: Set(None),
        created_by: Set(Some(current_user.id)), // TODO: Set with authenticated user ID if available
        updated_by: Set(Some(current_user.id)),
    };

    let result = match model.insert(&ctx.db).await {
        Ok(res) => res,
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

/// Returns data for function
///
/// # Errors
/// Returns an error if unable to retrieve model from database.
pub async fn store_student(
    Path(student_id): Path<Uuid>,
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
            Json(json!({"error": "Candidate not found"})),
        )
            .into_response();
    };

    // Fetch associated user
    let user_opt = match AuthUser::Entity::find()
        .filter(AuthUser::Column::DeletedAt.is_null())
        .filter(AuthUser::Column::IndividualId.eq(current_student.individual_id))
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

    // Validate payload
    if let Err(errors) = validator::Validate::validate(&payload) {
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
        individual_id: Set(current_user.clone().individual_id),
        created_at: Set(Some(now)),
        updated_at: Set(Some(now)),
        sync_at: Set(None),
        deleted_at: Set(None),
        created_by: Set(Some(current_user.id)), // TODO: Set with authenticated user ID if available
        updated_by: Set(Some(current_user.id)),
    };

    let result = match model.insert(&ctx.db).await {
        Ok(res) => res,
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

pub fn routes(ctx: &AppContext) -> Routes {
    Routes::new()
        .prefix("api/person/master/family_cards")
        .add(
            "/store/candidate/{candidate_id}",
            post(store_candidate).layer(AuthenticatedLayer::new(ctx.clone())),
        )
        .add(
            "/store/student/{student_id}",
            post(store_student).layer(AuthenticatedLayer::new(ctx.clone())),
        )
}

#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::middleware::is_authenticated::AuthenticatedLayer;
use crate::models::academic::candidate::master::candidates::_entities::candidates as AcademicCandidateMasterCandidate;
use crate::models::auth::users::_entities::users as AuthUser;
use crate::models::contact::master::residences::_entities::residences as ReferenceModel;
use crate::models::contact::master::residences::validation::residences::ModelValidator;
use crate::vendor::validation::common::format_validation_errors;
use axum::{Json, debug_handler, extract::Path, http::StatusCode};
use chrono::Utc;
use loco_rs::prelude::*;
use serde_json::json;
use uuid::{Uuid, uuid};
// use validator::ValidationErrors;

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
    if let Err(errors) = payload.validate() {
        let error_json = format_validation_errors(&errors, "Validation Failed"); // Assume this function exists
        return (StatusCode::UNPROCESSABLE_ENTITY, Json(error_json)).into_response();
    }

    // Create and insert new model
    let now = Utc::now().naive_utc();
    let id = Uuid::new_v4();
    let residence_type_id: Uuid = uuid!("a24667c1-6e1f-4b00-982f-b3b297bc4d60");
    let model = ReferenceModel::ActiveModel {
        id: Set(id),
        street: Set(payload.street),
        citizens_association: Set(payload.citizens_association),
        neighborhood_association: Set(payload.neighborhood_association),
        province_id: Set(payload.province_id),
        regency_id: Set(payload.regency_id),
        sub_district_id: Set(payload.sub_district_id),
        village_id: Set(payload.village_id),
        residence_type_id: Set(residence_type_id),
        residenceable_type: Set("App\\Models\\Person\\Master\\Individual".to_string()),
        residenceable_id: Set(current_user.clone().individual_id),
        latitude: Set(None),
        longitude: Set(None),
        zoom: Set(None),
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
        .prefix("api/contact/master/residences")
        .add("/", get(index))
        .add("/", post(store))
        .add("/{id}", get(show))
        .add("/{id}", delete(destroy))
        .add("/{id}", post(update))
        .add("/list", get(list))
        .add(
            "/store/candidate/{candidate_id}",
            post(store_candidate).layer(AuthenticatedLayer::new(ctx.clone())),
        )
}

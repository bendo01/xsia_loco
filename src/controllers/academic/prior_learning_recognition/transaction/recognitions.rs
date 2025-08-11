#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use crate::middleware::is_authenticated::AuthenticatedLayer;
// use crate::middleware::is_authorized::AuthorizedLayer;
use crate::models::academic::candidate::master::candidates::_entities::candidates as AcademicCandidateMasterCandidate;
use crate::models::academic::prior_learning_recognition::transaction::recognitions::_entities::recognitions as AcademicPriorLearningRecognitionTransactionRecognition;
use crate::models::institution::master::units::_entities::units as InstitutionMasterUnit;
use crate::models::auth::users::_entities::users as AuthUser;
use crate::models::academic::course::master::curriculums::_entities::curriculums as AcademicMasterCurriculum;
use crate::models::academic::prior_learning_recognition::transaction::recognitions::validation::recognitions::{
    // ModelValidation,
    ModelValidator,
};
use crate::vendor::validation::common::format_validation_errors;
use axum::{Json, debug_handler, extract::Path, http::StatusCode};
use axum::extract::Extension;
use chrono::Utc;
use loco_rs::prelude::*;
// use sea_orm::sea_query::Expr; // Import Expr to build expressions
// use sea_orm::{ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};
// use sea_query::extension::postgres::PgExpr;
// use random_string::generate;
use serde::Serialize;
use serde_json::json;
use uuid::Uuid;
// use validator::ValidationErrors;
#[derive(Serialize)]
struct ReturnMessage {
    message: String,
}

#[debug_handler]
pub async fn store(
    State(ctx): State<AppContext>,
    Extension(user): Extension<AuthUser::Model>,
    JsonValidateWithMessage(payload): JsonValidateWithMessage<ModelValidator>,
    
) -> Result<Response> {
    // let validation_errors = ValidationErrors::new();
    if let Err(validation_errors) = payload.validate() {
        let error_json = format_validation_errors(&validation_errors, "Validation Failed");
        return Ok((
            StatusCode::UNPROCESSABLE_ENTITY, // Set status to 422
            Json(error_json),                 // Return JSON-formatted errors
        ).into_response());
    }

    // find candidate
    let candidate = AcademicCandidateMasterCandidate::Entity::find_by_id(payload.candidate_id)
        .filter(AcademicCandidateMasterCandidate::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await?;

    if let Some(candidate) = candidate {
        let unit = InstitutionMasterUnit::Entity::find_by_id(payload.unit_id)
            .filter(InstitutionMasterUnit::Column::DeletedAt.is_null())
            .one(&ctx.db)
            .await?;

        if let Some(unit) = unit {
            let curriculum = AcademicMasterCurriculum::Entity::find()
                .filter(AcademicMasterCurriculum::Column::DeletedAt.is_null())
                .filter(AcademicMasterCurriculum::Column::UnitId.eq(unit.id))
                .filter(AcademicMasterCurriculum::Column::IsActive.eq(true))
                .one(&ctx.db)
                .await?;

            if let Some(curriculum) = curriculum {
                let registration_name = format!("{}_{}", unit.code, candidate.name);
                let now = Utc::now().naive_utc();
                let id = Uuid::new_v4();

                let model = AcademicPriorLearningRecognitionTransactionRecognition::ActiveModel {
                    id: Set(id),
                    name: Set(registration_name),
                    candidate_id: Set(candidate.id),
                    unit_id: Set(unit.id),
                    curriculum_id: Set(curriculum.id),
                    created_at: Set(Some(now)),
                    updated_at: Set(Some(now)),
                    sync_at: Set(None),
                    deleted_at: Set(None),
                    created_by: Set(Some(user.id)), // TODO: Set with authenticated user ID if available
                    updated_by: Set(Some(user.id)),
                };

                let model_result = match model.insert(&ctx.db).await {
                    Ok(res) => res,
                    Err(err) => {
                        return Ok((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(json!({"error": err.to_string()})),
                        )
                            .into_response());
                    }
                };
                return Ok((StatusCode::OK, Json(model_result)).into_response());
            }
        }
    }
    let msg = ReturnMessage {
        message: "Gagal Menyimpan Data".to_string(),
    };
    Ok((StatusCode::UNPROCESSABLE_ENTITY, Json(msg)).into_response())
}

#[debug_handler]
pub async fn show(Path(_id): Path<Uuid>, State(_ctx): State<AppContext>) -> Result<Response> {
    format::empty()
}

pub fn routes(ctx: &AppContext) -> Routes {
    Routes::new()
        .prefix("api/academic/prior_learning_recognition/transaction/recognition")
        .add(
            "/store",
            post(store).layer(AuthenticatedLayer::new(ctx.clone())),
        )
}

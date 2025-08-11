use crate::common::settings::Settings;
use crate::models::academic::candidate::master::candidates::_entities::candidates as AcademicCandidatMasterCandidate;
use crate::models::academic::general::reference::academic_years::_entities::academic_years as AcademicGeneralReferenceAcademicYear;
use crate::models::auth::users::_entities::users as AuthUser;
use loco_rs::prelude::*;
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, QueryOrder};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use uuid::Uuid;
use validator::{Validate, ValidationError};

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct CandidateValidator {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,
    #[validate(length(min = 2, message = "Name must be at least 2 characters long"))]
    pub name: String,
    pub phone: String,
    pub registration_type_id: Uuid,
    pub school_name: String,
    pub school_regency_id: Uuid,
    pub student_national_number: Option<String>,
    pub state_smart_card_number: Option<String>,
    pub guidence_name: Option<String>,
    pub guidence_phone_number: Option<String>,
}

pub trait CandidateValidation {
    fn validate_unique_email(
        &self,
        db: &DatabaseConnection,
    ) -> impl std::future::Future<Output = Result<(), ValidationError>> + Send;
}

impl CandidateValidation for CandidateValidator {
    async fn validate_unique_email(&self, db: &DatabaseConnection) -> Result<(), ValidationError> {
        // Check if email already exists in database
        let existing_user = AuthUser::Entity::find()
            .filter(AuthUser::Column::DeletedAt.is_null())
            .filter(
                model::query::condition()
                    .eq(AuthUser::Column::Email, &self.email)
                    .build(),
            )
            .one(db)
            .await;

        match existing_user {
            Ok(Some(_)) => {
                let mut error = ValidationError::new("email harus unik");
                error.message = Some(Cow::Borrowed("email sudah ada"));
                Err(error)
            }
            Ok(None) => Ok(()),
            Err(err) => {
                let mut error = ValidationError::new("error pengaksesan database");
                error.message = Some(Cow::Owned(err.to_string()));
                Err(error)
            }
        }
    }
}

impl CandidateValidator {
    /// Generates a unique code for a candidate based on the institution code and a sequential thread number.
    ///
    /// # Arguments
    ///
    /// * `db` - A database connection for querying existing candidates
    ///
    /// # Returns
    ///
    /// A tuple containing the thread number (i32) and the generated code (String)
    ///
    /// # Errors
    ///
    /// This function will return a `DbErr` if:
    /// - There is an error when beginning the database transaction
    /// - There is an error when querying the database for the last candidate
    /// - There is an error when committing the database transaction
    ///
    /// # Panics
    ///
    /// This function will panic if:
    /// - The `current_institution_id` in the application configuration is not a valid UUID
    /// - The `current_student_admission_academic_year_id` in the application configuration is not a valid UUID
    pub async fn generate_code(ctx: AppContext) -> Result<(i32, String), DbErr> {
        // Start a transaction
        // let txn = &ctx.db.begin().await?;

        if let Some(settings) = &ctx.config.settings {
            let settings = Settings::from_json(settings)
                .map_err(|e| DbErr::Custom(format!("Failed to parse settings: {e}")))?;
            let setting_institution_id = settings.current_institution_id;
            let setting_academic_year_id = settings.current_student_admission_academic_year_id;
            let institution_code = settings.current_institution_code;

            let current_institution_id = Uuid::parse_str(setting_institution_id.as_str())
                .expect("Invalid UUID format in CURRENT_INSTITUTION_ID");
            let current_academic_year_id = Uuid::parse_str(setting_academic_year_id.as_str())
                .expect("Invalid UUID format in CURRENT_INSTITUTION_ID");

            let current_academic_year = AcademicGeneralReferenceAcademicYear::Entity::find()
                .filter(AcademicGeneralReferenceAcademicYear::Column::DeletedAt.is_null())
                .filter(
                    AcademicGeneralReferenceAcademicYear::Column::Id.eq(current_academic_year_id),
                )
                .one(&ctx.db)
                .await?;

            let Some(academic_year) = current_academic_year else {
                return Err(DbErr::Custom("Current academic year not found".to_string()));
            };

            // Find the candidate with the highest thread number
            let last_candidate: Option<AcademicCandidatMasterCandidate::Model> =
                AcademicCandidatMasterCandidate::Entity::find()
                    .filter(AcademicCandidatMasterCandidate::Column::DeletedAt.is_null())
                    .filter(
                        AcademicCandidatMasterCandidate::Column::AcademicYearId
                            .eq(current_academic_year_id),
                    )
                    .filter(
                        AcademicCandidatMasterCandidate::Column::InstitutionId
                            .eq(current_institution_id),
                    )
                    .order_by_desc(AcademicCandidatMasterCandidate::Column::Thread)
                    .order_by_desc(AcademicCandidatMasterCandidate::Column::CreatedAt)
                    .one(&ctx.db)
                    .await?;

            // Calculate the next thread number
            // Using map_or instead of map().unwrap_or() as suggested by Clippy
            let thread = last_candidate.map_or(0, |c| c.thread) + 1;

            // Generate the code string
            let code = format!(
                "KANDIDAT-{}-{}-{:03}",
                institution_code.clone(),
                academic_year.clone().year,
                thread
            );

            // Commit the transaction
            // txn.commit().await?;

            // Return the thread number and code as a tuple
            Ok((thread, code))
        } else {
            Err(DbErr::Custom(
                "fail to register user because setting not loaded".to_string(),
            ))
        }
    }
}

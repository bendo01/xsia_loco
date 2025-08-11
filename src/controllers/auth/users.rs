use crate::common::settings::Settings;
use crate::models::academic::candidate::master::candidates::_entities::candidates as AcademicCandidateMasterCandidate;
use crate::models::academic::candidate::reference::registration_types::_entities::registration_types as AcademicCandidateReferenceRegistrationType;
use crate::models::academic::candidate::transaction::unit_choices::_entities::unit_choices as AcademicCandidateTransactionUnitChoice;
use crate::models::academic::student::master::students::_entities::students as AcademicStudentMasterStudent;
use crate::models::auth::roles::_entities::roles as AuthRole;
use crate::models::auth::users::_entities::users as ReferenceModel;
use crate::models::auth::users::data_objects::DataObject;
use crate::models::auth::users::validation::candidate::{CandidateValidation, CandidateValidator};
use crate::models::auth::users::validation::claim_user::{ClaimUserValidation, ClaimUserValidator};
use crate::models::contact::master::phones::_entities::phones as ContactMasterPhone;
use crate::models::institution::master::units::_entities::units as InstitutionMasterUnit;
use crate::models::person::master::individuals::_entities::individuals as PersonMasterIndividual;
use crate::vendor::validation::common::format_validation_errors;
use crate::{
    mailers::auth::auth::AuthMailer,
    models::auth::users::{
        _entities::users,
        users::{LoginParams, RegisterParams},
    },
    // views::auth::{CurrentResponse, LoginResponse},
    views::auth::users::LoginResponse,
};
use axum::{Json, debug_handler, extract::Path, http::StatusCode};
use chrono::Local;
use loco_rs::{hash, prelude::*};
use regex::Regex;
use sea_orm::{
    ColumnTrait, EntityTrait, FromQueryResult, QueryFilter, QueryOrder, QuerySelect, Set,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::OnceLock;
use uuid::{Uuid, uuid};
use validator::ValidationErrors;

pub static EMAIL_DOMAIN_RE: OnceLock<Regex> = OnceLock::new();

// fn get_allow_email_domain_re() -> &'static Regex {
//     EMAIL_DOMAIN_RE.get_or_init(|| {
//         Regex::new(r"@example\.com$|@gmail\.com$").expect("Failed to compile regex")
//     })
// }

#[derive(Debug, FromQueryResult, Serialize)]
struct PartialInstitutionMasterUnit {
    pub id: Uuid,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ForgotParams {
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResetParams {
    pub token: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MagicLinkParams {
    pub email: String,
}

#[allow(clippy::too_many_lines)]
#[debug_handler]
async fn user_claim(
    State(ctx): State<AppContext>,
    JsonValidateWithMessage(params): JsonValidateWithMessage<ClaimUserValidator>,
) -> Result<Response> {
    let mut validation_errors = ValidationErrors::new();
    if let Err(validation_errors) = params.validate() {
        let error_json = format_validation_errors(&validation_errors, "Validation Failed");
        return Ok((StatusCode::UNPROCESSABLE_ENTITY, Json(error_json)).into_response());
    }

    // Check uniqueness for code, alphabet_code, and name
    if let Err(e) = params.clone().validate_unique_email(&ctx.db).await {
        validation_errors.add("email", e.clone());
    }

    if let Err(e) = params.clone().validate_student_exists(&ctx.db).await {
        validation_errors.add("student_code", e.clone());
    }

    if let Err(e) = params.clone().validate_individual_exists(&ctx.db).await {
        validation_errors.add("individual_code", e.clone());
    }

    if let Err(e) = params
        .clone()
        .validate_individual_related_to_student(&ctx.db)
        .await
    {
        validation_errors.add("individual_code", e.clone());
        validation_errors.add("student_code", e.clone());
    }

    if let Err(e) = params
        .clone()
        .validate_individual_related_to_user(&ctx.db)
        .await
    {
        validation_errors.add("individual_code", e.clone());
        validation_errors.add("email", e.clone());
    }

    // Validate the payload using validator
    if !validation_errors.is_empty() {
        let error_json = format_validation_errors(&validation_errors, "Gagal Validasi");
        return Ok((StatusCode::UNPROCESSABLE_ENTITY, Json(error_json)).into_response());
    }

    if let Some(settings) = &ctx.config.settings {
        let settings = Settings::from_json(settings)?;
        let mut unit_ids: Vec<Uuid> = Vec::new();
        #[allow(unused_assignments)]
        let mut student: Option<AcademicStudentMasterStudent::Model> = None;
        #[allow(unused_assignments)]
        let mut individual: Option<PersonMasterIndividual::Model> = None;
        let mut is_user_found = false;
        let mut is_individual_related_to_student = false;

        if let Ok(institution_id) = Uuid::parse_str(settings.current_institution_id.as_str()) {
            let unit_type_id = uuid!("019759fd-36e8-4f43-80ed-4f687a48145d");
            let list_unit_ids = InstitutionMasterUnit::Entity::find()
                .filter(InstitutionMasterUnit::Column::DeletedAt.is_null())
                .filter(InstitutionMasterUnit::Column::InstitutionId.eq(institution_id))
                .filter(InstitutionMasterUnit::Column::UnitTypeId.eq(unit_type_id))
                .order_by_asc(InstitutionMasterUnit::Column::Id)
                .select_only()
                .column(InstitutionMasterUnit::Column::Id)
                .into_model::<PartialInstitutionMasterUnit>()
                .all(&ctx.db)
                .await?;

            for unit_id in &list_unit_ids {
                unit_ids.push(unit_id.id);
            }

            if !unit_ids.is_empty() {
                let student_result = AcademicStudentMasterStudent::Entity::find()
                    .filter(AcademicStudentMasterStudent::Column::DeletedAt.is_null())
                    .filter(AcademicStudentMasterStudent::Column::UnitId.is_in(unit_ids))
                    .filter(
                        AcademicStudentMasterStudent::Column::Code.eq(params.clone().student_code),
                    )
                    .one(&ctx.db)
                    .await?;
                if let Some(student_temp) = student_result {
                    student = Some(student_temp);
                } else {
                    return Ok((
                        StatusCode::NOT_FOUND,
                        Json(json!({
                            "message": "Student not found",
                            "status": "error"
                        })),
                    )
                        .into_response());
                }
            } else {
                return Ok((
                    StatusCode::NOT_FOUND,
                    Json(json!({
                        "message": "No units found",
                        "status": "error"
                    })),
                )
                    .into_response());
            }
        } else {
            return Ok((
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "message": "Failed to parse UUID",
                    "status": "error"
                })),
            )
                .into_response());
        }

        // Find individual student
        let individual_result = PersonMasterIndividual::Entity::find()
            .filter(PersonMasterIndividual::Column::DeletedAt.is_null())
            .filter(PersonMasterIndividual::Column::Code.eq(params.clone().individual_code))
            .one(&ctx.db)
            .await?;

        if let Some(individual_temp) = individual_result {
            individual = Some(individual_temp);
        } else {
            return Ok((
                StatusCode::NOT_FOUND,
                Json(json!({
                    "message": "Individual not found",
                    "status": "error"
                })),
            )
                .into_response());
        }

        // Find user
        let user_result = ReferenceModel::Entity::find()
            .filter(ReferenceModel::Column::DeletedAt.is_null())
            .filter(ReferenceModel::Column::Email.eq(params.clone().email))
            .one(&ctx.db)
            .await?;

        if let Some(_user_temp) = user_result {
            is_user_found = true;
        }

        // Check if individual is associated with student
        if let (Some(student_data), Some(individual_data)) = (&student, &individual) {
            if student_data.individual_id == individual_data.id {
                is_individual_related_to_student = true;
            }
        }

        if !is_user_found && is_individual_related_to_student {
            // Create user
            let uuidv7_string = uuid7::uuid7().to_string();
            let user_generated_id = Uuid::parse_str(&uuidv7_string)
                .map_err(|_| Error::string("Invalid UUID format"))?;
            let password_hash = hash::hash_password(params.clone().password.as_str())
                .map_err(|e| ModelError::Any(e.into()))?;

            if let Some(individual_data) = &individual {
                let user_created = ReferenceModel::ActiveModel {
                    id: ActiveValue::set(user_generated_id),
                    email: ActiveValue::set(params.clone().email),
                    password: ActiveValue::set(password_hash),
                    name: ActiveValue::set(individual_data.name.clone()),
                    individual_id: ActiveValue::set(individual_data.id),
                    ..Default::default()
                }
                .insert(&ctx.db)
                .await?;

                // Create role
                let uuidv7_string = uuid7::uuid7().to_string();
                let role_generated_id = Uuid::parse_str(&uuidv7_string)
                    .map_err(|_| Error::string("Invalid UUID format"))?;

                let role_insert = AuthRole::ActiveModel {
                    id: ActiveValue::set(role_generated_id),
                    name: ActiveValue::set("Mahasiswa".to_string()),
                    user_id: ActiveValue::set(user_created.id),
                    position_type_id: ActiveValue::set(uuid!(
                        "5917531e-8e5d-4258-b2e7-0c5489fd63ff"
                    )),
                    roleable_id: ActiveValue::Set(student.clone().unwrap().id),
                    roleable_type: ActiveValue::set(
                        "App\\Models\\Academic\\Student\\Master\\Student".to_string(),
                    ),
                    ..Default::default()
                };

                let role = role_insert.insert(&ctx.db).await?;

                let mut user_active_model = user_created.into_active_model();
                user_active_model.current_role_id = Set(role.id);
                user_active_model.update(&ctx.db).await?;
            } else {
                return Ok((
                    StatusCode::NOT_FOUND,
                    Json(json!({
                        "message": "Individual data not available for user creation",
                        "status": "error"
                    })),
                )
                    .into_response());
            }
        } else {
            return Ok((
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "message": "User and individual are not associated with student",
                    "status": "error"
                })),
            )
                .into_response());
        }
    } else {
        return Ok((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "message": "Setting Not loaded",
                "status": "error"
            })),
        )
            .into_response());
    }

    Ok(Json(json!({
        "message": "Akuisisi akun berhasil",
        "status": "success"
    }))
    .into_response())
}
/// Register function creates a new user with the given parameters and sends a
/// welcome email to the user
#[allow(clippy::too_many_lines)]
#[debug_handler]
async fn candidate_register(
    State(ctx): State<AppContext>,
    JsonValidateWithMessage(params): JsonValidateWithMessage<CandidateValidator>,
) -> Result<Response> {
    let mut validation_errors = ValidationErrors::new();
    if let Err(validation_errors) = params.validate() {
        let error_json = format_validation_errors(&validation_errors, "Validation Failed");
        return Ok((
            StatusCode::UNPROCESSABLE_ENTITY, // Set status to 422
            Json(error_json),                 // Return JSON-formatted errors
        )
            .into_response());
    }

    // Check uniqueness for code, alphabet_code, and name
    if let Err(e) = params.validate_unique_email(&ctx.db).await {
        validation_errors.add("email", e);
    }

    // Validate the payload using validator
    // params.validate()?;
    if !validation_errors.is_empty() {
        let error_json = format_validation_errors(&validation_errors, "Validation Failed");
        return Ok((StatusCode::UNPROCESSABLE_ENTITY, Json(error_json)).into_response());
    }

    let candidate_user = RegisterParams {
        email: params.email.clone(),
        password: params.password.clone(),
        name: params.name.clone(),
    };
    let res = users::Model::create_with_password(&ctx.db, &candidate_user).await;
    let user = match res {
        Ok(user) => user,
        Err(err) => {
            tracing::info!(
                message = err.to_string(),
                user_email = &candidate_user.email,
                "could not register user",
            );
            return format::json(());
        }
    };

    // create candidate
    let now = Local::now().naive_local();
    let uuidv7_string = uuid7::uuid7().to_string();
    let candidate_pk_id = Uuid::parse_str(&uuidv7_string).expect("Invalid UUID format");
    let empty_id: Uuid = uuid!("00000000-0000-0000-0000-000000000000");
    // let candidate_pk_id = Uuid::new_v4();
    let (generated_thread, generated_code) = CandidateValidator::generate_code(ctx.clone()).await?;
    if let Some(settings) = &ctx.config.settings {
        let settings = Settings::from_json(settings)?;
        // println!("{:?}", settings.current_institution_code);
        let setting_current_institution_id = settings.current_institution_id;
        let setting_current_academic_year_id = settings.current_student_admission_academic_year_id;
        let current_institution_id = Uuid::parse_str(setting_current_institution_id.as_str())
            .expect("Invalid UUID format in CURRENT_INSTITUTION_ID");
        let current_academic_year_id = Uuid::parse_str(setting_current_academic_year_id.as_str())
            .expect("Invalid UUID format in CURRENT_INSTITUTION_ID");
        let current_candidate = AcademicCandidateMasterCandidate::ActiveModel {
            id: Set(candidate_pk_id),
            thread: Set(generated_thread),
            code: Set(generated_code.clone()),
            name: Set(user.name.clone()),
            school_name: Set(Some(params.school_name.clone())),
            school_regency_id: Set(params.school_regency_id),
            registration_type_id: Set(params.registration_type_id),
            student_national_number: Set(params.student_national_number.clone()),
            state_smart_card_number: Set(params.state_smart_card_number.clone()),
            guidence_name: Set(params.guidence_name.clone()),
            guidence_phone_number: Set(params.guidence_phone_number.clone()),
            user_id: Set(user.id),
            academic_year_id: Set(current_academic_year_id),
            institution_id: Set(current_institution_id),
            student_id: Set(None),
            individual_id: Set(empty_id),
            created_at: Set(Some(now)),
            updated_at: Set(Some(now)),
            sync_at: Set(None),
            deleted_at: Set(None),
            created_by: Set(Some(user.id)), // TODO: Set with authenticated user ID if available
            updated_by: Set(Some(user.id)),
        };
        let created_candidate = current_candidate.insert(&ctx.db).await;
        let _ = match created_candidate {
            Ok(user_created_candidate) => user_created_candidate,
            Err(err) => {
                tracing::info!(
                    message = err.to_string(),
                    user_email = &candidate_user.email,
                    "could not create candidate",
                );
                return format::json(());
            }
        };
        // end create candidate
        // start create candidate unit choice
        let candidate_unit_choice = AcademicCandidateReferenceRegistrationType::Entity::find()
            .filter(AcademicCandidateReferenceRegistrationType::Column::DeletedAt.is_null())
            .filter(
                AcademicCandidateReferenceRegistrationType::Column::Id
                    .eq(params.registration_type_id),
            )
            .one(&ctx.db)
            .await;
        let current_unit_choice = match candidate_unit_choice {
            Ok(current_unit_choice) => current_unit_choice,
            Err(err) => {
                tracing::info!(
                    message = err.to_string(),
                    user_email = &candidate_user.email,
                    "could not find candidate unit choice",
                );
                return format::json(());
            }
        };

        // create candidate unit choice
        let uuidv7_string = uuid7::uuid7().to_string();
        let current_candidate_unit_choice_pk_id =
            Uuid::parse_str(&uuidv7_string).expect("Invalid UUID format");
        // let current_candidate_unit_choice_pk_id = Uuid::new_v4();
        let phase_uuid: Uuid = uuid!("246e0348-5b4f-4c27-8159-5f706725b651");
        let create_candidate_unit_choice = AcademicCandidateTransactionUnitChoice::ActiveModel {
            id: Set(current_candidate_unit_choice_pk_id),
            priority: Set(1),
            phase_id: Set(phase_uuid),
            unit_id: Set(current_unit_choice.clone().unwrap().unit_id),
            student_registration_id: Set(current_unit_choice
                .clone()
                .unwrap()
                .student_registration_id),
            registration_category_id: Set(current_unit_choice
                .clone()
                .unwrap()
                .registration_category_id),
            candidate_id: Set(candidate_pk_id),
            created_at: Set(Some(now)),
            updated_at: Set(Some(now)),
            sync_at: Set(None),
            deleted_at: Set(None),
            created_by: Set(Some(user.id)), // TODO: Set with authenticated user ID if available
            updated_by: Set(Some(user.id)),
        };
        let ccuc = create_candidate_unit_choice.insert(&ctx.db).await;
        match ccuc {
            Ok(_) => (),
            Err(e) => {
                println!("Error creating candidate unit choice: {e}");
            }
        }

        // end create candidate unit choice
        // start create role
        // let role_pk_id = Uuid::new_v4();
        let uuidv7_string = uuid7::uuid7().to_string();
        let role_pk_id = Uuid::parse_str(&uuidv7_string).expect("Invalid UUID format");
        let position_type_uuid: Uuid = uuid!("6ca078ac-fb8d-41af-a6b0-781277de1c48");
        let current_role = AuthRole::ActiveModel {
            id: Set(role_pk_id),
            name: Set(generated_code.clone()),
            user_id: Set(user.id),
            position_type_id: Set(position_type_uuid),
            roleable_id: Set(candidate_pk_id),
            roleable_type: Set("App\\Models\\Academic\\Candidate\\Master\\Candidate".to_string()),
            created_at: Set(Some(now)),
            updated_at: Set(Some(now)),
            sync_at: Set(None),
            deleted_at: Set(None),
            created_by: Set(Some(user.id)), // TODO: Set with authenticated user ID if available
            updated_by: Set(Some(user.id)),
        };
        let created_role = current_role.insert(&ctx.db).await;
        let current_role = match created_role {
            Ok(current_role) => current_role,
            Err(_err) => {
                tracing::info!(user_email = &candidate_user.email, "could not create role",);
                return format::json(());
            }
        };
        // end create role
        // create candidate phone
        let uuidv7_string = uuid7::uuid7().to_string();
        let phone_pk_id = Uuid::parse_str(&uuidv7_string).expect("Invalid UUID format");
        // let phone_pk_id = Uuid::new_v4();
        let phone_type_uuid: Uuid = uuid!("5cf06051-0434-4581-807f-52ee40c7b7bf");
        let current_phone = ContactMasterPhone::ActiveModel {
            id: Set(phone_pk_id),
            phone_number: Set(params.phone.clone()),
            phone_type_id: Set(phone_type_uuid),
            phoneable_id: Set(candidate_pk_id),
            phoneable_type: Set("App\\Models\\Academic\\Candidate\\Master\\Candidate".to_string()),
            created_at: Set(Some(now)),
            updated_at: Set(Some(now)),
            sync_at: Set(None),
            deleted_at: Set(None),
            created_by: Set(Some(user.id)), // TODO: Set with authenticated user ID if available
            updated_by: Set(Some(user.id)),
        };
        let created_phone = current_phone.insert(&ctx.db).await;
        let _ = match created_phone {
            Ok(created_phone) => created_phone,
            Err(_err) => {
                tracing::info!(user_email = &candidate_user.email, "could not create role",);
                return format::json(());
            }
        };
        // end candidate phone
        // start set user active role
        let mut user_active_model = user.clone().into_active_model();
        user_active_model.current_role_id = Set(current_role.id);
        let user_updated_model = user_active_model.update(&ctx.db).await;
        match user_updated_model {
            Ok(updated_user) => updated_user,
            Err(_err) => {
                tracing::info!(user_email = &candidate_user.email, "could not update user",);
                return format::json(());
            }
        };
        // end set user active role

        let user = user
            .into_active_model()
            .set_email_verification_sent(&ctx.db)
            .await?;
        // let updated_user = user_updated_model
        //     .set_email_verification_sent(&ctx.db)
        //     .await?;
        AuthMailer::send_welcome(&ctx, &user).await?;
        format::json(json!({ "message": "success user registration" }))
    } else {
        Err(Error::string(
            "fail to register user because setting not loaded",
        ))
    }
}

/// Register function creates a new user with the given parameters and sends a
/// welcome email to the user
#[debug_handler]
async fn register(
    State(ctx): State<AppContext>,
    Json(params): Json<RegisterParams>,
) -> Result<Response> {
    let res = users::Model::create_with_password(&ctx.db, &params).await;

    let user = match res {
        Ok(user) => user,
        Err(err) => {
            tracing::info!(
                message = err.to_string(),
                user_email = &params.email,
                "could not register user",
            );
            return format::json(());
        }
    };

    let user = user
        .into_active_model()
        .set_email_verification_sent(&ctx.db)
        .await?;

    AuthMailer::send_welcome(&ctx, &user).await?;

    format::json(())
}

/// Verify register user. if the user not verified his email, he can't login to
/// the system.
#[debug_handler]
async fn verify(State(ctx): State<AppContext>, Path(token): Path<String>) -> Result<Response> {
    let user = users::Model::find_by_verification_token(&ctx.db, &token).await?;

    if user.email_verified_at.is_some() {
        tracing::info!(pid = user.pid.to_string(), "user already verified");
    } else {
        let active_model = user.into_active_model();
        let user = active_model.verified(&ctx.db).await?;
        tracing::info!(pid = user.pid.to_string(), "user verified");
    }

    format::json(())
}

/// In case the user forgot his password  this endpoints generate a forgot token
/// and send email to the user. In case the email not found in our DB, we are
/// returning a valid request for for security reasons (not exposing users DB
/// list).
#[debug_handler]
async fn forgot(
    State(ctx): State<AppContext>,
    Json(params): Json<ForgotParams>,
) -> Result<Response> {
    let Ok(user) = users::Model::find_by_email(&ctx.db, &params.email).await else {
        // we don't want to expose our users email. if the email is invalid we still
        // returning success to the caller
        return format::json(());
    };

    let user = user
        .into_active_model()
        .set_forgot_password_sent(&ctx.db)
        .await?;

    AuthMailer::forgot_password(&ctx, &user).await?;

    format::json(())
}

/// reset user password by the given parameters
#[debug_handler]
async fn reset(State(ctx): State<AppContext>, Json(params): Json<ResetParams>) -> Result<Response> {
    let Ok(user) = users::Model::find_by_reset_token(&ctx.db, &params.token).await else {
        // we don't want to expose our users email. if the email is invalid we still
        // returning success to the caller
        tracing::info!("reset token not found");

        return format::json(());
    };
    user.into_active_model()
        .reset_password(&ctx.db, &params.password)
        .await?;

    format::json(())
}

/// Creates a user login and returns a token
#[debug_handler]
async fn login(State(ctx): State<AppContext>, Json(params): Json<LoginParams>) -> Result<Response> {
    // let user = users::Model::find_by_email(&ctx.db, &params.email).await?;
    let login_user = users::Entity::find()
        .filter(users::Column::DeletedAt.is_null())
        .filter(users::Column::EmailVerifiedAt.is_not_null())
        .filter(users::Column::Email.eq(&params.email))
        .one(&ctx.db)
        .await?;

    let Some(user) = login_user else {
        return unauthorized("Email Belum Terverifikasi");
    };

    let valid = user.verify_password(&params.password);

    if !valid {
        return unauthorized("unauthorized!");
    }

    let jwt_secret = ctx.config.get_jwt_config()?;

    let token = user
        .generate_jwt(&jwt_secret.secret, jwt_secret.expiration)
        .or_else(|_| unauthorized("unauthorized!"))?;

    format::json(LoginResponse::new(&user, &token))
}

#[debug_handler]
async fn current(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let user = ReferenceModel::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let returned = DataObject::get_by_id(&ctx, user.id).await?;
    // println!("User: {:?}", user);
    // println!("Returned user: {:?}", returned);
    format::json(returned)
}

#[debug_handler]
async fn set_current_role(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Path(role_id): Path<Uuid>,
) -> Result<Response> {
    let user = ReferenceModel::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let mut item = user.clone().into_active_model();
    item.current_role_id = Set(role_id);
    let _updated_item = item.update(&ctx.db).await?;
    let returned = DataObject::get_by_id(&ctx, user.id).await?;
    format::json(returned)
}

/// Magic link authentication provides a secure and passwordless way to log in to the application.
///
/// # Flow
/// 1. **Request a Magic Link**:
///    A registered user sends a POST request to `/magic-link` with their email.
///    If the email exists, a short-lived, one-time-use token is generated and sent to the user's email.
///    For security and to avoid exposing whether an email exists, the response always returns 200, even if the email is invalid.
///
/// 2. **Click the Magic Link**:
///    The user clicks the link (/magic-link/{token}), which validates the token and its expiration.
///    If valid, the server generates a JWT and responds with a [`LoginResponse`].
///    If invalid or expired, an unauthorized response is returned.
///
/// This flow enhances security by avoiding traditional passwords and providing a seamless login experience.
// async fn magic_link(
//     State(ctx): State<AppContext>,
//     Json(params): Json<MagicLinkParams>,
// ) -> Result<Response> {
//     let email_regex = get_allow_email_domain_re();
//     if !email_regex.is_match(&params.email) {
//         tracing::debug!(
//             email = params.email,
//             "The provided email is invalid or does not match the allowed domains"
//         );
//         return bad_request("invalid request");
//     }

//     let Ok(user) = users::Model::find_by_email(&ctx.db, &params.email).await else {
//         // we don't want to expose our users email. if the email is invalid we still
//         // returning success to the caller
//         tracing::debug!(email = params.email, "user not found by email");
//         return format::empty_json();
//     };

//     let user = user.into_active_model().create_magic_link(&ctx.db).await?;
//     AuthMailer::send_magic_link(&ctx, &user).await?;

//     format::empty_json()
// }

/// Verifies a magic link token and authenticates the user.
// async fn magic_link_verify(
//     Path(token): Path<String>,
//     State(ctx): State<AppContext>,
// ) -> Result<Response> {
//     let Ok(user) = users::Model::find_by_magic_token(&ctx.db, &token).await else {
//         // we don't want to expose our users email. if the email is invalid we still
//         // returning success to the caller
//         return unauthorized("unauthorized!");
//     };

//     let user = user.into_active_model().clear_magic_link(&ctx.db).await?;

//     let jwt_secret = ctx.config.get_jwt_config()?;

//     let token = user
//         .generate_jwt(&jwt_secret.secret, &jwt_secret.expiration)
//         .or_else(|_| unauthorized("unauthorized!"))?;

//     format::json(LoginResponse::new(&user, &token))
// }

#[allow(clippy::empty_line_after_doc_comments)]
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/auth/users")
        .add("/set_current_role/{role_id}", get(set_current_role))
        .add("/candidate/register", post(candidate_register))
        .add("/register", post(register))
        .add("/user_claim", post(user_claim))
        .add("/verify/{token}", get(verify))
        .add("/login", post(login))
        .add("/forgot", post(forgot))
        .add("/reset", post(reset))
        .add("/current", get(current))
    // .add("/magic-link", post(magic_link))
    // .add("/magic-link/{token}", get(magic_link_verify))
}

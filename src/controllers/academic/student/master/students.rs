#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::middleware::is_authenticated::AuthenticatedLayer;
use crate::models::academic::general::reference::academic_years::_entities::academic_years as AcademicGeneralReferenceAcademicYear;
use crate::models::academic::student::master::students::_entities::students as AcademicStudentMasterStudent;
use crate::models::academic::student::master::students::student_validate::StudentValidate;
use crate::models::institution::master::units::_entities::units as InstitutionMasterUnit;
use crate::models::person::master::individuals::_entities::individuals as PersonMasterIndividual;
use axum::debug_handler;
use loco_rs::prelude::*;
use sea_orm::LoaderTrait;

#[debug_handler]
pub async fn student_validation(
    Path(id): Path<Uuid>,
    State(ctx): State<AppContext>,
) -> Result<Response, loco_rs::Error> {
    let candidate_result = StudentValidate::check(&ctx, id).await;
    match candidate_result {
        Ok(candidate_check) => format::json(candidate_check),
        Err(err) => {
            let error_message = format!("Error checking student: {err}");
            Err(Error::Message(error_message))
        }
    }
}

#[debug_handler]
pub async fn index(State(_ctx): State<AppContext>) -> Result<Response> {
    format::empty()
}

#[debug_handler]
pub async fn index_unit(
    State(_ctx): State<AppContext>,
    Path(_unit_id): Path<Uuid>,
) -> Result<Response> {
    format::empty()
}

#[debug_handler]
pub async fn index_institution(
    State(ctx): State<AppContext>,
    Path(institution_id): Path<Uuid>,
) -> Result<Response> {
    let unit_type_id: Uuid = uuid::uuid!("019759fd-36e8-4f43-80ed-4f687a48145d");
    let unit_opt = InstitutionMasterUnit::Entity::find()
        .filter(InstitutionMasterUnit::Column::DeletedAt.is_null())
        .filter(InstitutionMasterUnit::Column::InstitutionId.eq(institution_id))
        .filter(InstitutionMasterUnit::Column::UnitTypeId.eq(unit_type_id))
        .all(&ctx.db)
        .await?;

    // Extract unit IDs from the found units
    let unit_ids: Vec<Uuid> = unit_opt.iter().map(|unit| unit.id).collect();

    // If no units found, return empty response
    if unit_ids.is_empty() {
        return format::json(Vec::<AcademicStudentMasterStudent::Model>::new());
    }

    // Query students that belong to these units
    let students = AcademicStudentMasterStudent::Entity::find()
        .filter(AcademicStudentMasterStudent::Column::DeletedAt.is_null())
        .filter(AcademicStudentMasterStudent::Column::UnitId.is_in(unit_ids))
        .all(&ctx.db)
        .await?;

    // Load multiple relations efficiently
    let units = students
        .load_one(InstitutionMasterUnit::Entity, &ctx.db)
        .await?;
    let individuals = students
        .load_one(PersonMasterIndividual::Entity, &ctx.db)
        .await?;
    let academic_years = students
        .load_one(AcademicGeneralReferenceAcademicYear::Entity, &ctx.db)
        .await?;

    // Combine data
    let response_data: Vec<_> = students
        .into_iter()
        .enumerate()
        .map(|(i, student)| {
            serde_json::json!({
                "student": student,
                "unit": units[i].as_ref(),
                "individual": individuals[i].as_ref(),
                "academic_year": academic_years[i].as_ref(),
            })
        })
        .collect();

    format::json(response_data)
}

#[debug_handler]
pub async fn store(State(_ctx): State<AppContext>) -> Result<Response> {
    format::empty()
}

#[debug_handler]
pub async fn show(State(ctx): State<AppContext>, Path(id): Path<Uuid>) -> Result<Response> {
    // Query students that belong to these units
    let students = AcademicStudentMasterStudent::Entity::find()
        .filter(AcademicStudentMasterStudent::Column::DeletedAt.is_null())
        .filter(AcademicStudentMasterStudent::Column::Id.eq(id))
        .one(&ctx.db)
        .await?;
    format::json(students)
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

pub fn routes(ctx: &AppContext) -> Routes {
    Routes::new()
        .prefix("api/academic/student/master/students")
        .add(
            "/student_validation/{id}",
            get(student_validation).layer(AuthenticatedLayer::new(ctx.clone())),
        )
        .add(
            "/index_institution/{institution_id}",
            get(index_institution).layer(AuthenticatedLayer::new(ctx.clone())),
        )
        .add("/{id}", get(show))
        .add("/{id}", delete(destroy))
        .add("/{id}", post(update))
        .add("/list", get(list))
        .add("/", get(index))
        .add("/", post(store))
}

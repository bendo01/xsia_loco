use crate::models::academic::campaign::transaction::activities::_entities::activities as AcademicCampaignTransactionActivity;
use crate::models::academic::campaign::transaction::class_codes::_entities::class_codes as AcademicCampaignTransactionClassCode;
use crate::models::academic::campaign::transaction::teach_decrees::_entities::teach_decrees as AcademicCampaignTransactionTeachDecree;
use crate::models::academic::campaign::transaction::teaches::_entities::teaches as AcademicCampaignTransactionTeach;
use crate::models::academic::course::master::courses::_entities::courses as AcademicCourseMasterCourse;
use crate::models::academic::course::master::curriculum_details::_entities::curriculum_details as AcademicCourseMasterCurriculumDetail;
use crate::models::academic::course::master::curriculums::_entities::curriculums as AcademicCourseMasterCurriculum;
use crate::models::academic::course::reference::semesters::_entities::semesters as AcademicCourseReferenceSemester;
use crate::models::academic::general::reference::academic_years::_entities::academic_years as AcademicGeneralReferenceAcademicYear;
use crate::models::academic::student::campaign::activities::_entities::activities as AcademicStudentCampaignActivity;
use crate::models::academic::student::campaign::detail_activities::_entities::detail_activities as AcademicStudentCampaignDetailActivity;
use crate::models::academic::student::master::students::_entities::students as AcademicStudentMasterStudent;
use crate::models::institution::master::institutions::_entities::institutions as AcademicInstitutionMasterInstitution;
use crate::models::institution::master::units::_entities::units as AcademicInstitutionMasterUnit;

use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct GenerateTeachParams {
    pub class_code_id: Uuid,
    pub activity_id: Uuid,
    pub course_id: Uuid,
    pub unit_id: Uuid,
}

pub struct DataGenerator;
impl DataGenerator {
    async fn teach(
        app_context: &AppContext,
        params: GenerateTeachParams,
    ) -> Result<AcademicCampaignTransactionTeach::Model, Error> {
        // Implementation of generate_teach function
        // find teach decree
        let teach_decree_result = AcademicCampaignTransactionTeachDecree::Entity::find()
            .filter(AcademicCampaignTransactionTeachDecree::Column::DeletedAt.is_null())
            .filter(
                AcademicCampaignTransactionTeachDecree::Column::ActivityId.eq(params.activity_id),
            )
            .order_by_asc(AcademicCampaignTransactionTeachDecree::Column::CreatedAt)
            .one(&app_context.db)
            .await;
        // Then handle the Result
        let teach_decree_opt = match teach_decree_result {
            Ok(opt) => opt,
            Err(db_err) => {
                return Err(Error::Message(format!(
                    "Database error while querying activity: {db_err}"
                )));
            }
        };

        let Some(teach_decree) = teach_decree_opt else {
            let params_activity_id = params.activity_id.to_string();
            return Err(Error::Message(format!(
                "teach decree not found with activity ID: {params_activity_id}"
            )));
        };

        let uuidv7_string = uuid7::uuid7().to_string();
        let pk_id = Uuid::parse_str(&uuidv7_string).expect("Invalid UUID format");
        let data = AcademicCampaignTransactionTeach::ActiveModel {
            id: Set(pk_id),
            name: Set(Some("None".to_string())),
            max_member: Set(30),
            class_code_id: Set(params.class_code_id),
            activity_id: Set(params.activity_id),
            course_id: Set(params.course_id),
            teach_decree_id: Set(teach_decree.id),
            ..Default::default()
        };
        match data.insert(&app_context.db).await {
            Ok(inserted_model) => {
                println!("Teach Activity inserted successfully");
                Ok(inserted_model)
            }
            Err(_e) => Err(Error::Message(
                "Database error while inserting Teach activity".to_string(),
            )),
        }
    }
}

pub struct RegenerateAllStudentDetailActivities;
#[async_trait]
impl Task for RegenerateAllStudentDetailActivities {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "RegenerateAllStudentDetailActivities".to_string(),
            detail: "regenerate student detail activities".to_string(),
        }
    }

    #[allow(clippy::too_many_lines)]
    async fn run(&self, app_context: &AppContext, vars: &task::Vars) -> Result<(), Error> {
        let mut unit_activity_id: Uuid =
            Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap();
        let mut student_register_academic_year_id: Uuid =
            Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap();
        let mut curriculum_id: Uuid =
            Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap();
        let mut semester_id: Uuid =
            Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap();
        let unit_activity_id_string = vars.cli_arg("unit_activity_id");
        let student_register_academic_year_id_string =
            vars.cli_arg("student_register_academic_year_id");
        let curriculum_semester = vars.cli_arg("semester_id");
        let current_curriculum = vars.cli_arg("curriculum_id");

        // start parse args to uuid with proper error handling
        if let Ok(id_str) = unit_activity_id_string {
            unit_activity_id = Uuid::parse_str(id_str.as_str()).map_err(|e| {
                Error::string(&format!("Invalid unit_activity_id UUID format: {}", e))
            })?;
        }

        if let Ok(id_str) = student_register_academic_year_id_string {
            student_register_academic_year_id = Uuid::parse_str(id_str.as_str()).map_err(|e| {
                Error::string(&format!(
                    "Invalid student_register_academic_year_id UUID format: {}",
                    e
                ))
            })?;
        }

        if let Ok(id_str) = curriculum_semester {
            semester_id = Uuid::parse_str(id_str.as_str())
                .map_err(|e| Error::string(&format!("Invalid semester_id UUID format: {}", e)))?;
        }

        if let Ok(id_str) = current_curriculum {
            curriculum_id = Uuid::parse_str(id_str.as_str())
                .map_err(|e| Error::string(&format!("Invalid curriculum_id UUID format: {}", e)))?;
        }
        // end parse args to uuid

        // find curriculum
        let curriculum_result = AcademicCourseMasterCurriculum::Entity::find()
            .filter(AcademicCourseMasterCurriculum::Column::DeletedAt.is_null())
            .filter(AcademicCourseMasterCurriculum::Column::Id.eq(curriculum_id))
            .one(&app_context.db)
            .await;

        // Then handle the Result
        let curriculum_opt = match curriculum_result {
            Ok(opt) => opt,
            Err(db_err) => {
                return Err(Error::Message(format!(
                    "Database error while querying activity: {db_err}"
                )));
            }
        };

        let Some(curriculum) = curriculum_opt else {
            return Err(Error::Message(format!(
                "curriculum not found with ID: {curriculum_id}"
            )));
        };

        // find semester
        let semester_result = AcademicCourseReferenceSemester::Entity::find()
            .filter(AcademicCourseReferenceSemester::Column::DeletedAt.is_null())
            .filter(AcademicCourseReferenceSemester::Column::Id.eq(semester_id))
            .one(&app_context.db)
            .await;

        // Then handle the Result
        let semester_opt = match semester_result {
            Ok(opt) => opt,
            Err(db_err) => {
                return Err(Error::Message(format!(
                    "Database error while querying activity: {db_err}"
                )));
            }
        };

        let Some(semester) = semester_opt else {
            return Err(Error::Message(format!(
                "Unit activity not found with ID: {semester_id}"
            )));
        };

        // find unit activity
        let unit_activity_result = AcademicCampaignTransactionActivity::Entity::find()
            .filter(AcademicCampaignTransactionActivity::Column::DeletedAt.is_null())
            .filter(AcademicCampaignTransactionActivity::Column::Id.eq(unit_activity_id))
            .one(&app_context.db)
            .await;

        // Then handle the Result
        let unit_activity_opt = match unit_activity_result {
            Ok(opt) => opt,
            Err(db_err) => {
                return Err(Error::Message(format!(
                    "Database error while querying activity: {db_err}"
                )));
            }
        };

        let Some(unit_activity) = unit_activity_opt else {
            return Err(Error::Message(format!(
                "Unit activity not found with ID: {unit_activity_id}"
            )));
        };

        // find unit
        let unit_result = AcademicInstitutionMasterUnit::Entity::find()
            .filter(AcademicInstitutionMasterUnit::Column::DeletedAt.is_null())
            .filter(AcademicInstitutionMasterUnit::Column::Id.eq(unit_activity.unit_id))
            .one(&app_context.db)
            .await;
        // Then handle the Result
        let unit_opt = match unit_result {
            Ok(opt) => opt,
            Err(db_err) => {
                return Err(Error::Message(format!(
                    "Database error while querying unit: {db_err}"
                )));
            }
        };

        let Some(unit) = unit_opt else {
            return Err(Error::Message(format!(
                "Unit unit not found with unit activity ID: {unit_activity_id}"
            )));
        };

        // find institution
        let institution_result = AcademicInstitutionMasterInstitution::Entity::find()
            .filter(AcademicInstitutionMasterInstitution::Column::DeletedAt.is_null())
            .filter(AcademicInstitutionMasterInstitution::Column::Id.eq(unit.institution_id))
            .one(&app_context.db)
            .await;
        // Then handle the Result
        let institution_opt = match institution_result {
            Ok(opt) => opt,
            Err(db_err) => {
                return Err(Error::Message(format!(
                    "Database error while querying unit: {db_err}"
                )));
            }
        };

        let Some(institution) = institution_opt else {
            return Err(Error::Message(format!(
                "institution not found with unit activity ID: {unit_activity_id}"
            )));
        };

        // find acaddmic year
        let academic_year_result = AcademicGeneralReferenceAcademicYear::Entity::find()
            .filter(AcademicGeneralReferenceAcademicYear::Column::DeletedAt.is_null())
            .filter(
                AcademicGeneralReferenceAcademicYear::Column::Id.eq(unit_activity.academic_year_id),
            )
            .one(&app_context.db)
            .await;
        // Then handle the Result
        let academic_year_opt = match academic_year_result {
            Ok(opt) => opt,
            Err(db_err) => {
                return Err(Error::Message(format!(
                    "Database error while querying unit: {db_err}"
                )));
            }
        };

        let Some(academic_year) = academic_year_opt else {
            return Err(Error::Message(format!(
                "academic year not found with unit activity ID: {unit_activity_id}"
            )));
        };

        // find all curriculum courses
        let curriculum_details_result = AcademicCourseMasterCurriculumDetail::Entity::find()
            .filter(AcademicCourseMasterCurriculumDetail::Column::DeletedAt.is_null())
            .filter(AcademicCourseMasterCurriculumDetail::Column::CurriculumId.eq(curriculum.id))
            .filter(AcademicCourseMasterCurriculumDetail::Column::SemesterId.eq(semester.id))
            .order_by_asc(AcademicCourseMasterCurriculumDetail::Column::Code)
            .all(&app_context.db)
            .await;

        // Then handle the Result
        let curriculum_details_opt = match curriculum_details_result {
            Ok(opt) => opt,
            Err(db_err) => {
                return Err(Error::Message(format!(
                    "Database error while querying activity: {db_err}"
                )));
            }
        };

        let curriculum_details = curriculum_details_opt;
        if curriculum_details.is_empty() {
            return Err(Error::Message(format!(
                "No curriculum details found for curriculum with ID: {}",
                curriculum.id
            )));
        }

        let student_results = AcademicStudentMasterStudent::Entity::find()
            .filter(AcademicStudentMasterStudent::Column::DeletedAt.is_null())
            .filter(AcademicStudentMasterStudent::Column::UnitId.eq(unit_activity.unit_id))
            .filter(
                AcademicStudentMasterStudent::Column::AcademicYearId
                    .eq(student_register_academic_year_id),
            )
            .order_by_asc(AcademicStudentMasterStudent::Column::Code)
            .all(&app_context.db)
            .await;

        // Then handle the Result
        let students = match student_results {
            Ok(students) => students,
            Err(db_err) => {
                return Err(Error::Message(format!(
                    "Database error while querying activity: {db_err}"
                )));
            }
        };

        if students.is_empty() {
            return Err(Error::Message(format!(
                "No students found for students with unit ID: {}",
                unit_activity.unit_id
            )));
        }

        // Now loop through the students
        for student in students {
            if student.class_code_id.is_nil() {
                let error_message = format!(
                    "student class code is nil and need it to assigned student ID: {} {} {}",
                    student.id, student.code, student.code
                );
                return Err(Error::Message(error_message));
            }
            // find student class code
            let student_class_code_result = AcademicCampaignTransactionClassCode::Entity::find()
                .filter(AcademicCampaignTransactionClassCode::Column::DeletedAt.is_null())
                .filter(AcademicCampaignTransactionClassCode::Column::Id.eq(student.class_code_id))
                .one(&app_context.db)
                .await;
            // Then handle the Result
            let student_class_code_opt = match student_class_code_result {
                Ok(opt) => opt,
                Err(db_err) => {
                    return Err(Error::Message(format!(
                        "Database error while querying student activity: {db_err}"
                    )));
                }
            };

            let Some(student_class_code) = student_class_code_opt else {
                let currently_student_class_code_id = student.class_code_id.to_string();
                let currently_student_id = student.id.to_string();
                return Err(Error::Message(format!(
                    "class code not found with Student Class Code ID: {currently_student_class_code_id} and student ID: {currently_student_id}"
                )));
            };

            // find student activity
            let student_activity_result = AcademicStudentCampaignActivity::Entity::find()
                .filter(AcademicStudentCampaignActivity::Column::DeletedAt.is_null())
                .filter(AcademicStudentCampaignActivity::Column::StudentId.eq(student.id))
                .filter(
                    AcademicStudentCampaignActivity::Column::UnitActivityId.eq(unit_activity.id),
                )
                .one(&app_context.db)
                .await;
            // Then handle the Result
            let student_activity_opt = match student_activity_result {
                Ok(opt) => opt,
                Err(db_err) => {
                    return Err(Error::Message(format!(
                        "Database error while querying student activity: {db_err}"
                    )));
                }
            };

            if let Some(student_activity) = student_activity_opt {
                // delete detail activity
                let detail_activity_delete_result =
                    AcademicStudentCampaignDetailActivity::Entity::delete_many()
                        .filter(
                            AcademicStudentCampaignDetailActivity::Column::ActivityId
                                .eq(student_activity.id),
                        )
                        .exec(&app_context.db)
                        .await;
                if detail_activity_delete_result.is_err() {
                    return Err(Error::Message(format!(
                        "Database error while deleting student detail activity: {}",
                        detail_activity_delete_result.err().unwrap()
                    )));
                }

                // genererate new student activitty detail activity
                let curriculum_details_clone = curriculum_details.clone();
                for curriculum_detail in curriculum_details_clone {
                    // find course data
                    let course_result = AcademicCourseMasterCourse::Entity::find()
                        .filter(AcademicCourseMasterCourse::Column::DeletedAt.is_null())
                        .filter(
                            AcademicCourseMasterCourse::Column::Id.eq(curriculum_detail.course_id),
                        )
                        .one(&app_context.db)
                        .await;
                    let course_opt = match course_result {
                        Ok(opt) => opt,
                        Err(db_err) => {
                            return Err(Error::Message(format!(
                                "Database error while querying teach activity: {db_err}"
                            )));
                        }
                    };

                    let Some(course) = course_opt else {
                        let currently_course_id = curriculum_detail.course_id.to_string();
                        return Err(Error::Message(format!(
                            "Unit activity not found course ID: {currently_course_id}"
                        )));
                    };

                    // find teach data
                    let teach_result = AcademicCampaignTransactionTeach::Entity::find()
                        .filter(AcademicCampaignTransactionTeach::Column::DeletedAt.is_null())
                        .filter(
                            AcademicCampaignTransactionTeach::Column::ClassCodeId
                                .eq(student_class_code.id),
                        )
                        .filter(AcademicCampaignTransactionTeach::Column::CourseId.eq(course.id))
                        .filter(
                            AcademicCampaignTransactionTeach::Column::ActivityId
                                .eq(unit_activity.id),
                        )
                        .one(&app_context.db)
                        .await;
                    let teach_opt = match teach_result {
                        Ok(opt) => opt,
                        Err(db_err) => {
                            return Err(Error::Message(format!(
                                "Database error while querying teach activity: {db_err}"
                            )));
                        }
                    };

                    let teach = if let Some(teach) = teach_opt {
                        teach
                    } else {
                        // Generate a new teach record when one doesn't exist
                        let params = GenerateTeachParams {
                            class_code_id: student_class_code.id,
                            activity_id: unit_activity.id,
                            course_id: course.id,
                            unit_id: unit_activity.unit_id,
                        };

                        match DataGenerator::teach(app_context, params).await {
                            Ok(generated_teach) => generated_teach,
                            Err(e) => {
                                let currently_student_class_code_id =
                                    student.class_code_id.to_string();
                                let currently_course_id = course.id.to_string();
                                let currently_unit_activity_id = unit_activity.id.to_string();
                                return Err(Error::Message(format!(
                                    "Failed to generate teach record for Class Code ID: {currently_student_class_code_id}, course ID: {currently_course_id}, and Unit Activity ID: {currently_unit_activity_id}. Error: {e}"
                                )));
                            }
                        }
                    };

                    // create student activity detail activity
                    let detail_activity_name = format!(
                        "{}_{}_{}_{}_{}",
                        institution.clone().code,
                        unit.clone().code,
                        student.clone().code,
                        academic_year.clone().feeder_name,
                        course.clone().code
                    );

                    let uuidv7_string = uuid7::uuid7().to_string();
                    let pk_id = Uuid::parse_str(&uuidv7_string).expect("Invalid UUID format");
                    let data = AcademicStudentCampaignDetailActivity::ActiveModel {
                        id: Set(pk_id),
                        name: Set(Some(detail_activity_name.clone())),
                        credit: Set(course.clone().total_credit),
                        curiculum_detail_sequence: Set(curriculum_detail.clone().code),
                        course_id: Set(course.clone().id),
                        activity_id: Set(student_activity.clone().id),
                        teach_id: Set(teach.clone().id),
                        ..Default::default()
                    };

                    let inserted = data.insert(&app_context.db).await;
                    match inserted {
                        Ok(_) => {
                            println!(
                                "Detail activity {detail_activity_name} inserted successfully"
                            );
                        }
                        Err(_e) => {
                            return Err(Error::Message(format!(
                                "Database error while inserting detail activity: {detail_activity_name}"
                            )));
                        }
                    }
                }
            } else {
                println!(
                    "Unit activity not found student with Unit Activity ID: {} and student ID: {}",
                    unit_activity.id, student.id
                );
            }
        }
        Ok(())
    }
}

use crate::models::academic::campaign::transaction::activities::_entities::activities as AcademicCampaignTransactionActivity;
use crate::models::academic::general::reference::academic_years::_entities::academic_years as AcademicGeneralReferenceAcademicYear;
use crate::models::academic::student::campaign::activities::_entities::activities as AcademicStudentCampaignActivity;
use crate::models::academic::student::master::students::_entities::students as AcademicStudentMasterStudent;
use crate::models::institution::master::units::data_objects::DataObject as InstitutionMasterUnitDataObject;
use chrono::Local;
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder, Set};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::{Uuid, uuid};

pub struct ChekStudentCampaignActivityParams {
    pub student: AcademicStudentMasterStudent::Model,
    pub unit: InstitutionMasterUnitDataObject,
    pub unit_activity: AcademicCampaignTransactionActivity::Model,
    pub academic_year: AcademicGeneralReferenceAcademicYear::Model,
    pub fee: f64,
}

pub struct DataGenerator;
impl DataGenerator {
    async fn upsert(
        app_context: &AppContext,
        params: ChekStudentCampaignActivityParams,
    ) -> Result<AcademicStudentCampaignActivity::Model, Error> {
        let existing_student_campaign_activity = AcademicStudentCampaignActivity::Entity::find()
            .filter(AcademicStudentCampaignActivity::Column::DeletedAt.is_null())
            .filter(
                AcademicStudentCampaignActivity::Column::StudentId.eq(params.student.id.clone()),
            )
            .filter(
                AcademicStudentCampaignActivity::Column::UnitActivityId.eq(params.unit_activity.id),
            )
            .one(&app_context.db)
            .await
            .map_err(|e| {
                Error::Message(format!(
                    "Database error while checking for existing activity: {e}"
                ))
            })?;
        if let Some(student_campaign_activity) = existing_student_campaign_activity {
            // Activity exists, return it
            Ok(student_campaign_activity)
        } else {
            let now = Local::now().naive_local();
            let uuidv7_string = uuid7::uuid7().to_string();
            let pk_id = Uuid::parse_str(&uuidv7_string).expect("Invalid UUID format");
            let default_value_uuid: Uuid = uuid!("00000000-0000-0000-0000-000000000000");
            let default_status_uuid: Uuid = uuid!("71579672-37b7-455c-9353-02ea5810cf8b");

            let new_student_campaign_activity = AcademicStudentCampaignActivity::ActiveModel {
                id: Set(pk_id),
                name: Set(format!(
                    "StudentActivity {} {} {} {}",
                    params.unit.institution.unwrap().code,
                    params.unit.unit.code,
                    params.student.code.clone(),
                    params.academic_year.feeder_name
                )),
                cumulative_index: Set(0.0),
                grand_cumulative_index: Set(0.0),
                total_credit: Set(0.0),
                grand_total_credit: Set(0.0),
                student_id: Set(params.student.id.clone()),
                unit_activity_id: Set(params.unit_activity.id.clone()),
                status_id: Set(default_status_uuid),
                resign_status_id: Set(default_value_uuid.clone()),
                unit_id: Set(params.unit_activity.unit_id.clone()),
                feeder_id: Set(default_value_uuid.clone()),
                finance_id: Set(default_value_uuid.clone()),
                finance_fee: Set(params.fee.clone()),
                is_lock: Set(false),
                created_at: Set(Some(now)),
                updated_at: Set(Some(now)),
                sync_at: Set(None),
                deleted_at: Set(None),
                created_by: Set(None),
                updated_by: Set(None),
            };

            let student_campaign_activity =
                AcademicStudentCampaignActivity::Entity::insert(new_student_campaign_activity)
                    .exec(&app_context.db)
                    .await
                    .map_err(|e| Error::Message(format!("Failed to insert new activity: {e}")))?;

            // Fetch the newly created activity
            let student_campaign_activity_model =
                AcademicStudentCampaignActivity::Entity::find_by_id(
                    student_campaign_activity.last_insert_id,
                )
                .one(&app_context.db)
                .await
                .map_err(|e| {
                    Error::Message(format!("Failed to fetch newly created activity: {e}"))
                })?
                .ok_or_else(|| Error::Message("Newly created activity not found".to_string()))?;

            Ok(student_campaign_activity_model)
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GenerateUnitStudentCampaignActivityParams {
    pub unit_id: Uuid,
    pub academic_year_id: Uuid,
    pub student_academic_year_id: Uuid,
}

pub struct GenerateUnitStudentCampaignActivities;
#[async_trait]
impl Task for GenerateUnitStudentCampaignActivities {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "GenerateUnitStudentCampaignActivities".to_string(),
            detail: "Generate Unit Student Campaign Activities".to_string(),
        }
    }

    #[allow(clippy::too_many_lines)]
    async fn run(&self, app_context: &AppContext, vars: &task::Vars) -> Result<(), Error> {
        let mut unit_id: Uuid = Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap();
        let mut academic_year_id: Uuid =
            Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap();
        let mut student_academic_year_id: Uuid =
            Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap();
        let mut fee: f64 = 0.0;
        
        let unit_id_string = vars.cli_arg("unit_id");
        let academic_year_id_string = vars.cli_arg("academic_year_id");
        let student_academic_year_id_string = vars.cli_arg("student_academic_year_id");
        let fee_string = vars.cli_arg("fee");

        if let Ok(unit_id_str) = unit_id_string {
            unit_id = Uuid::parse_str(&unit_id_str)
                .map_err(|e| Error::Message(format!("Invalid unit_id UUID format: {}", e)))?;
        }

        if let Ok(academic_year_id_str) = academic_year_id_string {
            academic_year_id = Uuid::parse_str(&academic_year_id_str)
                .map_err(|e| Error::Message(format!("Invalid academic_year_id UUID format: {}", e)))?;
        }

        if let Ok(student_academic_year_id_str) = student_academic_year_id_string {
            student_academic_year_id = Uuid::parse_str(&student_academic_year_id_str)
                .map_err(|e| Error::Message(format!("Invalid student_academic_year_id UUID format: {}", e)))?;
        }

        if let Ok(fee_str) = fee_string {
            fee = f64::from_str(&fee_str)
                .map_err(|e| Error::Message(format!("Invalid fee format: {}", e)))?;
        }

        // find academic_year
        let academic_year_result = AcademicGeneralReferenceAcademicYear::Entity::find()
            .filter(AcademicGeneralReferenceAcademicYear::Column::DeletedAt.is_null())
            .filter(AcademicGeneralReferenceAcademicYear::Column::Id.eq(academic_year_id))
            .one(&app_context.db)
            .await;

        let academic_year_opt = match academic_year_result {
            Ok(ref opt) => opt,
            Err(db_err) => {
                return Err(Error::Message(format!(
                    "Database error while querying activity: {db_err}"
                )));
            }
        };

        let Some(academic_year) = academic_year_opt else {
            return Err(Error::Message(format!(
                "academic_year not found with ID: {academic_year_id}"
            )));
        };

        // find student academic year
        let student_academic_year_result = AcademicGeneralReferenceAcademicYear::Entity::find()
            .filter(AcademicGeneralReferenceAcademicYear::Column::DeletedAt.is_null())
            .filter(AcademicGeneralReferenceAcademicYear::Column::Id.eq(student_academic_year_id))
            .one(&app_context.db)
            .await;

        let student_academic_year_opt = match student_academic_year_result {
            Ok(ref opt) => opt,
            Err(db_err) => {
                return Err(Error::Message(format!(
                    "Database error while querying activity: {db_err}"
                )));
            }
        };

        let Some(student_academic_year) = student_academic_year_opt else {
            return Err(Error::Message(format!(
                "student_academic_year not found with ID: {academic_year_id}"
            )));
        };

        // find unit
        let unit_result =
            InstitutionMasterUnitDataObject::get_by_id(app_context, unit_id, false).await?;
        let Some(ref unit_data) = unit_result else {
            return Err(Error::Message(format!("unit not found with ID: {unit_id}")));
        };

        // find unit activity
        let unit_activity_result = AcademicCampaignTransactionActivity::Entity::find()
            .filter(AcademicCampaignTransactionActivity::Column::UnitId.eq(unit_id))
            .filter(
                AcademicCampaignTransactionActivity::Column::AcademicYearId.eq(academic_year_id),
            )
            .filter(AcademicCampaignTransactionActivity::Column::DeletedAt.is_null())
            .one(&app_context.db)
            .await;
        let unit_activity_opt = match unit_activity_result {
            Ok(ref opt) => opt,
            Err(db_err) => {
                return Err(Error::Message(format!(
                    "Database error while querying activity: {db_err}"
                )));
            }
        };

        let Some(unit_activity) = unit_activity_opt else {
            return Err(Error::Message(format!(
                "unit_activity not found with ID: {academic_year_id}"
            )));
        };
        // println!("academic_year: {:#?}", academic_year);
        // println!("student_academic_year: {:#?}", student_academic_year);
        // println!("unit_result: {:#?}", unit_result);
        // println!("unit_activity: {:#?}", unit_activity);

        // find students
        let student_result = AcademicStudentMasterStudent::Entity::find()
            .filter(AcademicStudentMasterStudent::Column::DeletedAt.is_null())
            .filter(AcademicStudentMasterStudent::Column::UnitId.eq(unit_data.unit.id))
            .filter(
                AcademicStudentMasterStudent::Column::AcademicYearId.eq(student_academic_year.id),
            )
            .order_by_asc(AcademicStudentMasterStudent::Column::Code)
            .all(&app_context.db)
            .await;
        let students = match student_result {
            Ok(students) => students,
            Err(db_err) => {
                return Err(Error::Message(format!(
                    "Database error while querying students: {db_err}"
                )));
            }
        };

        if students.is_empty() {
            return Err(Error::Message(format!("students not found")));
        }

        for student in students {
            // println!("student: {:#?}", student);
            let params = ChekStudentCampaignActivityParams {
                student: student.clone(),
                unit: unit_data.clone(),
                unit_activity: unit_activity.clone(),
                fee: fee,
                academic_year: academic_year.clone(),
            };

            if let Err(err) = DataGenerator::upsert(app_context, params).await {
                eprintln!("Error processing unit {}: {}", student.code, err);
            }
        }

        Ok(())
    }
}

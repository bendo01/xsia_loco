use crate::models::academic::campaign::transaction::activities::_entities::activities as AcademicCampaignTransactionActivity;
use crate::models::academic::general::reference::academic_years::_entities::academic_years as AcademicGeneralReferenceAcademicYear;
use crate::models::institution::master::units::_entities::units as InstitutionMasterUnit;
use crate::models::institution::master::units::data_objects::DataObject as InstitutionMasterUnitDataObject;

use chrono::Local;
use loco_rs::prelude::*;
// use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder, Set};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use uuid::{Uuid, uuid};

#[derive(Debug, Deserialize, Serialize)]
pub struct GenerateUnitActivityParams {
    pub institution_id: Uuid,
    pub academic_year_id: Uuid,
}

pub struct ChekUnitActivityParams {
    pub unit_object: InstitutionMasterUnitDataObject,
    pub academic_year: AcademicGeneralReferenceAcademicYear::Model,
}

pub struct DataGenerator;
impl DataGenerator {
    async fn upsert(
        app_context: &AppContext,
        params: ChekUnitActivityParams,
    ) -> Result<AcademicCampaignTransactionActivity::Model, Error> {
        // println!("Unit Object: {:?}", params.unit_object);
        // println!("Academic Year: {:?}", params.academic_year);

        // Check if an activity already exists for this unit and academic year
        let existing_activity = AcademicCampaignTransactionActivity::Entity::find()
            .filter(
                AcademicCampaignTransactionActivity::Column::UnitId.eq(params.unit_object.unit.id),
            )
            .filter(
                AcademicCampaignTransactionActivity::Column::AcademicYearId
                    .eq(params.academic_year.id),
            )
            .filter(AcademicCampaignTransactionActivity::Column::DeletedAt.is_null())
            .one(&app_context.db)
            .await
            .map_err(|e| {
                Error::Message(format!(
                    "Database error while checking for existing activity: {e}"
                ))
            })?;

        if let Some(activity) = existing_activity {
            // Activity exists, return it
            Ok(activity)
        } else {
            // Create a new activity
            let now = Local::now().naive_local();
            let uuidv7_string = uuid7::uuid7().to_string();
            let pk_id = Uuid::parse_str(&uuidv7_string).expect("Invalid UUID format");
            let new_activity = AcademicCampaignTransactionActivity::ActiveModel {
                id: Set(pk_id),
                name: Set(format!(
                    "Activity {} {} {}",
                    params.unit_object.institution.unwrap().code,
                    params.unit_object.unit.code,
                    params.academic_year.feeder_name
                )),
                unit_id: Set(params.unit_object.unit.id),
                academic_year_id: Set(params.academic_year.id),
                week_quantity: Set(0),
                student_target: Set(0),
                candidate_number: Set(0),
                candidate_pass: Set(0),
                became_student: Set(0),
                transfer_student: Set(0),
                total_class_member: Set(0),
                start_date: Set(params.academic_year.start_date.clone()),
                end_date: Set(params.academic_year.end_date.clone()),
                start_transaction: Set(params.academic_year.start_date.clone()),
                end_transaction: Set(None),
                is_active: Set(true),
                feeder_id: Set(None),
                created_at: Set(Some(now)),
                updated_at: Set(Some(now)),
                sync_at: Set(None),
                deleted_at: Set(None),
                created_by: Set(None),
                updated_by: Set(None),
            };

            let activity = AcademicCampaignTransactionActivity::Entity::insert(new_activity)
                .exec(&app_context.db)
                .await
                .map_err(|e| Error::Message(format!("Failed to insert new activity: {e}")))?;

            // Fetch the newly created activity
            let activity_model =
                AcademicCampaignTransactionActivity::Entity::find_by_id(activity.last_insert_id)
                    .one(&app_context.db)
                    .await
                    .map_err(|e| {
                        Error::Message(format!("Failed to fetch newly created activity: {e}"))
                    })?
                    .ok_or_else(|| {
                        Error::Message("Newly created activity not found".to_string())
                    })?;

            Ok(activity_model)
        }
    }
}

pub struct GenerateInstitutionUnitCampaignActivities;
#[async_trait]
impl Task for GenerateInstitutionUnitCampaignActivities {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "GenerateInstitutionUnitCampaignActivities".to_string(),
            detail: "generate institution unit campaign activities".to_string(),
        }
    }

    #[allow(clippy::too_many_lines)]
    async fn run(&self, app_context: &AppContext, vars: &task::Vars) -> Result<(), Error> {
        let mut institution_id: Uuid =
            Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap();
        let mut academic_year_id: Uuid =
            Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap();

        let institution_id_string = vars.cli_arg("institution_id");
        let academic_year_id_string = vars.cli_arg("academic_year_id");

        if institution_id_string.is_ok() {
            institution_id = Uuid::parse_str(institution_id_string.unwrap().as_str()).unwrap();
        }

        if academic_year_id_string.is_ok() {
            academic_year_id = Uuid::parse_str(academic_year_id_string.unwrap().as_str()).unwrap();
        }

        // find academic_year
        let academic_year_result = AcademicGeneralReferenceAcademicYear::Entity::find()
            .filter(AcademicGeneralReferenceAcademicYear::Column::DeletedAt.is_null())
            .filter(AcademicGeneralReferenceAcademicYear::Column::Id.eq(academic_year_id))
            .one(&app_context.db)
            .await;

        let academic_year_opt = match academic_year_result {
            Ok(opt) => opt,
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

        // println!("Academic Year: {:?}", academic_year);

        // find institution units
        let unit_type_id: Uuid = uuid!("019759fd-36e8-4f43-80ed-4f687a48145d");
        let unit_result = InstitutionMasterUnit::Entity::find()
            .filter(InstitutionMasterUnit::Column::DeletedAt.is_null())
            .filter(InstitutionMasterUnit::Column::InstitutionId.eq(institution_id))
            .filter(InstitutionMasterUnit::Column::UnitTypeId.eq(unit_type_id))
            .all(&app_context.db)
            .await;

        let units = match unit_result {
            Ok(units) => units,
            Err(db_err) => {
                return Err(Error::Message(format!(
                    "Database error while querying activity: {db_err}"
                )));
            }
        };

        if units.is_empty() {
            return Err(Error::Message(format!(
                "units not found with Institution ID: {institution_id} and type {unit_type_id}"
            )));
        }

        for unit in units {
            // println!("Unit: {:?}", unit);
            // upsert campaign activities for each unit
            let data_result =
                InstitutionMasterUnitDataObject::get_by_id(app_context, unit.id, false).await;
            match data_result {
                Ok(unit_data) => {
                    let params = ChekUnitActivityParams {
                        unit_object: unit_data.clone().expect("Error on Unit Data"),
                        academic_year: academic_year.clone(),
                    };

                    if let Err(err) = DataGenerator::upsert(app_context, params).await {
                        eprintln!("Error processing unit {}: {}", unit.id, err);
                    }
                    // println!("{:?}", unit_data.clone());
                }
                Err(err) => {
                    eprintln!("Error fetching unit data for {}: {}", unit.id, err);
                }
            }
        }

        Ok(())
    }
}

use loco_rs::prelude::*;
use crate::common::settings::Settings;
use uuid::Uuid;
use chrono::Local;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use crate::models::feeder::akumulasi::estimasi::_entities::estimasi as FeederAkumulasiEstimasi;
use crate::tasks::feeder_dikti::downstream::request_only_data::{InputRequestData, RequestData};
use crate::models::feeder::master::evaluasi_kelas::_entities::evaluasi_kelas::FeederDataModel;

#[derive(Deserialize, Debug, Serialize)]
pub struct WorkerArgs {
    pub act: String,
    pub filter: Option<String>,
    pub order: Option<String>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

pub struct EstimateKomponenEvaluasiKelas;

#[async_trait]
impl Task for EstimateKomponenEvaluasiKelas {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "EstimateKomponenEvaluasiKelas".to_string(),
            detail: "Task generator".to_string(),
        }
    }

    async fn run(&self, app_context: &AppContext, _vars: &task::Vars) -> Result<()> {
        println!("Task EstimateKomponenEvaluasiKelas generated");
        // Here you can add the logic to enqueue specific workers or perform actions
        // related to estimating evaluation components for classes.
        let institution_id: Uuid = if let Some(current_settings) = &app_context.config.settings {
            // println!("Settings loaded");
            let settings = Settings::from_json(current_settings)?;
            match Uuid::parse_str(settings.current_institution_id.as_str()) {
                Ok(parsed_id) => {
                    // println!("Successfully parsed institution id");
                    parsed_id
                }
                Err(_) => {
                    println!("Failed to parse institution id");
                    return Err(Error::Message("Invalid institution ID format".to_string()));
                }
            }
        } else {
            return Err(Error::Message("Setting not loaded".to_string()));
        };

        let data_result = FeederAkumulasiEstimasi::Entity::find()
            .filter(FeederAkumulasiEstimasi::Column::DeletedAt.is_null())
            .filter(FeederAkumulasiEstimasi::Column::InstitutionId.eq(institution_id))
            .filter(FeederAkumulasiEstimasi::Column::Name.eq("EstimateKomponenEvaluasiKelas".to_string()))
            .one(&app_context.db)
            .await;

        // Then handle the Result
        let data_opt = match data_result {
            Ok(opt) => opt,
            Err(db_err) => {
                return Err(Error::Message(format!(
                    "Database error while querying reference: {db_err}"
                )));
            }
        };

            // determine limit from existing reference or use default
            let mut limit: i32 = 100;

            if let Some(existing_reference) = data_opt {
            // Reference already exists — reset offsets and persist the change.
                let existing_id = existing_reference.id.clone();
                // read the configured page size from the existing reference
                limit = existing_reference.total_data_per_request;
                let mut active: FeederAkumulasiEstimasi::ActiveModel = existing_reference.into_active_model();
                active.last_offset = Set(0);
                active.total_data = Set(0);
                active.updated_at = Set(Local::now().naive_local());
            // optional: update audit fields if present
            match active.update(&app_context.db).await {
                Ok(_) => {
                    println!("EstimateKomponenEvaluasiKelas reference reset (id={})", existing_id);
                    // continue to perform requests below
                }
                Err(err) => {
                    return Err(Error::Message(format!("Failed to update existing reference: {err}")));
                }
            }
        } else {
            // println!("There is No Data Provided");
            let uuidv7_string = uuid7::uuid7().to_string();
            let pk_id = Uuid::parse_str(&uuidv7_string).expect("Invalid UUID format");
            let input = FeederAkumulasiEstimasi::ActiveModel {
                id: Set(pk_id),
                institution_id: Set(institution_id),
                name: Set("EstimateKomponenEvaluasiKelas".to_string()),
                total_data_per_request: Set(100),
                last_offset: Set(0),
                total_data: Set(0),
                ..Default::default()
            };
            match input.insert(&app_context.db).await {
                Ok(_) => {
                    println!("Inserted EstimateKomponenEvaluasiKelas reference for institution {institution_id}");
                    println!("EstimateKomponenEvaluasiKelas reference created - will start requesting data");
                    // default limit already set to 100 above; continue to perform requests below
                }
                Err(err) => {
                    return Err(Error::Message(format!(
                        "Failed to insert new reference data: {err}"
                    )));
                }
            }
        }

        // request to feeder dikti api to get data in a loop using pagination
        // use the AppContext passed into the task (app_context) when making requests
        let mut offset: i32 = 0;

        loop {
            let request_result = RequestData::get::<FeederDataModel>(app_context, InputRequestData {
                act: "GetListKomponenEvaluasiKelas".to_string(),
                filter: None,
                order: None,
                limit: Some(limit),
                offset: Some(offset),
            }).await;

            // Unwrap response or return on error so subsequent match arms are consistent
            let response = match request_result {
                Ok(resp) => resp,
                Err(err) => {
                    println!("Error requesting feeder data: {err}");
                    return Err(Error::Message(format!("Failed requesting feeder: {err}")));
                }
            };

            match response.data {
                Some(vec) if !vec.is_empty() => {
                    let received = vec.len() as i32;
                    println!("Received {} records (offset={})", received, offset);
                    // Enqueue a worker to upsert this page
                    
                    // let worker_args = crate::workers::feeder_dikti::downstream::master::upsert::get_list_komponen_evaluasi_kelas::WorkerArgs {
                    //     limit: Some(limit),
                    //     offset: Some(offset),
                    // };

                    // perform_later returns Result<(), Error>
                    // match crate::workers::feeder_dikti::downstream::master::upsert::get_list_komponen_evaluasi_kelas::Worker::perform_later(app_context, worker_args).await {
                    //     Ok(_) => println!("✅ Enqueued upsert worker for limit {} offset {}", limit, offset),
                    //     Err(err) => eprintln!("❌ Failed to enqueue upsert worker for limit {} offset {}: {:?}", limit, offset, err),
                    // }

                    // Persist progress to FeederAkumulasiEstimasi: update last_offset and total_data
                    let update_result = FeederAkumulasiEstimasi::Entity::find()
                        .filter(FeederAkumulasiEstimasi::Column::DeletedAt.is_null())
                        .filter(FeederAkumulasiEstimasi::Column::InstitutionId.eq(institution_id))
                        .filter(FeederAkumulasiEstimasi::Column::Name.eq("EstimateKomponenEvaluasiKelas".to_string()))
                        .one(&app_context.db)
                        .await;

                    if let Ok(Some(mut record)) = update_result {
                        let current_total = record.total_data;
                        // compute next offset as page-based stepping
                        let next_offset = offset + limit;
                        record.total_data = current_total + received;
                        record.last_offset = next_offset;
                        record.updated_at = Local::now().naive_local();
                        // capture updated values before moving `record` into an ActiveModel
                        let updated_total = record.total_data;
                        let updated_last_offset = record.last_offset;
                        let updated_updated_at = record.updated_at;
                        let mut active: FeederAkumulasiEstimasi::ActiveModel = record.into_active_model();
                        active.total_data = Set(updated_total);
                        active.last_offset = Set(updated_last_offset);
                        active.updated_at = Set(updated_updated_at);
                        if let Err(err) = active.update(&app_context.db).await {
                            eprintln!("Failed to persist progress to FeederAkumulasiEstimasi: {:?}", err);
                        }
                    } else if let Err(err) = update_result {
                        eprintln!("Failed to fetch FeederAkumulasiEstimasi to persist progress: {:?}", err);
                    }

                    // Advance offset for next page (page-based stepping)
                    offset += limit;
                    // continue loop to fetch next page
                }
                Some(_) | None => {
                    println!("No more data returned from feeder (offset={})", offset);
                    break;
                }
            }
        }

        // finished paging
        Ok(())
    }   
}
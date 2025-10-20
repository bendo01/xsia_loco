use crate::workers::feeder_dikti::downstream::master::download::get_krs_mahasiswa::{
    Worker, WorkerArgs,
};

use crate::models::feeder::akumulasi::jumlah_data::_entities::jumlah_data as FeederAkumulasiJumlahData;
use crate::common::settings::Settings;

use tokio::time::{sleep, Duration};
use loco_rs::prelude::*;
// use tokio::time::{Duration, sleep};
pub struct ExecuteWorkerGetKRSMahasiswa;
#[async_trait]
impl Task for ExecuteWorkerGetKRSMahasiswa {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "ExecuteWorkerGetKRSMahasiswa".to_string(),
            detail: "Task generator".to_string(),
        }
    }
    async fn run(&self, app_context: &AppContext, _vars: &task::Vars) -> Result<()> {
        // println!("Task ExecuteWorkerGetKRSMahasiswa generated");
        // Initialize with default UUID, will be updated from settings
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

        let data_result = FeederAkumulasiJumlahData::Entity::find()
            .filter(FeederAkumulasiJumlahData::Column::DeletedAt.is_null())
            .filter(FeederAkumulasiJumlahData::Column::InstitutionId.eq(institution_id))
            .filter(FeederAkumulasiJumlahData::Column::Name.eq("FA0024GetCountPerkuliahanMahasiswa".to_string()))
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

        if let Some(existing_reference) = data_opt {
            // calculate pagination limit offset base on data from existing_reference.total_feeder
            // example total_feeder = 680 limit is 100;
            // Enqueue the worker
            
            //  loop through based on 
            let limit = 1000;
            let total_feeder = existing_reference.total_feeder;
            for offset in (0..total_feeder).step_by(limit as usize) {
                let worker_args = WorkerArgs {
                    act: "GetKRSMahasiswa".to_string(),
                    filter: None,
                    order: Some("nim ASC".to_string()),
                    limit: Some(limit),
                    offset: Some(offset),
                };
                match Worker::perform_later(app_context, worker_args).await {
                    Ok(_) => {
                        println!("✅ Enqueued worker for GetKRSMahasiswa: limit {} offset: {}", limit, offset);
                    }
                    Err(err) => {
                        eprintln!("❌ Failed to enqueue worker for limit {} offset: {}. Error: {:?}", limit, offset, err);
                    }
                }
            }
            sleep(Duration::from_secs(20)).await;
        } else {
            println!("There is No Data Provided");
        }
        println!("Task ExecuteWorkerGetKRSMahasiswa completed - all workers enqueued");
        Ok(())
    }
}

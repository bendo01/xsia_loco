use crate::workers::feeder_dikti::downstream::master::download::get_transkrip_mahasiswa::{
    Worker, WorkerArgs,
};

use crate::common::settings::Settings;
use crate::models::feeder::akumulasi::jumlah_data::_entities::jumlah_data as FeederAkumulasiJumlahData;

use loco_rs::prelude::*;
use tokio::time::{Duration, sleep};
pub struct ExecuteWorkerGetTranskripMahasiswa;
#[async_trait]
impl Task for ExecuteWorkerGetTranskripMahasiswa {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "ExecuteWorkerGetTranskripMahasiswa".to_string(),
            detail: "Task generator".to_string(),
        }
    }
    async fn run(&self, app_context: &AppContext, _vars: &task::Vars) -> Result<()> {
        let institution_id: Uuid = if let Some(current_settings) = &app_context.config.settings {
            let settings = Settings::from_json(current_settings)?;
            match Uuid::parse_str(settings.current_institution_id.as_str()) {
                Ok(parsed_id) => parsed_id,
                Err(_) => return Err(Error::Message("Invalid institution ID format".to_string())),
            }
        } else {
            return Err(Error::Message("Setting not loaded".to_string()));
        };

        let data_result = FeederAkumulasiJumlahData::Entity::find()
            .filter(FeederAkumulasiJumlahData::Column::DeletedAt.is_null())
            .filter(FeederAkumulasiJumlahData::Column::InstitutionId.eq(institution_id))
            .filter(
                FeederAkumulasiJumlahData::Column::Name
                    .eq("FA0030GetCountTranskripMahasiswa".to_string()),
            )
            .one(&app_context.db)
            .await;

        let data_opt = match data_result {
            Ok(opt) => opt,
            Err(db_err) => {
                return Err(Error::Message(format!(
                    "Database error while querying reference: {db_err}"
                )));
            }
        };

        if let Some(existing_reference) = data_opt {
            let limit = 1000;
            let total_feeder = existing_reference.total_feeder;
            for offset in (0..total_feeder).step_by(limit as usize) {
                let worker_args = WorkerArgs {
                    act: "GetTranskripMahasiswa".to_string(),
                    filter: None,
                    order: None,
                    limit: Some(limit),
                    offset: Some(offset),
                };
                match Worker::perform_later(app_context, worker_args).await {
                    Ok(_) => println!(
                        "✅ Enqueued worker for GetTranskripMahasiswa: limit {} offset: {}",
                        limit, offset
                    ),
                    Err(err) => eprintln!(
                        "❌ Failed to enqueue worker for limit {} offset: {}. Error: {:?}",
                        limit, offset, err
                    ),
                }
            }
            sleep(Duration::from_secs(20)).await;
        } else {
            println!("There is No Data Provided");
        }
        println!("Task ExecuteWorkerGetTranskripMahasiswa completed - all workers enqueued");
        Ok(())
    }
}

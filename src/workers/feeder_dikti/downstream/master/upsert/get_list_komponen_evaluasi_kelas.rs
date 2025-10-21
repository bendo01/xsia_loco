use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use colored::Colorize;
use chrono::Local;

// use crate::tasks::feeder_dikti::downstream::request_only_data::{InputRequestData, RequestData};
use crate::models::feeder::master::komponen_evaluasi_kelas::_entities::komponen_evaluasi_kelas as FeederMasterKomponenEvaluasiKelas;
use crate::models::feeder::master::komponen_evaluasi_kelas::feeder_model::ModelInput as FeederModel;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputData;

impl InputData {
    pub async fn upsert(ctx: &AppContext, input: FeederModel) -> Result<(), Error> {
        let data_result = FeederMasterKomponenEvaluasiKelas::Entity::find()
            .filter(FeederMasterKomponenEvaluasiKelas::Column::DeletedAt.is_null())
            .filter(
                FeederMasterKomponenEvaluasiKelas::Column::IdKomponenEvaluasi
                    .eq(input.id_komponen_evaluasi),
            )
            .filter(
                FeederMasterKomponenEvaluasiKelas::Column::IdKelasKuliah.eq(input.id_kelas_kuliah),
            )
            .one(&ctx.db)
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

        // If the record exists, update it; otherwise, insert a new one
        if let Some(existing_reference) = data_opt {
            let mut reference: FeederMasterKomponenEvaluasiKelas::ActiveModel = existing_reference.into();
            reference.id_komponen_evaluasi = Set(input.id_komponen_evaluasi);
            reference.id_kelas_kuliah = Set(input.id_kelas_kuliah);
            reference.id_jenis_evaluasi = Set(input.id_jenis_evaluasi);
            reference.nomor_urut = Set(input.nomor_urut);
            reference.bobot_evaluasi = Set(input.bobot_evaluasi);
            reference.nama = Set(input.nama);
            reference.nama_inggris = Set(input.nama_inggris);
            reference.last_update = Set(input.last_update.unwrap_or(Local::now().naive_local().date())); // <- ensure NaiveDate by unwrapping or using today
            reference.tgl_create = Set(input.tgl_create.unwrap_or(Local::now().naive_local().date())); // <- Now NaiveDate, fallback to today
            match reference.update(&ctx.db).await {
                Ok(_updated_model) => {
                    println!("{}", "Data updated successfully".green());
                    Ok(())
                }
                Err(err) => {
                    return Err(Error::Message(format!("Failed to update reference: {err}")));
                }
            }
        } else {
            let uuidv7_string = uuid7::uuid7().to_string();
            let pk_id = Uuid::parse_str(&uuidv7_string).expect("Invalid UUID format");
            let new_reference = FeederMasterKomponenEvaluasiKelas::ActiveModel {
                id: Set(pk_id),
                id_komponen_evaluasi: Set(input.id_komponen_evaluasi),
                id_kelas_kuliah: Set(input.id_kelas_kuliah),
                id_jenis_evaluasi: Set(input.id_jenis_evaluasi),
                nomor_urut: Set(input.nomor_urut),
                nama: Set(input.nama),
                nama_inggris: Set(input.nama_inggris),
                bobot_evaluasi: Set(input.bobot_evaluasi),
                last_update: Set(input.last_update.unwrap_or(Local::now().naive_local().date())), // <- ensure NaiveDate by unwrapping or using today
                tgl_create: Set(input.tgl_create.unwrap_or(Local::now().naive_local().date())),   // <- NaiveDate, fallback to today
                // fields expect Option<NaiveDateTime>, wrap in Some(...)
                created_at: Set(Some(Local::now().naive_local())),
                updated_at: Set(Some(Local::now().naive_local())),
                sync_at: Set(Some(Local::now().naive_local())),
                ..Default::default()
            };
            match FeederMasterKomponenEvaluasiKelas::Entity::insert(new_reference)
                .exec(&ctx.db)
                .await
            {
                Ok(_insert_result) => {
                    println!("{}", "Data inserted successfully".green());
                    Ok(())
                }
                Err(err) => {
                    return Err(Error::Message(format!(
                        "Failed to insert new reference: {err}"
                    )));
                }
            }
        }
    }
}


pub struct Worker {
    pub ctx: AppContext,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct WorkerArgs {}

#[async_trait]
impl BackgroundWorker<WorkerArgs> for Worker {
    /// Creates a new instance of the Worker with the given application context.
    ///
    /// This function is called when registering the worker with the queue system.
    ///
    /// # Parameters
    /// * `ctx` - The application context containing shared resources
    fn build(ctx: &AppContext) -> Self {
        Self { ctx: ctx.clone() }
    }

    /// Returns the class name of the worker.
    ///
    /// This name is used when enqueueing jobs and identifying the worker in logs.
    /// The implementation returns the struct name as a string.
    fn class_name() -> String {
        "GetListKomponenEvaluasiKelas".to_string()
    }

    /// Returns tags associated with this worker.
    ///
    /// Tags can be used to filter which workers run during startup.
    /// The default implementation returns an empty vector (no tags).
    fn tags() -> Vec<String> {
        Vec::new()
    }

    /// Performs the actual work when a job is processed.
    ///
    /// This is the main function that contains the worker's logic.
    /// It gets executed when a job is dequeued from the job queue.
    ///
    /// # Returns
    /// * `Result<()>` - Ok if the job completed successfully, Err otherwise
    async fn perform(&self, _args: WorkerArgs) -> Result<()> {
        
        
        Ok(())
    }
}

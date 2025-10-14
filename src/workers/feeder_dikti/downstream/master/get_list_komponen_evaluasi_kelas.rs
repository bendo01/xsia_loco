use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

use chrono::Local;
use chrono::NaiveDate;
use colored::Colorize;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use uuid::Uuid; // Removed unused `uuid` macro import

use crate::models::feeder::master::komponen_evaluasi_kelas::_entities::komponen_evaluasi_kelas as FeederMasterKomponenEvaluasiKelas;
use crate::tasks::feeder_dikti::downstream::request_only_data::{InputRequestData, RequestData};

fn deserialize_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    NaiveDate::parse_from_str(&s, "%d-%m-%Y").map_err(serde::de::Error::custom)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelInput {
    pub id_komponen_evaluasi: Uuid,
    pub id_kelas_kuliah: Uuid,
    pub id_jenis_evaluasi: i32,
    pub nama: Option<String>,
    pub nama_inggris: Option<String>,
    pub nomor_urut: i32,
    pub bobot_evaluasi: String,
    #[serde(deserialize_with = "deserialize_date")]
    pub last_update: NaiveDate,
    #[serde(deserialize_with = "deserialize_date")]
    pub tgl_create: NaiveDate,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelData;

impl ModelData {
    pub async fn upsert(ctx: &AppContext, input: ModelInput) -> Result<(), Error> {
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
            let mut reference: FeederMasterKomponenEvaluasiKelas::ActiveModel =
                existing_reference.into();
            reference.id_komponen_evaluasi = Set(input.id_komponen_evaluasi);
            reference.id_kelas_kuliah = Set(input.id_kelas_kuliah);
            reference.id_jenis_evaluasi = Set(input.id_jenis_evaluasi);
            reference.nomor_urut = Set(input.nomor_urut);
            reference.bobot_evaluasi = Set(input.bobot_evaluasi);
            reference.nama = Set(input.nama);
            reference.nama_inggris = Set(input.nama_inggris);
            reference.last_update = Set(input.last_update); // <- Now NaiveDate
            reference.tgl_create = Set(input.tgl_create); // <- Now NaiveDate
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
                last_update: Set(input.last_update), // <- NaiveDate
                tgl_create: Set(input.tgl_create),   // <- NaiveDate
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
        println!("=================GetListKomponenEvaluasiKelas=======================");
        // TODO: Some actual work goes here...
        let req_option = RequestData::get::<ModelInput>(
            &self.ctx,
            InputRequestData {
                act: "GetListKomponenEvaluasiKelas".to_string(),
                filter: None,
                order: None,
                limit: None,
                offset: None,
            },
        )
        .await;

        if let Ok(response) = req_option {
            match response.data {
                Some(data_vec) if !data_vec.is_empty() => {
                    println!("Processing {} items", data_vec.len());
                    for item in data_vec {
                        if let Err(e) = ModelData::upsert(&self.ctx, item).await {
                            println!("Failed to upsert item: {}", e);
                        }
                    }
                }
                Some(_) => println!("Received empty data vector"),
                None => println!("No data in response"),
            }
        } else {
            println!("Failed to get data: {:#?}", req_option);
        }
        Ok(())
    }
}

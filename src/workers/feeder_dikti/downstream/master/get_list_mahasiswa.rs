use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
// use tokio::time::{sleep, Duration};
use chrono::Local;
use chrono::NaiveDate;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use uuid::Uuid; // Removed unused `uuid` macro import

use crate::models::feeder::master::mahasiswa::_entities::mahasiswa as FeederMasterMahasiswa;
use crate::tasks::feeder_dikti::downstream::request_data_pagination::{
    InputRequestData, RequestData,
};

use crate::library::deserialization::{
    deserialize_date_opt, de_opt_f32, de_opt_u32,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelInput {
    pub nama_mahasiswa: String,
    pub jenis_kelamin: Option<String>,

    #[serde(deserialize_with = "deserialize_date_opt")]
    pub tanggal_lahir: Option<NaiveDate>,

    pub id_perguruan_tinggi: Option<String>,
    pub nipd: Option<String>,

    #[serde(deserialize_with = "de_opt_f32")]
    pub ipk: Option<f32>,

    #[serde(deserialize_with = "de_opt_u32")]
    pub total_sks: Option<u32>,

    pub id_sms: Option<String>,
    pub id_mahasiswa: String,

    #[serde(deserialize_with = "de_opt_u32")]
    pub id_agama: Option<u32>,   // FIXED

    pub nama_agama: Option<String>,
    pub id_prodi: Option<String>,
    pub nama_program_studi: Option<String>,

    #[serde(deserialize_with = "de_opt_u32")]
    pub id_status_mahasiswa: Option<u32>,   // FIXED

    pub nama_status_mahasiswa: Option<String>,
    pub nim: Option<String>,
    pub id_periode: Option<String>,
    pub nama_periode_masuk: Option<String>,
    pub id_registrasi_mahasiswa: Option<String>,
    pub id_periode_keluar: Option<String>,

    #[serde(deserialize_with = "deserialize_date_opt")]
    pub tanggal_keluar: Option<NaiveDate>,

    #[serde(deserialize_with = "deserialize_date_opt")]
    pub last_update: Option<NaiveDate>,

    #[serde(deserialize_with = "deserialize_date_opt")]
    pub tgl_create: Option<NaiveDate>,

    pub status_sync: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelData;

impl ModelData {
    pub async fn upsert(ctx: &AppContext, input: ModelInput) -> Result<(), Error> {
        let data_result = FeederMasterMahasiswa::Entity::find()
            .filter(FeederMasterMahasiswa::Column::DeletedAt.is_null())
            .filter(
                FeederMasterMahasiswa::Column::IdRegistrasiMahasiswa
                    .eq(input.id_registrasi_mahasiswa.clone()),
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
            let mut reference: FeederMasterMahasiswa::ActiveModel =
                existing_reference.into();
            // reference.id_komponen_evaluasi = Set(input.id_komponen_evaluasi);
            reference.nama_mahasiswa = Set(input.nama_mahasiswa);
            reference.jenis_kelamin = Set(input.jenis_kelamin);
            reference.tanggal_lahir = Set(input.tanggal_lahir);
            reference.id_perguruan_tinggi = Set(input.id_perguruan_tinggi);
            reference.ipk = Set(input.ipk);
            reference.total_sks = Set(input.total_sks);
            reference.id_sms = Set(input.id_sms);
            reference.id_mahasiswa = Set(input.id_mahasiswa);
            reference.id_agama = Set(input.id_agama);
            reference.nama_agama = Set(input.nama_agama);
            reference.id_prodi = Set(input.id_prodi);
            reference.nama_program_studi = Set(input.nama_program_studi);
            reference.id_status_mahasiswa = Set(input.id_status_mahasiswa);
            reference.nama_status_mahasiswa = Set(input.nama_status_mahasiswa);
            reference.nim = Set(input.nim);
            reference.id_periode = Set(input.id_periode);
            reference.nama_periode_masuk = Set(input.nama_periode_masuk);
            reference.id_registrasi_mahasiswa = Set(input.id_registrasi_mahasiswa);
            reference.id_periode_keluar = Set(input.id_periode_keluar);
            reference.tanggal_keluar = Set(input.tanggal_keluar);
            reference.last_update = Set(input.last_update);
            reference.tgl_create = Set(input.tgl_create);
            reference.status_sync = Set(input.status_sync);


            match reference.update(&ctx.db).await {
                Ok(_updated_model) => {
                    println!("{}", "Data updated successfully");
                    Ok(())
                }
                Err(err) => {
                    return Err(Error::Message(format!("Failed to update reference: {err}")));
                }
            }
        } else {
            let uuidv7_string = uuid7::uuid7().to_string();
            let pk_id = Uuid::parse_str(&uuidv7_string).expect("Invalid UUID format");
            let new_reference = FeederMasterMahasiswa::ActiveModel {
                id: Set(pk_id),
                
                nama_mahasiswa: Set(input.nama_mahasiswa),
                jenis_kelamin: Set(input.jenis_kelamin),
                tanggal_lahir: Set(input.tanggal_lahir),
                id_perguruan_tinggi: Set(input.id_perguruan_tinggi),
                ipk: Set(input.ipk),
                total_sks: Set(input.total_sks),
                id_sms: Set(input.id_sms),
                id_mahasiswa: Set(input.id_mahasiswa),
                id_agama: Set(input.id_agama),
                nama_agama: Set(input.nama_agama),
                id_prodi: Set(input.id_prodi),
                nama_program_studi: Set(input.nama_program_studi),
                id_status_mahasiswa: Set(input.id_status_mahasiswa),
                nama_status_mahasiswa: Set(input.nama_status_mahasiswa),
                nim: Set(input.nim),
                id_periode: Set(input.id_periode),
                nama_periode_masuk: Set(input.nama_periode_masuk),
                id_registrasi_mahasiswa: Set(input.id_registrasi_mahasiswa),
                id_periode_keluar: Set(input.id_periode_keluar),
                tanggal_keluar: Set(input.tanggal_keluar),
                last_update: Set(input.last_update),
                tgl_create: Set(input.tgl_create),
                status_sync: Set(input.status_sync),
                created_at: Set(Some(Local::now().naive_local())),
                updated_at: Set(Some(Local::now().naive_local())),
                sync_at: Set(Some(Local::now().naive_local())),
                ..Default::default()
            };
            match FeederMasterMahasiswa::Entity::insert(new_reference)
                .exec(&ctx.db)
                .await
            {
                Ok(_insert_result) => {
                    println!("{}", "Data inserted successfully".to_string());
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
pub struct WorkerArgs {
    pub act: String,
    pub filter: Option<String>,
    pub order: Option<String>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

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
        "GetListMahasiswa".to_string()
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
    async fn perform(&self, args: WorkerArgs) -> Result<()> {
        println!("=================GetListMahasiswa=======================");
        // TODO: Some actual work goes here...
        println!("ARGS Data {:#?}", args);

        let req_result = RequestData::get::<ModelInput>(
            &self.ctx,
            InputRequestData {
                act: args.act,          // String
                filter: args.filter,    // Option<String>
                order: args.order,      // Option<String>
                limit: args.limit,      // Option<i32>
                offset: args.offset,    // Option<i32>
            },
        ).await;

        if let Ok(response) = req_result {
            match response.data {
                Some(data_vec) if !data_vec.is_empty() => {
                    println!("Processing {} items", data_vec.len());
                    // ...
                }
                Some(_) => println!("Received empty data vector"),
                None => println!("No data in response"),
            }
        } else {
            println!("Failed to get data: {:#?}", req_result);
        }

        Ok(())
    }
}

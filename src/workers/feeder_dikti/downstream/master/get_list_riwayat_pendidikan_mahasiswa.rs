use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
// use tokio::time::{sleep, Duration};
use chrono::Local;
use chrono::NaiveDate;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set};
use uuid::Uuid; // Removed unused `uuid` macro import

use crate::models::feeder::master::riwayat_pendidikan_mahasiswa::_entities::riwayat_pendidikan_mahasiswa as FeederMasterRiwayatPendidikanMahasiswa;
use crate::tasks::feeder_dikti::downstream::request_only_data::{InputRequestData, RequestData};

use crate::library::deserialization::{de_opt_date_dmy, de_opt_i32};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelInput {
    // UUIDs
    pub id_registrasi_mahasiswa: Uuid,
    pub id_mahasiswa: Uuid,
    pub id_perguruan_tinggi: Uuid,
    pub id_prodi: Uuid,

    // Identifiers / names
    pub nim: String,
    pub nama_mahasiswa: String,
    pub nama_perguruan_tinggi: String,
    pub nama_program_studi: String,
    pub nama_jenis_daftar: Option<String>,
    pub keterangan_keluar: Option<String>,
    pub nama_program_studi_asal: Option<String>,
    pub nama_perguruan_tinggi_asal: Option<String>,
    pub nama_periode_masuk: String,
    pub nm_bidang_minat: Option<String>,
    pub nama_pembiayaan_awal: Option<String>,
    pub nama_ibu_kandung: String,
    pub status_sync: String,

    // Numeric codes (accept number | numeric string | null)
    #[serde(deserialize_with = "de_opt_i32")]
    pub id_jenis_daftar: Option<i32>,

    #[serde(deserialize_with = "de_opt_i32")]
    pub id_jalur_daftar: Option<i32>,

    // Period codes often come as strings in source; treat as numeric code if you prefer
    #[serde(deserialize_with = "de_opt_i32")]
    pub id_periode_masuk: Option<i32>,

    #[serde(deserialize_with = "de_opt_i32")]
    pub id_jenis_keluar: Option<i32>,

    #[serde(deserialize_with = "de_opt_i32")]
    pub id_pembiayaan: Option<i32>,

    #[serde(deserialize_with = "de_opt_i32")]
    pub id_periode_keluar: Option<i32>,

    // Optional UUIDs (may be null)
    pub id_perguruan_tinggi_asal: Option<Uuid>,
    pub id_prodi_asal: Option<Uuid>,

    // Other enums/flags stored as short strings in source
    pub jenis_kelamin: String,

    // Credits recognized: string digits or null in source → numeric here
    #[serde(deserialize_with = "de_opt_i32")]
    pub sks_diakui: Option<i32>,

    // Money/amount (source may send "0" or numbers). If values can exceed i32, consider adding a de_opt_i64 helper and switching to i64.
    #[serde(deserialize_with = "de_opt_i32")]
    pub biaya_masuk: Option<i32>,

    // Optional fields that look like categorical codes
    pub id_bidang_minat: Option<String>, // keep as String if it’s alphanumeric

    // Dates (accept dd-MM-yyyy, then fallback yyyy-MM-dd), or null/empty
    #[serde(deserialize_with = "de_opt_date_dmy")]
    pub tanggal_daftar: Option<NaiveDate>,

    #[serde(deserialize_with = "de_opt_date_dmy")]
    pub tanggal_keluar: Option<NaiveDate>,

    #[serde(deserialize_with = "de_opt_date_dmy")]
    pub last_update: Option<NaiveDate>,

    #[serde(deserialize_with = "de_opt_date_dmy")]
    pub tgl_create: Option<NaiveDate>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelData;

impl ModelData {
    pub async fn upsert(ctx: &AppContext, input: ModelInput) -> Result<(), Error> {
        let find = FeederMasterRiwayatPendidikanMahasiswa::Entity::find()
            .filter(FeederMasterRiwayatPendidikanMahasiswa::Column::DeletedAt.is_null())
            .filter(
                FeederMasterRiwayatPendidikanMahasiswa::Column::IdRegistrasiMahasiswa
                    .eq(input.id_registrasi_mahasiswa),
            );

        // then .one(&ctx.db).await as before
        let data_result = find.one(&ctx.db).await;

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
            let mut reference: FeederMasterRiwayatPendidikanMahasiswa::ActiveModel =
                existing_reference.into();
            // Update fields
            reference.id_registrasi_mahasiswa = Set(input.id_registrasi_mahasiswa);
            reference.id_mahasiswa = Set(input.id_mahasiswa);
            reference.nim = Set(input.nim);
            reference.nama_mahasiswa = Set(input.nama_mahasiswa);
            reference.id_jenis_daftar = Set(input.id_jenis_daftar);
            reference.nama_jenis_daftar = Set(input.nama_jenis_daftar);
            reference.id_jalur_daftar = Set(input.id_jalur_daftar);
            reference.id_periode_masuk = Set(input.id_periode_masuk);
            reference.nama_periode_masuk = Set(input.nama_periode_masuk);
            reference.id_jenis_keluar = Set(input.id_jenis_keluar);
            reference.keterangan_keluar = Set(input.keterangan_keluar);
            reference.id_perguruan_tinggi = Set(input.id_perguruan_tinggi);
            reference.nama_perguruan_tinggi = Set(input.nama_perguruan_tinggi);
            reference.id_prodi = Set(input.id_prodi);
            reference.nama_program_studi = Set(input.nama_program_studi);
            reference.sks_diakui = Set(input.sks_diakui);
            reference.id_perguruan_tinggi_asal = Set(input.id_perguruan_tinggi_asal);
            reference.nama_perguruan_tinggi_asal = Set(input.nama_perguruan_tinggi_asal);
            reference.id_prodi_asal = Set(input.id_prodi_asal);
            reference.nama_program_studi_asal = Set(input.nama_program_studi_asal);
            reference.jenis_kelamin = Set(input.jenis_kelamin);
            reference.tanggal_daftar = Set(input.tanggal_daftar);
            reference.nama_ibu_kandung = Set(input.nama_ibu_kandung);
            reference.id_pembiayaan = Set(input.id_pembiayaan);
            reference.nama_pembiayaan_awal = Set(input.nama_pembiayaan_awal);
            reference.biaya_masuk = Set(input.biaya_masuk);
            reference.id_bidang_minat = Set(input.id_bidang_minat);
            reference.nm_bidang_minat = Set(input.nm_bidang_minat);
            reference.id_periode_keluar = Set(input.id_periode_keluar);
            reference.tanggal_keluar = Set(input.tanggal_keluar);
            reference.last_update = Set(input.last_update);
            reference.tgl_create = Set(input.tgl_create);
            reference.status_sync = Set(input.status_sync);
            reference.updated_at = Set(Some(Local::now().naive_local()));
            reference.sync_at = Set(Some(Local::now().naive_local()));

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
            let new_reference = FeederMasterRiwayatPendidikanMahasiswa::ActiveModel {
                id: Set(pk_id),
                id_registrasi_mahasiswa: Set(input.id_registrasi_mahasiswa),
                id_mahasiswa: Set(input.id_mahasiswa),
                nim: Set(input.nim),
                nama_mahasiswa: Set(input.nama_mahasiswa),
                id_jenis_daftar: Set(input.id_jenis_daftar),
                nama_jenis_daftar: Set(input.nama_jenis_daftar),
                id_jalur_daftar: Set(input.id_jalur_daftar),
                id_periode_masuk: Set(input.id_periode_masuk),
                nama_periode_masuk: Set(input.nama_periode_masuk),
                id_jenis_keluar: Set(input.id_jenis_keluar),
                keterangan_keluar: Set(input.keterangan_keluar),
                id_perguruan_tinggi: Set(input.id_perguruan_tinggi),
                nama_perguruan_tinggi: Set(input.nama_perguruan_tinggi),
                id_prodi: Set(input.id_prodi),
                nama_program_studi: Set(input.nama_program_studi),
                sks_diakui: Set(input.sks_diakui),
                id_perguruan_tinggi_asal: Set(input.id_perguruan_tinggi_asal),
                nama_perguruan_tinggi_asal: Set(input.nama_perguruan_tinggi_asal),
                id_prodi_asal: Set(input.id_prodi_asal),
                nama_program_studi_asal: Set(input.nama_program_studi_asal),
                jenis_kelamin: Set(input.jenis_kelamin),
                tanggal_daftar: Set(input.tanggal_daftar),
                nama_ibu_kandung: Set(input.nama_ibu_kandung),
                id_pembiayaan: Set(input.id_pembiayaan),
                nama_pembiayaan_awal: Set(input.nama_pembiayaan_awal),
                biaya_masuk: Set(input.biaya_masuk),
                id_bidang_minat: Set(input.id_bidang_minat),
                nm_bidang_minat: Set(input.nm_bidang_minat),
                id_periode_keluar: Set(input.id_periode_keluar),
                tanggal_keluar: Set(input.tanggal_keluar),
                last_update: Set(input.last_update),
                tgl_create: Set(input.tgl_create),
                status_sync: Set(input.status_sync),
                created_at: Set(Some(Local::now().naive_local())), // Option<DateTime>
                updated_at: Set(Some(Local::now().naive_local())), // Option<DateTime>
                sync_at: Set(Some(Local::now().naive_local())),    // Option<DateTime>
                ..Default::default()
            };
            match FeederMasterRiwayatPendidikanMahasiswa::Entity::insert(new_reference)
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
        "GetListRiwayatPendidikanMahasiswa".to_string()
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
        println!("=================GetListRiwayatPendidikanMahasiswa=======================");
        // TODO: Some actual work goes here...
        // println!("ARGS Data {:#?}", args);

        let req_result = RequestData::get::<ModelInput>(
            &self.ctx,
            InputRequestData {
                act: args.act,       // String
                filter: args.filter, // Option<String>
                order: args.order,   // Option<String>
                limit: args.limit,   // Option<i32>
                offset: args.offset, // Option<i32>
            },
        )
        .await;

        if let Ok(response) = req_result {
            match response.data {
                Some(data_vec) if !data_vec.is_empty() => {
                    println!("Processing {} items", data_vec.len());
                    // println!("{:#?}", data_vec);
                    // for item in data_vec {
                    //     if let Err(e) = ModelData::upsert(&self.ctx, item).await {
                    //         println!("Failed to upsert item: {}", e);
                    //     }
                    // }
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

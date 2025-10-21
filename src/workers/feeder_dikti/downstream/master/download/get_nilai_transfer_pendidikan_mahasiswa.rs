use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
// use tokio::time::{sleep, Duration};
use chrono::Local;
// use chrono::NaiveDate;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set};
use uuid::Uuid; // Removed unused `uuid` macro import

use crate::models::feeder::master::nilai_transfer_pendidikan_mahasiswa::_entities::nilai_transfer_pendidikan_mahasiswa as FeederMasterNilaiTransferPendidikanMahasiswa;
use crate::tasks::feeder_dikti::downstream::request_only_data::{InputRequestData, RequestData};

use crate::library::deserialization::de_opt_f32;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelInput {
    // UUIDs
    // Kunci & identitas
    pub id_transfer: Uuid,
    pub id_registrasi_mahasiswa: Uuid,
    pub id_matkul: Uuid,

    pub nim: String,
    pub nama_mahasiswa: String,

    pub id_prodi: Uuid,
    pub nama_program_studi: String,

    // Periode & semester
    #[serde(rename = "id_periode_masuk")]
    pub id_periode_masuk: String, // "20241" (string)
    pub id_semester: String,   // "20241" (string)
    pub nama_semester: String, // "2024/2025 Ganjil"

    // Mata kuliah asal
    pub kode_mata_kuliah_asal: String,
    pub nama_mata_kuliah_asal: String,
    #[serde(deserialize_with = "de_opt_f32")]
    pub sks_mata_kuliah_asal: Option<f32>,
    pub nilai_huruf_asal: Option<String>,

    // Matkul diakui (konversi)
    pub kode_matkul_diakui: String,
    pub nama_mata_kuliah_diakui: String,
    #[serde(deserialize_with = "de_opt_f32")]
    pub sks_mata_kuliah_diakui: Option<f32>,
    pub nilai_huruf_diakui: Option<String>,
    #[serde(deserialize_with = "de_opt_f32")]
    pub nilai_angka_diakui: Option<f32>, // "2.0000" → 2.0

    // Metadata/relasi opsional
    pub id_perguruan_tinggi: Option<Uuid>,
    pub id_aktivitas: Option<Uuid>,
    pub judul: Option<String>,
    pub id_jenis_aktivitas: Option<Uuid>,
    pub nama_jenis_aktivitas: Option<String>,

    // Status
    pub status_sync: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelData;

impl ModelData {
    pub async fn upsert(ctx: &AppContext, input: ModelInput) -> Result<(), Error> {
        let find = FeederMasterNilaiTransferPendidikanMahasiswa::Entity::find()
            .filter(FeederMasterNilaiTransferPendidikanMahasiswa::Column::DeletedAt.is_null())
            .filter(
                FeederMasterNilaiTransferPendidikanMahasiswa::Column::IdTransfer
                    .eq(input.id_transfer),
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
            let mut reference: FeederMasterNilaiTransferPendidikanMahasiswa::ActiveModel =
                existing_reference.into();
            // Update fields present in nilai_transfer_pendidikan_mahasiswa
            reference.id_transfer = Set(input.id_transfer);
            reference.id_registrasi_mahasiswa = Set(input.id_registrasi_mahasiswa);
            reference.id_matkul = Set(input.id_matkul);
            reference.nim = Set(input.nim);
            reference.nama_mahasiswa = Set(input.nama_mahasiswa);
            reference.id_prodi = Set(input.id_prodi);
            reference.nama_program_studi = Set(input.nama_program_studi);
            reference.id_periode_masuk = Set(input.id_periode_masuk);
            reference.id_semester = Set(input.id_semester);
            reference.nama_semester = Set(input.nama_semester);
            reference.kode_mata_kuliah_asal = Set(input.kode_mata_kuliah_asal);
            reference.nama_mata_kuliah_asal = Set(input.nama_mata_kuliah_asal);
            // convert f32 -> f64 where needed
            reference.sks_mata_kuliah_asal = Set(input.sks_mata_kuliah_asal.map(|v| v as f32));
            reference.nilai_huruf_asal = Set(input.nilai_huruf_asal);
            reference.kode_matkul_diakui = Set(input.kode_matkul_diakui);
            reference.nama_mata_kuliah_diakui = Set(input.nama_mata_kuliah_diakui);
            reference.sks_mata_kuliah_diakui = Set(input.sks_mata_kuliah_diakui.map(|v| v as f32));
            reference.nilai_huruf_diakui = Set(input.nilai_huruf_diakui);
            reference.nilai_angka_diakui = Set(input.nilai_angka_diakui.map(|v| v as f32));
            reference.id_perguruan_tinggi = Set(input.id_perguruan_tinggi);
            reference.id_aktivitas = Set(input.id_aktivitas);
            reference.judul = Set(input.judul);
            reference.id_jenis_aktivitas = Set(input.id_jenis_aktivitas);
            reference.nama_jenis_aktivitas = Set(input.nama_jenis_aktivitas);
            reference.status_sync = Set(input.status_sync);
            reference.updated_at = Set(Local::now().naive_local());
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
            let new_reference = FeederMasterNilaiTransferPendidikanMahasiswa::ActiveModel {
                id: Set(pk_id),
                id_transfer: Set(input.id_transfer),
                id_registrasi_mahasiswa: Set(input.id_registrasi_mahasiswa),
                id_matkul: Set(input.id_matkul),
                nim: Set(input.nim),
                nama_mahasiswa: Set(input.nama_mahasiswa),
                id_prodi: Set(input.id_prodi),
                nama_program_studi: Set(input.nama_program_studi),
                id_periode_masuk: Set(input.id_periode_masuk),
                id_semester: Set(input.id_semester),
                nama_semester: Set(input.nama_semester),
                kode_mata_kuliah_asal: Set(input.kode_mata_kuliah_asal),
                nama_mata_kuliah_asal: Set(input.nama_mata_kuliah_asal),
                sks_mata_kuliah_asal: Set(input.sks_mata_kuliah_asal.map(|v: f32| v as f32)),
                nilai_huruf_asal: Set(input.nilai_huruf_asal),
                kode_matkul_diakui: Set(input.kode_matkul_diakui),
                nama_mata_kuliah_diakui: Set(input.nama_mata_kuliah_diakui),
                sks_mata_kuliah_diakui: Set(input.sks_mata_kuliah_diakui.map(|v: f32| v as f32)),
                nilai_huruf_diakui: Set(input.nilai_huruf_diakui),
                nilai_angka_diakui: Set(input.nilai_angka_diakui.map(|v| v as f32)),
                id_perguruan_tinggi: Set(input.id_perguruan_tinggi),
                id_aktivitas: Set(input.id_aktivitas),
                judul: Set(input.judul),
                id_jenis_aktivitas: Set(input.id_jenis_aktivitas),
                nama_jenis_aktivitas: Set(input.nama_jenis_aktivitas),
                status_sync: Set(input.status_sync),
                created_at: Set(Local::now().naive_local()), // NaiveDateTime
                updated_at: Set(Local::now().naive_local()), // NaiveDateTime
                sync_at: Set(Some(Local::now().naive_local())), // NaiveDateTime
                ..Default::default()
            };
            match FeederMasterNilaiTransferPendidikanMahasiswa::Entity::insert(new_reference)
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
        "GetNilaiTransferPendidikanMahasiswa".to_string()
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
        println!("=================GetNilaiTransferPendidikanMahasiswa=======================");
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
        // println!("{:#?}", req_result);

        if let Ok(response) = req_result {
            match response.data {
                Some(data_vec) if !data_vec.is_empty() => {
                    // println!("Processing {} items", data_vec.len());
                    // println!("{:#?}", data_vec);
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
            println!("Failed to get data: {:#?}", req_result);
        }

        Ok(())
    }
}

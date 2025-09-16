use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
// use tokio::time::{sleep, Duration};
use crate::library::deserialization::{
    // de_opt_f32,
    de_opt_i32, // <-- use i32 version
    deserialize_date_opt,
    de_opt_boolish

};
use crate::tasks::feeder_dikti::downstream::request_data_pagination::{
    InputRequestData, RequestData,
};
use chrono::Local;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use uuid::Uuid; // Removed unused `uuid` macro import

use crate::models::feeder::master::biodata_mahasiswa::_entities::biodata_mahasiswa as FeederMasterBiodataMahasiswa;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelInput {
    pub nama_mahasiswa: Option<String>,
    pub jenis_kelamin: Option<String>,
    pub tempat_lahir: Option<String>,
    #[serde(deserialize_with = "deserialize_date_opt")]
    pub tanggal_lahir: Option<Date>,
    pub id_mahasiswa: Option<Uuid>,
    #[serde(deserialize_with = "de_opt_i32")]
    pub id_agama: Option<i32>,
    pub nama_agama: Option<String>,
    pub nik: Option<String>,
    pub nisn: Option<String>,
    pub npwp: Option<String>,
    pub id_negara: Option<String>,
    pub kewarganegaraan: Option<String>,
    pub jalan: Option<String>,
    pub dusun: Option<String>,
    #[serde(deserialize_with = "de_opt_i32")]
    pub rt: Option<i32>,
    #[serde(deserialize_with = "de_opt_i32")]
    pub rw: Option<i32>,
    pub kelurahan: Option<String>,
    pub kode_pos: Option<String>,
    pub id_wilayah: Option<String>,
    pub nama_wilayah: Option<String>,
    pub id_jenis_tinggal: Option<String>,
    pub nama_jenis_tinggal: Option<String>,
    pub id_alat_transportasi: Option<String>,
    pub nama_alat_transportasi: Option<String>,
    pub telepon: Option<String>,
    pub handphone: Option<String>,
    pub email: Option<String>,
    #[serde(deserialize_with = "de_opt_boolish")]
    pub penerima_kps: Option<bool>,
    pub nomor_kps: Option<String>,
    pub nik_ayah: Option<String>,
    pub nama_ayah: Option<String>,
    #[serde(deserialize_with = "deserialize_date_opt")]
    pub tanggal_lahir_ayah: Option<Date>,
    pub id_pendidikan_ayah: Option<String>,
    pub nama_pendidikan_ayah: Option<String>,
    #[serde(deserialize_with = "de_opt_i32")]
    pub id_pekerjaan_ayah: Option<i32>,
    pub nama_pekerjaan_ayah: Option<String>,
    #[serde(deserialize_with = "de_opt_i32")]
    pub id_penghasilan_ayah: Option<i32>,
    pub nama_penghasilan_ayah: Option<String>,
    pub nik_ibu: Option<String>,
    pub nama_ibu_kandung: Option<String>,
    #[serde(deserialize_with = "deserialize_date_opt")]
    pub tanggal_lahir_ibu: Option<Date>,
    pub id_pendidikan_ibu: Option<String>,
    pub nama_pendidikan_ibu: Option<String>,
    #[serde(deserialize_with = "de_opt_i32")]
    pub id_pekerjaan_ibu: Option<i32>,
    pub nama_pekerjaan_ibu: Option<String>,
    #[serde(deserialize_with = "de_opt_i32")]
    pub id_penghasilan_ibu: Option<i32>,
    pub nama_penghasilan_ibu: Option<String>,
    pub nama_wali: Option<String>,
    pub tanggal_lahir_wali: Option<String>,
    pub id_pendidikan_wali: Option<String>,
    pub nama_pendidikan_wali: Option<String>,
    pub id_pekerjaan_wali: Option<String>,
    pub nama_pekerjaan_wali: Option<String>,
    pub id_penghasilan_wali: Option<String>,
    pub nama_penghasilan_wali: Option<String>,
    pub id_kebutuhan_khusus_mahasiswa: Option<i32>,
    pub nama_kebutuhan_khusus_mahasiswa: Option<String>,
    pub id_kebutuhan_khusus_ayah: Option<i32>,
    pub nama_kebutuhan_khusus_ayah: Option<String>,
    pub id_kebutuhan_khusus_ibu: Option<i32>,
    pub nama_kebutuhan_khusus_ibu: Option<String>,
    pub status_sync: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelData;

impl ModelData {
    pub async fn upsert(ctx: &AppContext, input: ModelInput) -> Result<(), Error> {
        let mut find = FeederMasterBiodataMahasiswa::Entity::find()
            .filter(FeederMasterBiodataMahasiswa::Column::DeletedAt.is_null());

        if let Some(idreg) = input.id_mahasiswa {
            find = find.filter(FeederMasterBiodataMahasiswa::Column::IdMahasiswa.eq(idreg));
        }

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
            let mut reference: FeederMasterBiodataMahasiswa::ActiveModel =
                existing_reference.into();
            // reference.id_komponen_evaluasi = Set(input.id_komponen_evaluasi);
            reference.nama_mahasiswa = Set(input.nama_mahasiswa);
            reference.jenis_kelamin = Set(input.jenis_kelamin);
            reference.tempat_lahir = Set(input.tempat_lahir);
            reference.tanggal_lahir = Set(input.tanggal_lahir);
            reference.id_mahasiswa = Set(input.id_mahasiswa);

            reference.id_agama = Set(input.id_agama);
            reference.nama_agama = Set(input.nama_agama);
            reference.nik = Set(input.nik);
            reference.nisn = Set(input.nisn);
            reference.npwp = Set(input.npwp);
            reference.penerima_kps = Set(input.penerima_kps);
            reference.nomor_kps = Set(input.nomor_kps);

            reference.id_negara = Set(input.id_negara);
            reference.kewarganegaraan = Set(input.kewarganegaraan);
            reference.jalan = Set(input.jalan);
            reference.dusun = Set(input.dusun);
            reference.rt = Set(input.rt);
            reference.rw = Set(input.rw);
            reference.kelurahan = Set(input.kelurahan);
            reference.kode_pos = Set(input.kode_pos);
            reference.id_wilayah = Set(input.id_wilayah);
            reference.nama_wilayah = Set(input.nama_wilayah);
            reference.id_jenis_tinggal = Set(input.id_jenis_tinggal);
            reference.nama_jenis_tinggal = Set(input.nama_jenis_tinggal);
            reference.id_alat_transportasi = Set(input.id_alat_transportasi);
            reference.nama_alat_transportasi = Set(input.nama_alat_transportasi);
            reference.telepon = Set(input.telepon);
            reference.handphone = Set(input.handphone);
            reference.email = Set(input.email);

            reference.nik_ayah = Set(input.nik_ayah);
            reference.nama_ayah = Set(input.nama_ayah);
            reference.tanggal_lahir_ayah = Set(input.tanggal_lahir_ayah);
            reference.id_pendidikan_ayah = Set(input.id_pendidikan_ayah);
            reference.nama_pendidikan_ayah = Set(input.nama_pendidikan_ayah);
            reference.id_pekerjaan_ayah = Set(input.id_pekerjaan_ayah);
            reference.nama_pekerjaan_ayah = Set(input.nama_pekerjaan_ayah);
            reference.id_penghasilan_ayah = Set(input.id_penghasilan_ayah);
            reference.nama_penghasilan_ayah = Set(input.nama_penghasilan_ayah);

            reference.nik_ibu = Set(input.nik_ibu);
            reference.nama_ibu_kandung = Set(input.nama_ibu_kandung);
            reference.tanggal_lahir_ibu = Set(input.tanggal_lahir_ibu);
            reference.id_pendidikan_ibu = Set(input.id_pendidikan_ibu);
            reference.nama_pendidikan_ibu = Set(input.nama_pendidikan_ibu);
            reference.id_pekerjaan_ibu = Set(input.id_pekerjaan_ibu);
            reference.nama_pekerjaan_ibu = Set(input.nama_pekerjaan_ibu);
            reference.id_penghasilan_ibu = Set(input.id_penghasilan_ibu);
            reference.nama_penghasilan_ibu = Set(input.nama_penghasilan_ibu);

            reference.nama_wali = Set(input.nama_wali);
            reference.tanggal_lahir_wali = Set(input.tanggal_lahir_wali);
            reference.id_pendidikan_wali = Set(input.id_pendidikan_wali);
            reference.nama_pendidikan_wali = Set(input.nama_pendidikan_wali);
            reference.id_pekerjaan_wali = Set(input.id_pekerjaan_wali);
            reference.nama_pekerjaan_wali = Set(input.nama_pekerjaan_wali);
            reference.id_penghasilan_wali = Set(input.id_penghasilan_wali);
            reference.nama_penghasilan_wali = Set(input.nama_penghasilan_wali);

            reference.id_kebutuhan_khusus_mahasiswa = Set(input.id_kebutuhan_khusus_mahasiswa);
            reference.nama_kebutuhan_khusus_mahasiswa = Set(input.nama_kebutuhan_khusus_mahasiswa);
            reference.id_kebutuhan_khusus_ayah = Set(input.id_kebutuhan_khusus_ayah);
            reference.nama_kebutuhan_khusus_ayah = Set(input.nama_kebutuhan_khusus_ayah);
            reference.id_kebutuhan_khusus_ibu = Set(input.id_kebutuhan_khusus_ibu);
            reference.nama_kebutuhan_khusus_ibu = Set(input.nama_kebutuhan_khusus_ibu);

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
            let new_reference = FeederMasterBiodataMahasiswa::ActiveModel {
                id: Set(pk_id),

                nama_mahasiswa: Set(input.nama_mahasiswa),
                jenis_kelamin: Set(input.jenis_kelamin),
                tempat_lahir: Set(input.tempat_lahir),
                tanggal_lahir: Set(input.tanggal_lahir),
                id_mahasiswa: Set(input.id_mahasiswa),
                id_agama: Set(input.id_agama),
                nama_agama: Set(input.nama_agama),
                nik: Set(input.nik),
                nisn: Set(input.nisn),
                npwp: Set(input.npwp),
                id_negara: Set(input.id_negara),
                kewarganegaraan: Set(input.kewarganegaraan),
                jalan: Set(input.jalan),
                dusun: Set(input.dusun),
                rt: Set(input.rt),
                rw: Set(input.rw),
                kelurahan: Set(input.kelurahan),
                kode_pos: Set(input.kode_pos),
                id_wilayah: Set(input.id_wilayah),
                nama_wilayah: Set(input.nama_wilayah),
                id_jenis_tinggal: Set(input.id_jenis_tinggal),
                nama_jenis_tinggal: Set(input.nama_jenis_tinggal),
                id_alat_transportasi: Set(input.id_alat_transportasi),
                nama_alat_transportasi: Set(input.nama_alat_transportasi),
                telepon: Set(input.telepon),
                handphone: Set(input.handphone),
                email: Set(input.email),
                penerima_kps: Set(input.penerima_kps),
                nomor_kps: Set(input.nomor_kps),
                nik_ayah: Set(input.nik_ayah),
                nama_ayah: Set(input.nama_ayah),
                tanggal_lahir_ayah: Set(input.tanggal_lahir_ayah),
                id_pendidikan_ayah: Set(input.id_pendidikan_ayah),
                nama_pendidikan_ayah: Set(input.nama_pendidikan_ayah),
                id_pekerjaan_ayah: Set(input.id_pekerjaan_ayah),
                nama_pekerjaan_ayah: Set(input.nama_pekerjaan_ayah),
                id_penghasilan_ayah: Set(input.id_penghasilan_ayah),
                nama_penghasilan_ayah: Set(input.nama_penghasilan_ayah),
                nik_ibu: Set(input.nik_ibu),
                nama_ibu_kandung: Set(input.nama_ibu_kandung),
                tanggal_lahir_ibu: Set(input.tanggal_lahir_ibu),
                id_pendidikan_ibu: Set(input.id_pendidikan_ibu),
                nama_pendidikan_ibu: Set(input.nama_pendidikan_ibu),
                id_pekerjaan_ibu: Set(input.id_pekerjaan_ibu),
                nama_pekerjaan_ibu: Set(input.nama_pekerjaan_ibu),
                id_penghasilan_ibu: Set(input.id_penghasilan_ibu),
                nama_penghasilan_ibu: Set(input.nama_penghasilan_ibu),
                nama_wali: Set(input.nama_wali),
                tanggal_lahir_wali: Set(input.tanggal_lahir_wali),
                id_pendidikan_wali: Set(input.id_pendidikan_wali),
                nama_pendidikan_wali: Set(input.nama_pendidikan_wali),
                id_pekerjaan_wali: Set(input.id_pekerjaan_wali),
                nama_pekerjaan_wali: Set(input.nama_pekerjaan_wali),
                id_penghasilan_wali: Set(input.id_penghasilan_wali),
                nama_penghasilan_wali: Set(input.nama_penghasilan_wali),
                id_kebutuhan_khusus_mahasiswa: Set(input.id_kebutuhan_khusus_mahasiswa),
                nama_kebutuhan_khusus_mahasiswa: Set(input.nama_kebutuhan_khusus_mahasiswa),
                id_kebutuhan_khusus_ayah: Set(input.id_kebutuhan_khusus_ayah),
                nama_kebutuhan_khusus_ayah: Set(input.nama_kebutuhan_khusus_ayah),
                id_kebutuhan_khusus_ibu: Set(input.id_kebutuhan_khusus_ibu),
                nama_kebutuhan_khusus_ibu: Set(input.nama_kebutuhan_khusus_ibu),
                status_sync: Set(input.status_sync),
                created_at: Set(Some(Local::now().naive_local())),
                updated_at: Set(Some(Local::now().naive_local())),
                sync_at: Set(Some(Local::now().naive_local())),
                ..Default::default()
            };
            match FeederMasterBiodataMahasiswa::Entity::insert(new_reference)
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
        "GetBiodataMahasiswa".to_string()
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
        println!("=================GetBiodataMahasiswa=======================");
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

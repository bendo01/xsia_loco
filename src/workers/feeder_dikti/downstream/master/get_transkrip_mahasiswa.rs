use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::Local;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::models::feeder::master::transkrip_mahasiswa::_entities::transkrip_mahasiswa as FeederMasterTranskripMahasiswa;
use crate::tasks::feeder_dikti::downstream::request_only_data::{InputRequestData, RequestData};

use crate::library::deserialization::{de_opt_f32};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelInput {
    pub id: Option<Uuid>,
    pub id_registrasi_mahasiswa: Option<Uuid>,
    pub id_matkul: Option<Uuid>,
    pub id_kelas_kuliah: Option<Uuid>,
    pub id_nilai_transfer: Option<String>,
    pub id_konversi_aktivitas: Option<String>,
    pub smt_diambil: Option<String>,
    pub kode_mata_kuliah: Option<String>,
    pub nama_mata_kuliah: Option<String>,
    #[serde(deserialize_with = "de_opt_f32")]
    pub sks_mata_kuliah: Option<f32>,
    #[serde(deserialize_with = "de_opt_f32")]
    pub nilai_angka: Option<f32>,
    pub nilai_huruf: Option<String>,
    #[serde(deserialize_with = "de_opt_f32")]
    pub nilai_indeks: Option<f32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelData;

impl ModelData {
    pub async fn upsert(ctx: &AppContext, input: ModelInput) -> Result<(), Error> {
        let mut find = FeederMasterTranskripMahasiswa::Entity::find()
            .filter(FeederMasterTranskripMahasiswa::Column::DeletedAt.is_null());

        if let Some(id_reg) = input.id_registrasi_mahasiswa {
            find = find.filter(FeederMasterTranskripMahasiswa::Column::IdRegistrasiMahasiswa.eq(id_reg));
        }
        if let Some(id_matkul) = input.id_matkul {
            find = find.filter(FeederMasterTranskripMahasiswa::Column::IdMatkul.eq(id_matkul));
        }
        if let Some(id_kelas) = input.id_kelas_kuliah {
            find = find.filter(FeederMasterTranskripMahasiswa::Column::IdKelasKuliah.eq(id_kelas));
        }

        let data_result = find.one(&ctx.db).await;

        let data_opt = match data_result {
            Ok(opt) => opt,
            Err(db_err) => {
                return Err(Error::Message(format!(
                    "Database error while querying transkrip_mahasiswa: {db_err}"
                )));
            }
        };

        if let Some(existing) = data_opt {
            let mut model: FeederMasterTranskripMahasiswa::ActiveModel = existing.into();
            model.id_registrasi_mahasiswa = Set(input.id_registrasi_mahasiswa);
            model.id_matkul = Set(input.id_matkul);
            model.id_kelas_kuliah = Set(input.id_kelas_kuliah);
            model.id_nilai_transfer = Set(input.id_nilai_transfer);
            model.id_konversi_aktivitas = Set(input.id_konversi_aktivitas);
            model.smt_diambil = Set(input.smt_diambil);
            model.kode_mata_kuliah = Set(input.kode_mata_kuliah);
            model.nama_mata_kuliah = Set(input.nama_mata_kuliah);
            model.sks_mata_kuliah = Set(input.sks_mata_kuliah);
            model.nilai_angka = Set(input.nilai_angka);
            model.nilai_huruf = Set(input.nilai_huruf);
            model.nilai_indeks = Set(input.nilai_indeks);
            model.updated_at = Set(Some(Local::now().naive_local()));
            model.sync_at = Set(Some(Local::now().naive_local()));

            match model.update(&ctx.db).await {
                Ok(_) => Ok(()),
                Err(err) => Err(Error::Message(format!(
                    "Failed to update transkrip_mahasiswa: {err}"
                ))),
            }
        } else {
            let uuidv7_string = uuid7::uuid7().to_string();
            let pk_id = Uuid::parse_str(&uuidv7_string).expect("Invalid UUID format");
            let new_model = FeederMasterTranskripMahasiswa::ActiveModel {
                id: Set(pk_id),
                id_registrasi_mahasiswa: Set(input.id_registrasi_mahasiswa),
                id_matkul: Set(input.id_matkul),
                id_kelas_kuliah: Set(input.id_kelas_kuliah),
                id_nilai_transfer: Set(input.id_nilai_transfer),
                id_konversi_aktivitas: Set(input.id_konversi_aktivitas),
                smt_diambil: Set(input.smt_diambil),
                kode_mata_kuliah: Set(input.kode_mata_kuliah),
                nama_mata_kuliah: Set(input.nama_mata_kuliah),
                sks_mata_kuliah: Set(input.sks_mata_kuliah),
                nilai_angka: Set(input.nilai_angka),
                nilai_huruf: Set(input.nilai_huruf),
                nilai_indeks: Set(input.nilai_indeks),
                created_at: Set(Some(Local::now().naive_local())),
                updated_at: Set(Some(Local::now().naive_local())),
                sync_at: Set(Some(Local::now().naive_local())),
                ..Default::default()
            };

            match FeederMasterTranskripMahasiswa::Entity::insert(new_model).exec(&ctx.db).await {
                Ok(_) => Ok(()),
                Err(err) => Err(Error::Message(format!(
                    "Failed to insert transkrip_mahasiswa: {err}"
                ))),
            }
        }
    }
}

pub struct Worker { pub ctx: AppContext }

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
    fn build(ctx: &AppContext) -> Self { Self { ctx: ctx.clone() } }
    fn class_name() -> String { "GetTranskripMahasiswa".to_string() }
    fn tags() -> Vec<String> { Vec::new() }
    async fn perform(&self, args: WorkerArgs) -> Result<()> {
        println!("=================GetTranskripMahasiswa=======================");
        let req_result = RequestData::get::<ModelInput>(&self.ctx, InputRequestData {
            act: args.act, filter: args.filter, order: args.order, limit: args.limit, offset: args.offset
        }).await;

        if let Ok(response) = req_result {
            match response.data {
                Some(data_vec) if !data_vec.is_empty() => {
                    for item in data_vec {
                        if let Err(e) = ModelData::upsert(&self.ctx, item).await {
                            println!("Failed to upsert item: {}", e);
                        }
                    }
                }
                Some(_) => println!("Received empty data vector"),
                _ => println!("No data in response"),
            }
        } else {
            println!("Failed to get data: {:#?}", req_result);
        }

        Ok(())
    }
}

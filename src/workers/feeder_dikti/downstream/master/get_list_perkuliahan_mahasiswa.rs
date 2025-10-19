use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::Local;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::models::feeder::master::perkuliahan_mahasiswa::_entities::perkuliahan_mahasiswa as FeederMasterPerkuliahanMahasiswa;
use crate::tasks::feeder_dikti::downstream::request_only_data::{InputRequestData, RequestData};

use crate::library::deserialization::{de_opt_f32};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelInput {
    pub id: Option<Uuid>,
    pub id_registrasi_mahasiswa: Option<Uuid>,
    pub nim: Option<String>,
    pub nama_mahasiswa: Option<String>,
    pub id_prodi: Option<Uuid>,
    pub nama_program_studi: Option<String>,
    pub angkatan: Option<String>,
    pub id_periode_masuk: Option<String>,
    pub id_semester: Option<String>,
    pub nama_semester: Option<String>,
    pub id_status_mahasiswa: Option<String>,
    pub nama_status_mahasiswa: Option<String>,
    #[serde(deserialize_with = "de_opt_f32")]
    pub ips: Option<f32>,
    #[serde(deserialize_with = "de_opt_f32")]
    pub ipk: Option<f32>,
    #[serde(deserialize_with = "de_opt_f32")]
    pub sks_semester: Option<f32>,
    #[serde(deserialize_with = "de_opt_f32")]
    pub sks_total: Option<f32>,
    #[serde(deserialize_with = "de_opt_f32")]
    pub biaya_kuliah_smt: Option<f32>,
    pub id_pembiayaan: Option<String>,
    pub status_sync: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelData;

impl ModelData {
    pub async fn upsert(ctx: &AppContext, input: ModelInput) -> Result<(), Error> {
        let mut find = FeederMasterPerkuliahanMahasiswa::Entity::find()
            .filter(FeederMasterPerkuliahanMahasiswa::Column::DeletedAt.is_null());

        if let Some(id_reg) = input.id_registrasi_mahasiswa.clone() {
            find = find.filter(FeederMasterPerkuliahanMahasiswa::Column::IdRegistrasiMahasiswa.eq(id_reg));
        }

        if let Some(id_cs) = input.id_prodi.clone() {
            find = find.filter(FeederMasterPerkuliahanMahasiswa::Column::IdProdi.eq(id_cs));
        }

        if let Some(id_academic_year_register) = input.id_periode_masuk.clone() {
            find = find.filter(FeederMasterPerkuliahanMahasiswa::Column::IdPeriodeMasuk.eq(id_academic_year_register));
        }

        if let Some(id_sem) = input.id_semester.clone() {
            find = find.filter(FeederMasterPerkuliahanMahasiswa::Column::IdSemester.eq(id_sem));
        }


        let data_result = find.one(&ctx.db).await;

        let data_opt = match data_result {
            Ok(opt) => opt,
            Err(db_err) => {
                return Err(Error::Message(format!(
                    "Database error while querying perkuliahan_mahasiswa: {db_err}"
                )));
            }
        };

        if let Some(existing) = data_opt {
            let mut model: FeederMasterPerkuliahanMahasiswa::ActiveModel = existing.into();
            model.id_registrasi_mahasiswa = Set(input.id_registrasi_mahasiswa);
            model.nim = Set(input.nim);
            model.nama_mahasiswa = Set(input.nama_mahasiswa);
            model.id_prodi = Set(input.id_prodi);
            model.nama_program_studi = Set(input.nama_program_studi);
            model.angkatan = Set(input.angkatan);
            model.id_periode_masuk = Set(input.id_periode_masuk);
            model.id_semester = Set(input.id_semester);
            model.nama_semester = Set(input.nama_semester);
            model.id_status_mahasiswa = Set(input.id_status_mahasiswa);
            model.nama_status_mahasiswa = Set(input.nama_status_mahasiswa);
            model.ips = Set(input.ips);
            model.ipk = Set(input.ipk);
            model.sks_semester = Set(input.sks_semester);
            model.sks_total = Set(input.sks_total);
            model.biaya_kuliah_smt = Set(input.biaya_kuliah_smt);
            model.id_pembiayaan = Set(input.id_pembiayaan);
            model.status_sync = Set(input.status_sync);
            model.updated_at = Set(Some(Local::now().naive_local()));
            model.sync_at = Set(Some(Local::now().naive_local()));

            match model.update(&ctx.db).await {
                Ok(_) => Ok(()),
                Err(err) => Err(Error::Message(format!(
                    "Failed to update perkuliahan_mahasiswa: {err}"
                ))),
            }
        } else {
            let uuidv7_string = uuid7::uuid7().to_string();
            let pk_id = Uuid::parse_str(&uuidv7_string).expect("Invalid UUID format");
            let new_model = FeederMasterPerkuliahanMahasiswa::ActiveModel {
                id: Set(pk_id),
                id_registrasi_mahasiswa: Set(input.id_registrasi_mahasiswa),
                nim: Set(input.nim),
                nama_mahasiswa: Set(input.nama_mahasiswa),
                id_prodi: Set(input.id_prodi),
                nama_program_studi: Set(input.nama_program_studi),
                angkatan: Set(input.angkatan),
                id_periode_masuk: Set(input.id_periode_masuk),
                id_semester: Set(input.id_semester),
                nama_semester: Set(input.nama_semester),
                id_status_mahasiswa: Set(input.id_status_mahasiswa),
                nama_status_mahasiswa: Set(input.nama_status_mahasiswa),
                ips: Set(input.ips),
                ipk: Set(input.ipk),
                sks_semester: Set(input.sks_semester),
                sks_total: Set(input.sks_total),
                biaya_kuliah_smt: Set(input.biaya_kuliah_smt),
                id_pembiayaan: Set(input.id_pembiayaan),
                status_sync: Set(input.status_sync),
                created_at: Set(Some(Local::now().naive_local())),
                updated_at: Set(Some(Local::now().naive_local())),
                sync_at: Set(Some(Local::now().naive_local())),
                ..Default::default()
            };

            match FeederMasterPerkuliahanMahasiswa::Entity::insert(new_model).exec(&ctx.db).await {
                Ok(_) => Ok(()),
                Err(err) => Err(Error::Message(format!(
                    "Failed to insert perkuliahan_mahasiswa: {err}"
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
    fn class_name() -> String { "GetListPerkuliahanMahasiswa".to_string() }
    fn tags() -> Vec<String> { Vec::new() }
    async fn perform(&self, args: WorkerArgs) -> Result<()> {
        println!("=================GetListPerkuliahanMahasiswa=======================");
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


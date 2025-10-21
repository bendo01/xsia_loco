use chrono::Local;
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::feeder::master::kartu_rencana_studi_mahasiswa::_entities::kartu_rencana_studi_mahasiswa as FeederMasterKrsMahasiswa;
use crate::tasks::feeder_dikti::downstream::request_only_data::{InputRequestData, RequestData};

use crate::library::deserialization::de_opt_f32;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelInput {
    pub id: Option<Uuid>,
    pub id_registrasi_mahasiswa: Option<Uuid>,
    pub id_periode: Option<String>,
    pub id_prodi: Option<Uuid>,
    pub nama_program_studi: Option<String>,
    pub id_matkul: Option<Uuid>,
    pub kode_mata_kuliah: Option<String>,
    pub nama_mata_kuliah: Option<String>,
    pub id_kelas: Option<Uuid>,
    pub nama_kelas_kuliah: Option<String>,
    #[serde(deserialize_with = "de_opt_f32")]
    pub sks_mata_kuliah: Option<f32>,
    pub nim: Option<String>,
    pub nama_mahasiswa: Option<String>,
    pub angkatan: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelData;

impl ModelData {
    pub async fn upsert(ctx: &AppContext, input: ModelInput) -> Result<(), Error> {
        let mut find = FeederMasterKrsMahasiswa::Entity::find()
            .filter(FeederMasterKrsMahasiswa::Column::DeletedAt.is_null());

        if let Some(id_reg) = input.id_registrasi_mahasiswa {
            find = find.filter(FeederMasterKrsMahasiswa::Column::IdRegistrasiMahasiswa.eq(id_reg));
        }
        if let Some(id_matkul) = input.id_matkul {
            find = find.filter(FeederMasterKrsMahasiswa::Column::IdMatkul.eq(id_matkul));
        }
        if let Some(id_kelas) = input.id_kelas {
            find = find.filter(FeederMasterKrsMahasiswa::Column::IdKelas.eq(id_kelas));
        }

        let data_result = find.one(&ctx.db).await;

        let data_opt = match data_result {
            Ok(opt) => opt,
            Err(db_err) => {
                return Err(Error::Message(format!(
                    "Database error while querying kartu_rencana_studi_mahasiswa: {db_err}"
                )));
            }
        };

        if let Some(existing) = data_opt {
            let mut model: FeederMasterKrsMahasiswa::ActiveModel = existing.into();
            model.id_registrasi_mahasiswa = Set(input.id_registrasi_mahasiswa);
            model.id_periode = Set(input.id_periode);
            model.id_prodi = Set(input.id_prodi);
            model.nama_program_studi = Set(input.nama_program_studi);
            model.id_matkul = Set(input.id_matkul);
            model.kode_mata_kuliah = Set(input.kode_mata_kuliah);
            model.nama_mata_kuliah = Set(input.nama_mata_kuliah);
            model.id_kelas = Set(input.id_kelas);
            model.nama_kelas_kuliah = Set(input.nama_kelas_kuliah);
            model.sks_mata_kuliah = Set(input.sks_mata_kuliah);
            model.nim = Set(input.nim);
            model.nama_mahasiswa = Set(input.nama_mahasiswa);
            model.angkatan = Set(input.angkatan);
            model.updated_at = Set(Some(Local::now().naive_local()));
            model.sync_at = Set(Some(Local::now().naive_local()));

            match model.update(&ctx.db).await {
                Ok(_) => Ok(()),
                Err(err) => Err(Error::Message(format!(
                    "Failed to update kartu_rencana_studi_mahasiswa: {err}"
                ))),
            }
        } else {
            let uuidv7_string = uuid7::uuid7().to_string();
            let pk_id = Uuid::parse_str(&uuidv7_string).expect("Invalid UUID format");
            let new_model = FeederMasterKrsMahasiswa::ActiveModel {
                id: Set(pk_id),
                id_registrasi_mahasiswa: Set(input.id_registrasi_mahasiswa),
                id_periode: Set(input.id_periode),
                id_prodi: Set(input.id_prodi),
                nama_program_studi: Set(input.nama_program_studi),
                id_matkul: Set(input.id_matkul),
                kode_mata_kuliah: Set(input.kode_mata_kuliah),
                nama_mata_kuliah: Set(input.nama_mata_kuliah),
                id_kelas: Set(input.id_kelas),
                nama_kelas_kuliah: Set(input.nama_kelas_kuliah),
                sks_mata_kuliah: Set(input.sks_mata_kuliah),
                nim: Set(input.nim),
                nama_mahasiswa: Set(input.nama_mahasiswa),
                angkatan: Set(input.angkatan),
                created_at: Set(Some(Local::now().naive_local())),
                updated_at: Set(Some(Local::now().naive_local())),
                sync_at: Set(Some(Local::now().naive_local())),
                ..Default::default()
            };

            match FeederMasterKrsMahasiswa::Entity::insert(new_model)
                .exec(&ctx.db)
                .await
            {
                Ok(_) => Ok(()),
                Err(err) => Err(Error::Message(format!(
                    "Failed to insert kartu_rencana_studi_mahasiswa: {err}"
                ))),
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
    fn build(ctx: &AppContext) -> Self {
        Self { ctx: ctx.clone() }
    }
    fn class_name() -> String {
        "GetKrsMahasiswa".to_string()
    }
    fn tags() -> Vec<String> {
        Vec::new()
    }
    async fn perform(&self, args: WorkerArgs) -> Result<()> {
        println!("=================GetKrsMahasiswa=======================");
        let req_result = RequestData::get::<ModelInput>(
            &self.ctx,
            InputRequestData {
                act: args.act,
                filter: args.filter,
                order: args.order,
                limit: args.limit,
                offset: args.offset,
            },
        )
        .await;

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

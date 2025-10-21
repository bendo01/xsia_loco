use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
// use tokio::time::{sleep, Duration};
use chrono::Local;
// use chrono::NaiveDate;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::models::feeder::master::riwayat_nilai_mahasiswa::_entities::riwayat_nilai_mahasiswa as FeederMasterRiwayatNilaiMahasiswa;
use crate::tasks::feeder_dikti::downstream::request_only_data::{InputRequestData, RequestData};

use crate::library::deserialization::de_opt_f32;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelInput {
    // UUIDs
    pub id: Option<Uuid>,
    pub id_registrasi_mahasiswa: Option<Uuid>,
    pub id_prodi: Option<Uuid>,
    pub nama_program_studi: Option<String>,

    // period
    pub id_periode: Option<String>,

    // mata kuliah / kelas
    pub id_matkul: Option<Uuid>,
    pub nama_mata_kuliah: Option<String>,
    pub id_kelas: Option<Uuid>,
    pub nama_kelas_kuliah: Option<String>,

    // Numeric fields (may come as strings) — deserialize helpers convert to Option<f32>
    #[serde(deserialize_with = "de_opt_f32")]
    pub sks_mata_kuliah: Option<f32>,

    #[serde(deserialize_with = "de_opt_f32")]
    pub nilai_angka: Option<f32>,

    pub nilai_huruf: Option<String>,

    #[serde(deserialize_with = "de_opt_f32")]
    pub nilai_indeks: Option<f32>,

    pub nim: Option<String>,
    pub nama_mahasiswa: Option<String>,
    pub angkatan: Option<String>,
    pub status_sync: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelData;

impl ModelData {
    pub async fn upsert(ctx: &AppContext, input: ModelInput) -> Result<(), Error> {
        // Build find query conditionally depending on which identifying fields we have
        let mut find = FeederMasterRiwayatNilaiMahasiswa::Entity::find()
            .filter(FeederMasterRiwayatNilaiMahasiswa::Column::DeletedAt.is_null());

        if let Some(id_reg) = input.id_registrasi_mahasiswa {
            find = find.filter(
                FeederMasterRiwayatNilaiMahasiswa::Column::IdRegistrasiMahasiswa.eq(id_reg),
            );
        }

        if let Some(id_cp) = input.id_prodi {
            find = find.filter(FeederMasterRiwayatNilaiMahasiswa::Column::IdProdi.eq(id_cp));
        }

        if let Some(id_matkul) = input.id_matkul {
            find = find.filter(FeederMasterRiwayatNilaiMahasiswa::Column::IdMatkul.eq(id_matkul));
        }

        if let Some(id_class) = input.id_kelas {
            find = find.filter(FeederMasterRiwayatNilaiMahasiswa::Column::IdKelas.eq(id_class));
        }

        if let Some(id_academic_year) = input.id_periode.clone() {
            find = find
                .filter(FeederMasterRiwayatNilaiMahasiswa::Column::IdPeriode.eq(id_academic_year));
        }

        let data_result = find.one(&ctx.db).await;

        let data_opt = match data_result {
            Ok(opt) => opt,
            Err(db_err) => {
                return Err(Error::Message(format!(
                    "Database error while querying riwayat_nilai_mahasiswa: {db_err}"
                )));
            }
        };

        if let Some(existing) = data_opt {
            let mut model: FeederMasterRiwayatNilaiMahasiswa::ActiveModel = existing.into();
            model.id_registrasi_mahasiswa = Set(input.id_registrasi_mahasiswa);
            model.id_prodi = Set(input.id_prodi);
            model.nama_program_studi = Set(input.nama_program_studi);
            model.id_periode = Set(input.id_periode);
            model.id_matkul = Set(input.id_matkul);
            model.nama_mata_kuliah = Set(input.nama_mata_kuliah);
            model.id_kelas = Set(input.id_kelas);
            model.nama_kelas_kuliah = Set(input.nama_kelas_kuliah);
            model.sks_mata_kuliah = Set(input.sks_mata_kuliah);
            model.nilai_angka = Set(input.nilai_angka);
            model.nilai_huruf = Set(input.nilai_huruf);
            model.nilai_indeks = Set(input.nilai_indeks);
            model.nim = Set(input.nim);
            model.nama_mahasiswa = Set(input.nama_mahasiswa);
            model.angkatan = Set(input.angkatan);
            model.status_sync = Set(input.status_sync);
            model.updated_at = Set(Local::now().naive_local());
            model.sync_at = Set(Some(Local::now().naive_local()));

            match model.update(&ctx.db).await {
                Ok(_) => {
                    // println!("{}", "Riwayat nilai updated successfully");
                    Ok(())
                }
                Err(err) => Err(Error::Message(format!(
                    "Failed to update riwayat_nilai_mahasiswa: {err}"
                ))),
            }
        } else {
            let uuidv7_string = uuid7::uuid7().to_string();
            let pk_id = Uuid::parse_str(&uuidv7_string).expect("Invalid UUID format");
            let new_model = FeederMasterRiwayatNilaiMahasiswa::ActiveModel {
                id: Set(pk_id),
                id_registrasi_mahasiswa: Set(input.id_registrasi_mahasiswa),
                id_prodi: Set(input.id_prodi),
                nama_program_studi: Set(input.nama_program_studi),
                id_periode: Set(input.id_periode),
                id_matkul: Set(input.id_matkul),
                nama_mata_kuliah: Set(input.nama_mata_kuliah),
                id_kelas: Set(input.id_kelas),
                nama_kelas_kuliah: Set(input.nama_kelas_kuliah),
                sks_mata_kuliah: Set(input.sks_mata_kuliah),
                nilai_angka: Set(input.nilai_angka),
                nilai_huruf: Set(input.nilai_huruf),
                nilai_indeks: Set(input.nilai_indeks),
                nim: Set(input.nim),
                nama_mahasiswa: Set(input.nama_mahasiswa),
                angkatan: Set(input.angkatan),
                status_sync: Set(input.status_sync),
                created_at: Set(Local::now().naive_local()),
                updated_at: Set(Local::now().naive_local()),
                sync_at: Set(Some(Local::now().naive_local())),
                ..Default::default()
            };

            match FeederMasterRiwayatNilaiMahasiswa::Entity::insert(new_model)
                .exec(&ctx.db)
                .await
            {
                Ok(_) => {
                    // println!("{}", "Riwayat nilai inserted successfully");
                    Ok(())
                }
                Err(err) => Err(Error::Message(format!(
                    "Failed to insert riwayat_nilai_mahasiswa: {err}"
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
        "GetRiwayatNilaiMahasiswa".to_string()
    }

    fn tags() -> Vec<String> {
        Vec::new()
    }

    async fn perform(&self, args: WorkerArgs) -> Result<()> {
        println!("=================GetRiwayatNilaiMahasiswa=======================");
        // println!("ARGS Data {:#?}", args);
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
        // println!("{:#?}", req_result);
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

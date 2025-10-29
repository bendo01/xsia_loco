use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "feeder_master", table_name = "nilai_perkuliahan_kelas")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub sync_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub id_matkul: Uuid,
    pub kode_mata_kuliah: String,
    pub nama_mata_kuliah: String,
    pub id_kelas_kuliah: Uuid,
    pub nama_kelas_kuliah: String,
    pub sks_mata_kuliah: Option<f32>,
    pub jumlah_mahasiswa_krs: Option<i32>,
    pub jumlah_mahasiswa_dapat_nilai: Option<i32>,
    pub sks_tm: Option<f32>,
    pub sks_prak: Option<f32>,
    pub sks_prak_lap: Option<f32>,
    pub sks_sim: Option<f32>,
    pub bahasan_case: Option<String>,
    pub a_selenggara_pditt: Option<i32>,
    pub a_pengguna_pditt: Option<i32>,
    pub kuota_pditt: Option<i32>,
    pub tgl_mulai_koas: Option<Date>,
    pub tgl_selesai_koas: Option<Date>,
    pub id_mou: Option<Uuid>,
    pub id_kls_pditt: Option<Uuid>,
    pub id_sms: Uuid,
    pub id_smt: String,
    pub tgl_create: Date,
    pub lingkup_kelas: Option<i32>,
    pub mode_kuliah: Option<String>,
    pub nm_smt: String,
    pub nama_prodi: String,
    pub status_sync: String,

}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

use chrono::NaiveDate;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "feeder_master", table_name = "kelas_kuliah")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub id_kelas_kuliah: Uuid,
    pub id_prodi: Uuid,
    pub nama_program_studi: String,
    pub id_semester: String,
    pub nama_semester: String,
    pub id_matkul: Uuid,
    pub kode_mata_kuliah: String,
    pub nama_mata_kuliah: String,
    pub sks_mk: f32,
    pub sks_tm: f32,
    pub sks_prak: f32,
    pub sks_prak_lap: f32,
    pub sks_sim: f32,
    pub nama_kelas_kuliah: String,
    pub bahasan: Option<String>,
    pub tanggal_mulai_efektif: Option<NaiveDate>,
    pub tanggal_akhir_efektif: Option<NaiveDate>,
    pub kapasitas: Option<i32>,
    pub tanggal_tutup_daftar: Option<NaiveDate>,
    pub prodi_penyelenggara: Option<String>,
    pub perguruan_tinggi_penyelenggara: Option<String>,
    pub sync_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

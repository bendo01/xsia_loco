use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "feeder_master", table_name = "kelas_kuliah")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub sync_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    
    #[sea_orm(unique)]
    pub id_kelas_kuliah: Uuid,
    pub id_prodi: Option<Uuid>,
    pub nama_program_studi: Option<String>,
    pub id_semester: Option<String>,
    pub nama_semester: Option<String>,
    pub id_matkul: Option<Uuid>,
    pub kode_mata_kuliah: Option<String>,
    pub nama_mata_kuliah: Option<String>,
    pub nama_kelas_kuliah: Option<String>,
    pub sks_mk: Option<f32>,
    pub sks_tm: Option<f32>,
    pub sks_prak: Option<f32>,
    pub sks_prak_lap: Option<f32>,
    pub sks_sim: Option<f32>,
    pub bahasan: Option<String>,
    pub tanggal_mulai_efektif: Option<NaiveDate>,
    pub tanggal_akhir_efektif: Option<NaiveDate>,
    pub kapasitas: Option<i32>,
    pub tanggal_tutup_daftar: Option<NaiveDate>,
    pub prodi_penyelenggara: Option<String>,
    pub perguruan_tinggi_penyelenggara: Option<String>,
    pub sks: Option<f32>,
    pub id_dosen: Option<String>,
    pub nama_dosen: Option<String>,
    pub jumlah_mahasiswa: Option<i32>,
    pub apa_untuk_pditt: Option<bool>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

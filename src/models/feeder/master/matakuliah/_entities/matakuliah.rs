use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "feeder_master", table_name = "matakuliah")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    
    // Feeder DIKTI fields
    pub id_matkul: Option<Uuid>,
    pub kode_mata_kuliah: Option<String>,
    pub nama_mata_kuliah: Option<String>,
    pub id_prodi: Option<Uuid>,
    pub nama_program_studi: Option<String>,
    pub id_jenis_mata_kuliah: Option<String>,
    pub nama_jenis_mata_kuliah: Option<String>,
    pub id_kelompok_mata_kuliah: Option<String>,
    pub nama_kelompok_mata_kuliah: Option<String>,
    pub sks_mata_kuliah: Option<f32>,
    pub sks_tatap_muka: Option<f32>,
    pub sks_praktek: Option<f32>,
    pub sks_praktek_lapangan: Option<f32>,
    pub sks_simulasi: Option<f32>,
    pub metode_kuliah: Option<String>,
    pub ada_sap: Option<bool>,
    pub ada_silabus: Option<bool>,
    pub ada_bahan_ajar: Option<bool>,
    pub ada_acara_praktek: Option<bool>,
    pub ada_diktat: Option<bool>,
    pub tanggal_mulai_efektif: Option<DateTime>,
    pub tanggal_selesai_efektif: Option<DateTime>,
    pub id_jenj_didik: Option<String>,
    pub tgl_create: Option<DateTime>,
    pub status_sync: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

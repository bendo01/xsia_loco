use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "feeder_master", table_name = "matakuliah")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub id_matkul: Option<Uuid>,
    pub kode_mata_kuliah: Option<String>,
    pub nama_mata_kuliah: Option<String>,
    pub id_prodi: Option<Uuid>,
    pub nama_program_studi: Option<String>,
    pub id_jenis_mata_kuliah: Option<String>,
    pub id_kelompok_mata_kuliah: Option<String>,
    pub sks_mata_kuliah: Option<String>,
    pub sks_tatap_muka: Option<String>,
    pub sks_praktek: Option<String>,
    pub sks_praktek_lapangan: Option<String>,
    pub sks_simulasi: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub metode_kuliah: Option<String>,
    pub ada_sap: Option<String>,
    pub ada_silabus: Option<String>,
    pub ada_bahan_ajar: Option<String>,
    pub ada_acara_praktek: Option<String>,
    pub ada_diktat: Option<String>,
    pub tanggal_mulai_efektif: Option<DateTimeWithTimeZone>,
    pub tanggal_selesai_efektif: Option<DateTimeWithTimeZone>,
    pub sync_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

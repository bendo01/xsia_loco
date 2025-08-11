use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "feeder_master", table_name = "matakuliah_kurikulum")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub tgl_create: Option<Date>,
    pub id_kurikulum: Option<Uuid>,
    pub nama_kurikulum: Option<String>,
    pub id_matkul: Option<Uuid>,
    pub kode_mata_kuliah: Option<String>,
    pub nama_mata_kuliah: Option<String>,
    pub id_prodi: Option<Uuid>,
    pub nama_program_studi: Option<String>,
    pub semester: Option<String>,
    pub id_semester: Option<String>,
    pub semester_mulai_berlaku: Option<String>,
    pub sks_mata_kuliah: Option<String>,
    pub sks_tatap_muka: Option<String>,
    pub sks_praktek: Option<String>,
    pub sks_praktek_lapangan: Option<String>,
    pub sks_simulasi: Option<String>,
    pub apakah_wajib: Option<String>,
    pub status_sync: Option<String>,
    pub sync_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

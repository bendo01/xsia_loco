use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "feeder_master", table_name = "kurikulum")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    
    // Fields from feeder API
    pub id_kurikulum: Option<Uuid>,
    pub nama_kurikulum: Option<String>,
    pub id_prodi: Option<Uuid>,
    pub nama_program_studi: Option<String>,
    pub id_jenj_didik: Option<i32>,
    pub jml_sem_normal: Option<i32>,
    pub id_semester: Option<String>,
    pub semester_mulai_berlaku: Option<String>,
    pub jumlah_sks_lulus: Option<f32>,
    pub jumlah_sks_wajib: Option<f32>,
    pub jumlah_sks_pilihan: Option<f32>,
    pub jumlah_sks_mata_kuliah_wajib: Option<f32>,
    pub jumlah_sks_mata_kuliah_pilihan: Option<f32>,
    pub status_sync: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

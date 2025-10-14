use chrono::NaiveDate;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "feeder_master", table_name = "komponen_evaluasi_kelas")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub id_komponen_evaluasi: Uuid,
    pub id_kelas_kuliah: Uuid,
    pub id_jenis_evaluasi: i32,
    pub nama: Option<String>,
    pub nama_inggris: Option<String>,
    pub nomor_urut: i32,
    pub bobot_evaluasi: String,
    pub last_update: NaiveDate,
    pub tgl_create: NaiveDate,
    pub sync_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

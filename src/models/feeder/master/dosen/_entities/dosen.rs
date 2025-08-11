use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "feeder_master", table_name = "dosen")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub id_dosen: Option<Uuid>,
    pub nama_dosen: Option<String>,
    pub nidn: Option<String>,
    pub nip: Option<String>,
    pub jenis_kelamin: Option<String>,
    pub id_agama: Option<String>,
    pub nama_agama: Option<String>,
    pub tanggal_lahir: Option<Date>,
    pub id_status_aktif: Option<String>,
    pub nama_status_aktif: Option<String>,
    pub sync_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "feeder_master", table_name = "perkuliahan_mahasiswa")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub id_registrasi_mahasiswa: Option<Uuid>,
    pub nim: Option<String>,
    pub nama_mahasiswa: Option<String>,
    pub id_prodi: Option<Uuid>,
    pub nama_program_studi: Option<String>,
    pub angkatan: Option<String>,
    pub id_periode_masuk: Option<String>,
    pub id_semester: Option<String>,
    pub nama_semester: Option<String>,
    pub id_status_mahasiswa: Option<String>,
    pub nama_status_mahasiswa: Option<String>,
    pub ips: Option<f32>,
    pub ipk: Option<f32>,
    pub sks_semester: Option<f32>,
    pub sks_total: Option<f32>,
    pub biaya_kuliah_smt: Option<f32>,
    pub id_pembiayaan: Option<String>,
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

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "feeder_master", table_name = "mahasiswa")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub nama_mahasiswa: Option<String>,
    pub jenis_kelamin: Option<String>,
    pub tanggal_lahir: Option<Date>,
    pub id_perguruan_tinggi: Option<Uuid>,
    pub nipd: Option<String>,
    pub ipk: Option<String>,
    pub total_sks: Option<String>,
    pub id_sms: Option<Uuid>,
    pub id_mahasiswa: Option<Uuid>,
    pub id_agama: Option<String>,
    pub nama_agama: Option<String>,
    pub id_prodi: Option<String>,
    pub nama_program_studi: Option<String>,
    pub id_status_mahasiswa: Option<String>,
    pub nama_status_mahasiswa: Option<String>,
    pub nim: Option<String>,
    pub id_periode: Option<String>,
    pub nama_periode_masuk: Option<String>,
    pub id_registrasi_mahasiswa: Option<Uuid>,
    pub id_periode_keluar: Option<String>,
    pub tanggal_keluar: Option<Date>,
    pub last_update: Option<Date>,
    pub tgl_create: Option<Date>,
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

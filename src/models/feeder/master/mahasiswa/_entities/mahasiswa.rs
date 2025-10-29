use chrono::NaiveDate;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "feeder_master", table_name = "mahasiswa")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    pub nama_mahasiswa: String,
    pub jenis_kelamin: Option<String>,
    pub tanggal_lahir: Option<NaiveDate>,

    // uuid in DB
    pub id_perguruan_tinggi: Option<Uuid>,
    pub nipd: Option<String>,

    pub ipk: Option<f32>,

    // integer in DB
    pub total_sks: Option<i32>,

    // uuid in DB
    pub id_sms: Option<Uuid>,

    // uuid in DB (NOT varchar)
    pub id_mahasiswa: Uuid,

    // integer in DB
    pub id_agama: Option<i32>,

    pub nama_agama: Option<String>,

    // varchar(255) in DB (keep String)
    pub id_prodi: Option<String>,

    pub nama_program_studi: Option<String>,

    // integer in DB
    pub id_status_mahasiswa: Option<i32>,

    pub nama_status_mahasiswa: Option<String>,
    pub nim: Option<String>,
    pub id_periode: Option<String>,
    pub nama_periode_masuk: Option<String>,

    // uuid in DB
    pub id_registrasi_mahasiswa: Option<Uuid>,

    pub id_periode_keluar: Option<String>,
    pub tanggal_keluar: Option<NaiveDate>,
    pub last_update: Option<NaiveDate>,
    pub tgl_create: Option<NaiveDate>,
    pub status_sync: Option<String>,

    // timestamps (NaiveDateTime)
    pub sync_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

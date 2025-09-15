use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "feeder_master", table_name = "mahasiswa")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    pub nama_mahasiswa: String,
    pub jenis_kelamin: Option<String>,

    // #[serde(deserialize_with = "deserialize_date_opt")]
    pub tanggal_lahir: Option<NaiveDate>,

    pub id_perguruan_tinggi: Option<String>,
    pub nipd: Option<String>,

    // #[serde(deserialize_with = "de_opt_f32")]
    pub ipk: Option<f32>,

    // #[serde(deserialize_with = "de_opt_u32")]
    pub total_sks: Option<u32>,

    pub id_sms: Option<String>,
    pub id_mahasiswa: String,

    // #[serde(deserialize_with = "de_opt_u32")]
    pub id_agama: Option<u32>,   // FIXED

    pub nama_agama: Option<String>,
    pub id_prodi: Option<String>,
    pub nama_program_studi: Option<String>,

    // #[serde(deserialize_with = "de_opt_u32")]
    pub id_status_mahasiswa: Option<u32>,   // FIXED

    pub nama_status_mahasiswa: Option<String>,
    pub nim: Option<String>,
    pub id_periode: Option<String>,
    pub nama_periode_masuk: Option<String>,
    pub id_registrasi_mahasiswa: Option<String>,
    pub id_periode_keluar: Option<String>,

    // #[serde(deserialize_with = "deserialize_date_opt")]
    pub tanggal_keluar: Option<NaiveDate>,

    // #[serde(deserialize_with = "deserialize_date_opt")]
    pub last_update: Option<NaiveDate>,

    // #[serde(deserialize_with = "deserialize_date_opt")]
    pub tgl_create: Option<NaiveDate>,

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

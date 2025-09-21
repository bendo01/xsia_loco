use crate::library::deserialization::{de_opt_date_dmy, de_opt_i32};
use chrono::NaiveDate;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(
    schema_name = "feeder_master",
    table_name = "riwayat_pendidikan_mahasiswa"
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    // UUIDs
    pub id_registrasi_mahasiswa: Uuid,
    pub id_mahasiswa: Uuid,
    pub id_perguruan_tinggi: Uuid,
    pub id_prodi: Uuid,

    // Identifiers / names
    pub nim: String,
    pub nama_mahasiswa: String,
    pub nama_perguruan_tinggi: String,
    pub nama_program_studi: String,
    pub nama_jenis_daftar: Option<String>,
    pub keterangan_keluar: Option<String>,
    pub nama_program_studi_asal: Option<String>,
    pub nama_perguruan_tinggi_asal: Option<String>,
    pub nama_periode_masuk: String,
    pub nm_bidang_minat: Option<String>,
    pub nama_pembiayaan_awal: Option<String>,
    pub nama_ibu_kandung: String,
    pub status_sync: String,

    // Numeric codes (accept number | numeric string | null)
    #[serde(deserialize_with = "de_opt_i32")]
    pub id_jenis_daftar: Option<i32>,

    #[serde(deserialize_with = "de_opt_i32")]
    pub id_jalur_daftar: Option<i32>,

    // Period codes often come as strings in source; treat as numeric code if you prefer
    #[serde(deserialize_with = "de_opt_i32")]
    pub id_periode_masuk: Option<i32>,

    #[serde(deserialize_with = "de_opt_i32")]
    pub id_jenis_keluar: Option<i32>,

    #[serde(deserialize_with = "de_opt_i32")]
    pub id_pembiayaan: Option<i32>,

    #[serde(deserialize_with = "de_opt_i32")]
    pub id_periode_keluar: Option<i32>,

    // Optional UUIDs (may be null)
    pub id_perguruan_tinggi_asal: Option<Uuid>,
    pub id_prodi_asal: Option<Uuid>,

    // Other enums/flags stored as short strings in source
    pub jenis_kelamin: String,

    // Credits recognized: string digits or null in source → numeric here
    #[serde(deserialize_with = "de_opt_i32")]
    pub sks_diakui: Option<i32>,

    // Money/amount (source may send "0" or numbers). If values can exceed i32, consider adding a de_opt_i64 helper and switching to i64.
    #[serde(deserialize_with = "de_opt_i32")]
    pub biaya_masuk: Option<i32>,

    // Optional fields that look like categorical codes
    pub id_bidang_minat: Option<String>, // keep as String if it’s alphanumeric

    // Dates (accept dd-MM-yyyy, then fallback yyyy-MM-dd), or null/empty
    #[serde(deserialize_with = "de_opt_date_dmy")]
    pub tanggal_daftar: Option<NaiveDate>,

    #[serde(deserialize_with = "de_opt_date_dmy")]
    pub tanggal_keluar: Option<NaiveDate>,

    #[serde(deserialize_with = "de_opt_date_dmy")]
    pub last_update: Option<NaiveDate>,

    #[serde(deserialize_with = "de_opt_date_dmy")]
    pub tgl_create: Option<NaiveDate>,

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

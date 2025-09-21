use crate::library::deserialization::{de_opt_date_dmy, de_opt_i32, de_opt_string_or_int};
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

    // Basic strings
    pub nim: String,
    pub nama_mahasiswa: String,
    pub nama_perguruan_tinggi: String,
    pub nama_program_studi: String,
    pub nama_ibu_kandung: String,
    pub nama_pembiayaan_awal: String,
    pub nama_jenis_daftar: String,
    pub nama_periode_masuk: String,
    pub status_sync: String,
    pub jenis_kelamin: String,

    // Mixed “code” fields that may come as number or string → keep as String but robustly parse
    #[serde(deserialize_with = "de_opt_string_or_int")]
    pub id_jenis_daftar: Option<String>, // e.g., "1"
    #[serde(deserialize_with = "de_opt_string_or_int")]
    pub id_jalur_daftar: Option<String>, // e.g., "12"
    #[serde(deserialize_with = "de_opt_string_or_int")]
    pub id_periode_masuk: Option<String>, // e.g., "20251"
    #[serde(deserialize_with = "de_opt_string_or_int")]
    pub id_pembiayaan: Option<String>, // e.g., "1"

    // Optional exits
    pub id_jenis_keluar: Option<String>,
    pub keterangan_keluar: Option<String>,
    pub id_periode_keluar: Option<String>,

    // Origin campus/program (nullable UUIDs)
    pub id_perguruan_tinggi_asal: Option<Uuid>,
    pub nama_perguruan_tinggi_asal: Option<String>,
    pub id_prodi_asal: Option<Uuid>,
    pub nama_program_studi_asal: Option<String>,

    // Optional bidang minat (null in sample, but make it UUID if present)
    pub id_bidang_minat: Option<Uuid>,
    pub nm_bidang_minat: Option<String>,

    // Numbers with potential string inputs
    #[serde(deserialize_with = "de_opt_i32")]
    pub sks_diakui: Option<i32>, // sample shows "0" -> Some(0)

    // biaya_masuk appears as a plain number in sample; keep i64
    pub biaya_masuk: i64,

    // Dates (accept dd-MM-YYYY or YYYY-MM-DD or null)
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

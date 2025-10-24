use crate::library::deserialization::{
    de_opt_date_dmy,
    de_opt_f32,
    de_opt_i32, // <-- use i32 version
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid; // Removed unused `uuid` macro import

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelInput {
    pub nama_mahasiswa: String,
    pub jenis_kelamin: Option<String>,

    #[serde(deserialize_with = "de_opt_date_dmy")]
    pub tanggal_lahir: Option<NaiveDate>,

    // uuid in DB -> Uuid here (serde can parse from string)
    pub id_perguruan_tinggi: Option<Uuid>,
    pub nipd: Option<String>,

    #[serde(deserialize_with = "de_opt_f32")]
    pub ipk: Option<f32>,

    // integer in DB -> i32
    #[serde(deserialize_with = "de_opt_i32")]
    pub total_sks: Option<i32>,

    // uuid in DB
    pub id_sms: Option<Uuid>,

    // uuid in DB, required
    pub id_mahasiswa: Uuid,

    // integer in DB -> i32
    #[serde(deserialize_with = "de_opt_i32")]
    pub id_agama: Option<i32>,

    pub nama_agama: Option<String>,

    // varchar in DB
    pub id_prodi: Option<String>,
    pub nama_program_studi: Option<String>,

    // integer in DB -> i32
    #[serde(deserialize_with = "de_opt_i32")]
    pub id_status_mahasiswa: Option<i32>,

    pub nama_status_mahasiswa: Option<String>,
    pub nim: Option<String>,
    pub id_periode: Option<String>,
    pub nama_periode_masuk: Option<String>,

    // uuid in DB
    pub id_registrasi_mahasiswa: Option<Uuid>,

    pub id_periode_keluar: Option<String>,

    #[serde(deserialize_with = "de_opt_date_dmy")]
    pub tanggal_keluar: Option<NaiveDate>,

    #[serde(deserialize_with = "de_opt_date_dmy")]
    pub last_update: Option<NaiveDate>,

    #[serde(deserialize_with = "de_opt_date_dmy")]
    pub tgl_create: Option<NaiveDate>,

    pub status_sync: Option<String>,
}

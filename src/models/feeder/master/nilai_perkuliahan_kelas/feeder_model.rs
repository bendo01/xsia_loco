use crate::library::deserialization::{de_date_dmy, de_opt_date_dmy, de_opt_f32, de_opt_i32};
use loco_rs::prelude::Date;
use serde::{Deserialize, Serialize};
use uuid::Uuid; // Removed unused `uuid` macro import

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelInput {
    pub id_matkul: Uuid,
    pub kode_mata_kuliah: String,
    pub nama_mata_kuliah: String,
    pub id_kelas_kuliah: Uuid,
    pub nama_kelas_kuliah: Option<String>,
    #[serde(deserialize_with = "de_opt_f32")]
    pub sks_mata_kuliah: Option<f32>,
    #[serde(deserialize_with = "de_opt_i32")]
    pub jumlah_mahasiswa_krs: Option<i32>,
    #[serde(deserialize_with = "de_opt_i32")]
    pub jumlah_mahasiswa_dapat_nilai: Option<i32>,
    #[serde(deserialize_with = "de_opt_f32")]
    pub sks_tm: Option<f32>,
    #[serde(deserialize_with = "de_opt_f32")]
    pub sks_prak: Option<f32>,
    #[serde(deserialize_with = "de_opt_f32")]
    pub sks_prak_lap: Option<f32>,
    #[serde(deserialize_with = "de_opt_f32")]
    pub sks_sim: Option<f32>,
    pub bahasan_case: Option<String>,
    #[serde(deserialize_with = "de_opt_i32")]
    pub a_selenggara_pditt: Option<i32>,
    #[serde(deserialize_with = "de_opt_i32")]
    pub a_pengguna_pditt: Option<i32>,
    #[serde(deserialize_with = "de_opt_i32")]
    pub kuota_pditt: Option<i32>,
    #[serde(deserialize_with = "de_opt_date_dmy")]
    pub tgl_mulai_koas: Option<Date>,
    #[serde(deserialize_with = "de_opt_date_dmy")]
    pub tgl_selesai_koas: Option<Date>,
    pub id_mou: Option<Uuid>,
    pub id_kls_pditt: Option<Uuid>,
    pub id_sms: Uuid,
    pub id_smt: String,
    #[serde(deserialize_with = "de_date_dmy")]
    pub tgl_create: Date,
    #[serde(deserialize_with = "de_opt_i32")]
    pub lingkup_kelas: Option<i32>,
    pub mode_kuliah: Option<String>,
    pub nm_smt: String,
    pub nama_prodi: String,
    pub status_sync: String,
}

use crate::library::deserialization::{de_opt_f32, de_opt_i32, de_opt_string_or_int};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelInput {
    pub id_aktivitas_mengajar: Option<Uuid>,
    pub id_registrasi_dosen: Option<Uuid>,
    pub id_dosen: Option<Uuid>,
    pub nidn: Option<String>,
    pub nuptk: Option<String>,
    pub nama_dosen: Option<String>,
    pub id_kelas_kuliah: Option<Uuid>,
    pub nama_kelas_kuliah: Option<String>,
    pub id_substansi: Option<Uuid>,
    #[serde(deserialize_with = "de_opt_f32")]
    pub sks_substansi_total: Option<f32>,
    #[serde(deserialize_with = "de_opt_i32")]
    pub rencana_minggu_pertemuan: Option<i32>,
    #[serde(deserialize_with = "de_opt_i32")]
    pub realisasi_minggu_pertemuan: Option<i32>,
    #[serde(deserialize_with = "de_opt_string_or_int")]
    pub id_jenis_evaluasi: Option<String>,
    pub nama_jenis_evaluasi: Option<String>,
    pub id_prodi: Option<Uuid>,
    pub id_semester: Option<String>,
    pub perhitungan_sks: Option<String>,
    pub sync_at: Option<DateTime<Utc>>,
}
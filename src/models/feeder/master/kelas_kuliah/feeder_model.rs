use crate::library::deserialization::{de_f32, de_opt_date_dmy, de_opt_i32};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetDetailKelasKuliahResponse {
    #[serde(rename = "error_code")]
    pub error_code: i32,
    #[serde(rename = "error_desc")]
    pub error_desc: String,
    pub data: Vec<KelasKuliah>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KelasKuliah {
    #[serde(rename = "id_kelas_kuliah")]
    pub id_kelas_kuliah: Uuid,
    #[serde(rename = "id_prodi")]
    pub id_prodi: Uuid,
    #[serde(rename = "nama_program_studi")]
    pub nama_program_studi: String,
    #[serde(rename = "id_semester")]
    pub id_semester: String,
    #[serde(rename = "nama_semester")]
    pub nama_semester: String,
    #[serde(rename = "id_matkul")]
    pub id_matkul: Uuid,
    #[serde(rename = "kode_mata_kuliah")]
    pub kode_mata_kuliah: String,
    #[serde(rename = "nama_mata_kuliah")]
    pub nama_mata_kuliah: String,
    #[serde(rename = "sks_mk", deserialize_with = "de_f32")]
    pub sks_mk: f32,
    #[serde(rename = "sks_tm", deserialize_with = "de_f32")]
    pub sks_tm: f32,
    #[serde(rename = "sks_prak", deserialize_with = "de_f32")]
    pub sks_prak: f32,
    #[serde(rename = "sks_prak_lap", deserialize_with = "de_f32")]
    pub sks_prak_lap: f32,
    #[serde(rename = "sks_sim", deserialize_with = "de_f32")]
    pub sks_sim: f32,
    #[serde(rename = "nama_kelas_kuliah")]
    pub nama_kelas_kuliah: String,
    #[serde(rename = "bahasan")]
    pub bahasan: Option<String>,
    #[serde(rename = "tanggal_mulai_efektif", deserialize_with = "de_opt_date_dmy")]
    pub tanggal_mulai_efektif: Option<NaiveDate>,
    #[serde(rename = "tanggal_akhir_efektif", deserialize_with = "de_opt_date_dmy")]
    pub tanggal_akhir_efektif: Option<NaiveDate>,
    #[serde(rename = "kapasitas", deserialize_with = "de_opt_i32")]
    pub kapasitas: Option<i32>,
    #[serde(rename = "tanggal_tutup_daftar", deserialize_with = "de_opt_date_dmy")]
    pub tanggal_tutup_daftar: Option<NaiveDate>,
    #[serde(rename = "prodi_penyelenggara")]
    pub prodi_penyelenggara: Option<String>,
    #[serde(rename = "perguruan_tinggi_penyelenggara")]
    pub perguruan_tinggi_penyelenggara: Option<String>,
}
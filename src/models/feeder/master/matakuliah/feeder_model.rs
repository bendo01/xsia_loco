use crate::library::deserialization::{
    de_opt_date_dmy,
    de_opt_iso_datetime,
    de_opt_f32,
    de_opt_boolish,
    // de_opt_i32, // <-- use i32 version
};
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelInputDetailMatakuliah {
    pub id_matkul: Uuid,
    pub kode_mata_kuliah: String,
    pub nama_mata_kuliah: String,
    pub id_prodi: Uuid,
    pub nama_program_studi: String,
    pub id_jenis_mata_kuliah: Option<String>,
    pub id_kelompok_mata_kuliah: Option<String>,
    #[serde(deserialize_with = "de_opt_f32")]
    pub sks_mata_kuliah: Option<f32>,
    #[serde(deserialize_with = "de_opt_f32")]
    pub sks_tatap_muka: Option<f32>,
    #[serde(deserialize_with = "de_opt_f32")]
    pub sks_praktek: Option<f32>,
    #[serde(deserialize_with = "de_opt_f32")]
    pub sks_praktek_lapangan: Option<f32>,
    #[serde(deserialize_with = "de_opt_f32")]
    pub sks_simulasi: Option<f32>,
    pub metode_kuliah: Option<String>,
    #[serde(deserialize_with = "de_opt_boolish")]
    pub ada_sap: Option<bool>,
    #[serde(deserialize_with = "de_opt_boolish")]
    pub ada_silabus: Option<bool>,
    #[serde(deserialize_with = "de_opt_boolish")]
    pub ada_bahan_ajar: Option<bool>,
    #[serde(deserialize_with = "de_opt_boolish")]
    pub ada_acara_praktek: Option<bool>,
    #[serde(deserialize_with = "de_opt_boolish")]
    pub ada_diktat: Option<bool>,
    #[serde(deserialize_with = "de_opt_iso_datetime")]
    pub tanggal_mulai_efektif: Option<NaiveDateTime>,
    #[serde(deserialize_with = "de_opt_iso_datetime")]
    pub tanggal_selesai_efektif: Option<NaiveDateTime>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelInputListMatakuliah {
    pub id_jenj_didik: Option<String>,
    #[serde(deserialize_with = "de_opt_iso_datetime")]
    pub tgl_create: Option<NaiveDateTime>,
    pub id_matkul: Uuid,
    pub jns_mk: Option<String>,
    pub kel_mk: Option<String>,
    pub kode_mata_kuliah: String,
    pub nama_mata_kuliah: String,
    #[serde(deserialize_with = "de_opt_f32")]
    pub sks_mata_kuliah: Option<f32>,
    pub id_prodi: Uuid,
    pub nama_program_studi: String,
    pub id_jenis_mata_kuliah: Option<String>,
    pub id_kelompok_mata_kuliah: Option<String>,
    #[serde(deserialize_with = "de_opt_f32")]
    pub sks_tatap_muka: Option<f32>,
    #[serde(deserialize_with = "de_opt_f32")]
    pub sks_praktek: Option<f32>,
    #[serde(deserialize_with = "de_opt_f32")]
    pub sks_praktek_lapangan: Option<f32>,
    #[serde(deserialize_with = "de_opt_f32")]
    pub sks_simulasi: Option<f32>,
    pub metode_kuliah: Option<String>,
    #[serde(deserialize_with = "de_opt_boolish")]
    pub ada_sap: Option<bool>,
    #[serde(deserialize_with = "de_opt_boolish")]
    pub ada_silabus: Option<bool>,
    #[serde(deserialize_with = "de_opt_boolish")]
    pub ada_bahan_ajar: Option<bool>,
    #[serde(deserialize_with = "de_opt_boolish")]
    pub ada_acara_praktek: Option<bool>,
    #[serde(deserialize_with = "de_opt_boolish")]
    pub ada_diktat: Option<bool>,
    #[serde(deserialize_with = "de_opt_date_dmy")]
    pub tanggal_mulai_efektif: Option<NaiveDate>,
    #[serde(deserialize_with = "de_opt_date_dmy")]
    pub tanggal_selesai_efektif: Option<NaiveDate>,
    pub nama_kelompok_mata_kuliah: Option<String>,
    pub nama_jenis_mata_kuliah: Option<String>,
    pub status_sync: Option<String>,
}
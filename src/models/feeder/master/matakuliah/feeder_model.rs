use crate::library::deserialization::{
    de_opt_boolish,
    // de_opt_i32, // <-- use i32 version
    de_opt_date_dmy,
    de_opt_f32,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid; // Removed unused `uuid` macro import

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelInput {
    pub id: Option<Uuid>,
    pub id_matkul: Option<Uuid>,
    pub kode_mata_kuliah: Option<String>,
    pub nama_mata_kuliah: Option<String>,
    pub id_prodi: Option<Uuid>,
    pub nama_program_studi: Option<String>,
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
    #[serde(deserialize_with = "de_opt_date_dmy")]
    pub tanggal_mulai_efektif: Option<NaiveDate>,
    #[serde(deserialize_with = "de_opt_date_dmy")]
    pub tanggal_selesai_efektif: Option<NaiveDate>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

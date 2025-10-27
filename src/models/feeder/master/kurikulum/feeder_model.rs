use crate::library::deserialization::{de_opt_f32, de_opt_i32};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInput {
    pub id_kurikulum: Option<Uuid>,
    #[serde(deserialize_with = "de_opt_i32")]
    pub jml_sem_normal: Option<i32>,
    pub id_jenj_didik: Option<String>,
    pub nama_kurikulum: Option<String>,
    pub id_prodi: Option<Uuid>,
    pub nama_program_studi: Option<String>,
    pub id_semester: Option<String>,
    pub semester_mulai_berlaku: Option<String>,
    #[serde(deserialize_with = "de_opt_f32")]
    pub jumlah_sks_lulus: Option<f32>,
    #[serde(deserialize_with = "de_opt_f32")]
    pub jumlah_sks_wajib: Option<f32>,
    #[serde(deserialize_with = "de_opt_f32")]
    pub jumlah_sks_pilihan: Option<f32>,
    #[serde(deserialize_with = "de_opt_f32")]
    pub jumlah_sks_mata_kuliah_wajib: Option<f32>,
    #[serde(deserialize_with = "de_opt_f32")]
    pub jumlah_sks_mata_kuliah_pilihan: Option<f32>,
    pub status_sync: Option<String>,
}
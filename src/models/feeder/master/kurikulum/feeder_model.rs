use crate::library::deserialization::de_opt_f32;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInputDetailKurikulum {
    pub id_kurikulum: Uuid,
    pub nama_kurikulum: String,
    pub id_prodi: Uuid,
    pub nama_program_studi: String,
    pub id_semester: String,
    pub semester_mulai_berlaku: String,
    #[serde(deserialize_with = "de_opt_f32")]
    pub jumlah_sks_lulus: Option<f32>,
    #[serde(deserialize_with = "de_opt_f32")]
    pub jumlah_sks_wajib: Option<f32>,
    #[serde(deserialize_with = "de_opt_f32")]
    pub jumlah_sks_pilihan: Option<f32>,
    pub status_sync: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInputListKurikulum {
    #[serde(deserialize_with = "de_opt_f32")]
    pub id_jenj_didik: Option<f32>,
    #[serde(deserialize_with = "de_opt_f32")]
    pub jml_sem_normal: Option<f32>,
    pub id_kurikulum: Uuid,
    pub nama_kurikulum: String,
    pub id_prodi: Uuid,
    pub nama_program_studi: String,
    pub id_semester: String,
    pub semester_mulai_berlaku: String,
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
    pub status_sync: String,
}
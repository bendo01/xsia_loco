use crate::library::deserialization::de_opt_i32;
// use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelInput {
    pub id_registrasi_dosen: Uuid,
    pub id_dosen: Uuid,
    pub nama_dosen: String,
    pub id_periode: String,
    pub nama_periode: String,
    pub id_prodi: Uuid,
    pub nama_program_studi: String,
    pub id_matkul: Uuid,
    pub nama_mata_kuliah: String,
    pub id_kelas: Uuid,
    pub nama_kelas_kuliah: String,
    #[serde(deserialize_with = "de_opt_i32")]
    pub rencana_minggu_pertemuan: Option<i32>,
    #[serde(deserialize_with = "de_opt_i32")]
    pub realisasi_minggu_pertemuan: Option<i32>,
}

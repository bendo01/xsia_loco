use crate::library::deserialization::de_opt_f32;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelInput {
    pub id: Option<Uuid>,
    pub id_registrasi_mahasiswa: Option<Uuid>,
    pub id_matkul: Option<Uuid>,
    pub id_kelas_kuliah: Option<Uuid>,
    pub id_nilai_transfer: Option<String>,
    pub id_konversi_aktivitas: Option<String>,
    pub smt_diambil: Option<String>,
    pub kode_mata_kuliah: Option<String>,
    pub nama_mata_kuliah: Option<String>,
    #[serde(deserialize_with = "de_opt_f32")]
    pub sks_mata_kuliah: Option<f32>,
    #[serde(deserialize_with = "de_opt_f32")]
    pub nilai_angka: Option<f32>,
    pub nilai_huruf: Option<String>,
    #[serde(deserialize_with = "de_opt_f32")]
    pub nilai_indeks: Option<f32>,
}

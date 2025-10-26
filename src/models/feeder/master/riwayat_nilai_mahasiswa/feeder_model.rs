use crate::library::deserialization::de_opt_f32;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelInput {
    // UUIDs
    pub id: Option<Uuid>,
    pub id_registrasi_mahasiswa: Option<Uuid>,
    pub id_prodi: Option<Uuid>,
    pub nama_program_studi: Option<String>,

    // period
    pub id_periode: Option<String>,

    // mata kuliah / kelas
    pub id_matkul: Option<Uuid>,
    pub nama_mata_kuliah: Option<String>,
    pub id_kelas: Option<Uuid>,
    pub nama_kelas_kuliah: Option<String>,

    // Numeric fields (may come as strings) — deserialize helpers convert to Option<f32>
    #[serde(deserialize_with = "de_opt_f32")]
    pub sks_mata_kuliah: Option<f32>,

    #[serde(deserialize_with = "de_opt_f32")]
    pub nilai_angka: Option<f32>,

    pub nilai_huruf: Option<String>,

    #[serde(deserialize_with = "de_opt_f32")]
    pub nilai_indeks: Option<f32>,

    pub nim: Option<String>,
    pub nama_mahasiswa: Option<String>,
    pub angkatan: Option<String>,
    pub status_sync: Option<String>,
}

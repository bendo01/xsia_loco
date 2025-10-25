use serde::{Deserialize, Serialize};
use uuid::Uuid; // Removed unused `uuid` macro import

use crate::library::deserialization::de_opt_f32;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelInput {
    // UUIDs
    // Kunci & identitas
    pub id_transfer: Uuid,
    pub id_registrasi_mahasiswa: Uuid,
    pub id_matkul: Uuid,

    pub nim: String,
    pub nama_mahasiswa: String,

    pub id_prodi: Uuid,
    pub nama_program_studi: String,

    // Periode & semester
    #[serde(rename = "id_periode_masuk")]
    pub id_periode_masuk: String, // "20241" (string)
    pub id_semester: String,   // "20241" (string)
    pub nama_semester: String, // "2024/2025 Ganjil"

    // Mata kuliah asal
    pub kode_mata_kuliah_asal: String,
    pub nama_mata_kuliah_asal: String,
    #[serde(deserialize_with = "de_opt_f32")]
    pub sks_mata_kuliah_asal: Option<f32>,
    pub nilai_huruf_asal: Option<String>,

    // Matkul diakui (konversi)
    pub kode_matkul_diakui: String,
    pub nama_mata_kuliah_diakui: String,
    #[serde(deserialize_with = "de_opt_f32")]
    pub sks_mata_kuliah_diakui: Option<f32>,
    pub nilai_huruf_diakui: Option<String>,
    #[serde(deserialize_with = "de_opt_f32")]
    pub nilai_angka_diakui: Option<f32>, // "2.0000" → 2.0

    // Metadata/relasi opsional
    pub id_perguruan_tinggi: Option<Uuid>,
    pub id_aktivitas: Option<Uuid>,
    pub judul: Option<String>,
    pub id_jenis_aktivitas: Option<Uuid>,
    pub nama_jenis_aktivitas: Option<String>,

    // Status
    pub status_sync: String,
}

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(
    schema_name = "feeder_master",
    table_name = "nilai_transfer_pendidikan_mahasiswa"
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub id_transfer: Uuid,
    pub id_registrasi_mahasiswa: Uuid,
    pub id_matkul: Uuid,

    pub nim: String,
    pub nama_mahasiswa: String,

    pub id_prodi: Uuid,
    pub nama_program_studi: String,

    // Periode & semester
    #[serde(rename = "id_periode_masuk")]
    pub id_periode_masuk: String,     // "20241" (string)
    pub id_semester: String,          // "20241" (string)
    pub nama_semester: String,        // "2024/2025 Ganjil"

    // Mata kuliah asal
    pub kode_mata_kuliah_asal: String,
    pub nama_mata_kuliah_asal: String,
    pub sks_mata_kuliah_asal: Option<f32>,
    pub nilai_huruf_asal: Option<String>,

    // Matkul diakui (konversi)
    pub kode_matkul_diakui: String,
    pub nama_mata_kuliah_diakui: String,
    pub sks_mata_kuliah_diakui: Option<f32>,
    pub nilai_huruf_diakui: Option<String>,
    pub nilai_angka_diakui: Option<f32>, // "2.0000" → 2.0

    // Metadata/relasi opsional
    pub id_perguruan_tinggi: Option<Uuid>,
    pub id_aktivitas: Option<Uuid>,
    pub judul: Option<String>,
    pub id_jenis_aktivitas: Option<Uuid>,
    pub nama_jenis_aktivitas: Option<String>,

    // Status
    pub status_sync: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub sync_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(
    schema_name = "feeder_master",
    table_name = "transfer_pendidikan_mahasiswa"
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub id_transfer: Option<Uuid>,
    pub id_registrasi_mahasiswa: Option<Uuid>,
    pub nim: Option<String>,
    pub nama_mahasiswa: Option<String>,
    pub id_prodi: Option<Uuid>,
    pub nama_program_studi: Option<String>,
    pub id_periode_masuk: Option<String>,
    pub kode_mata_kuliah_asal: Option<String>,
    pub nama_mata_kuliah_asal: Option<String>,
    pub sks_mata_kuliah_asal: Option<f32>,
    pub nilai_huruf_asal: Option<String>,
    pub id_matkul: Option<Uuid>,
    pub kode_matkul_diakui: Option<String>,
    pub nama_mata_kuliah_diakui: Option<String>,
    pub sks_mata_kuliah_diakui: Option<f32>,
    pub nilai_huruf_diakui: Option<String>,
    pub nilai_angka_diakui: Option<f32>,
    pub id_perguruan_tinggi: Option<Uuid>,
    pub id_aktivitas: Option<String>,
    pub judul: Option<String>,
    pub id_jenis_aktivitas: Option<String>,
    pub nama_jenis_aktivitas: Option<String>,
    pub id_semester: Option<String>,
    pub nama_semester: Option<String>,
    pub status_sync: Option<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub sync_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

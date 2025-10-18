use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "feeder_master", table_name = "riwayat_nilai_mahasiswa")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub id_registrasi_mahasiswa: Option<Uuid>,
    pub id_prodi: Option<Uuid>,
    pub nama_program_studi: Option<String>,
    pub id_periode: Option<String>,
    pub id_matkul: Option<Uuid>,
    pub nama_mata_kuliah: Option<String>,
    pub id_kelas: Option<Uuid>,
    pub nama_kelas_kuliah: Option<String>,
    pub sks_mata_kuliah: Option<f32>,
    pub nilai_angka: Option<f32>,
    pub nilai_huruf: Option<String>,
    pub nilai_indeks: Option<f32>,
    pub nim: Option<String>,
    pub nama_mahasiswa: Option<String>,
    pub angkatan: Option<String>,
    pub status_sync: Option<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "feeder_master", table_name = "transkrip_mahasiswa")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub id_registrasi_mahasiswa: Option<Uuid>,
    pub id_matkul: Option<Uuid>,
    pub id_kelas_kuliah: Option<Uuid>,
    pub id_nilai_transfer: Option<String>,
    pub id_konversi_aktivitas: Option<String>,
    pub smt_diambil: Option<String>,
    pub kode_mata_kuliah: Option<String>,
    pub nama_mata_kuliah: Option<String>,
    pub sks_mata_kuliah: Option<f32>,
    pub nilai_angka: Option<f32>,
    pub nilai_huruf: Option<String>,
    pub nilai_indeks: Option<f32>,
    pub sync_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

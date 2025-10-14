use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(
    schema_name = "feeder_master",
    table_name = "dosen_pengajar_kelas_kuliah"
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub id_aktivitas_mengajar: Option<Uuid>,
    pub id_registrasi_dosen: Option<Uuid>,
    pub id_dosen: Option<Uuid>,
    pub nidn: Option<String>,
    pub nama_dosen: Option<String>,
    pub id_kelas_kuliah: Option<Uuid>,
    pub nama_kelas_kuliah: Option<String>,
    pub id_substansi: Option<Uuid>,
    pub sks_substansi_total: Option<f32>,
    pub rencana_minggu_pertemuan: Option<i32>,
    pub realisasi_minggu_pertemuan: Option<i32>,
    pub id_jenis_evaluasi: Option<String>,
    pub nama_jenis_evaluasi: Option<String>,
    pub id_prodi: Option<Uuid>,
    pub id_semester: Option<String>,
    pub perhitungan_sks: Option<String>,
    pub sync_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

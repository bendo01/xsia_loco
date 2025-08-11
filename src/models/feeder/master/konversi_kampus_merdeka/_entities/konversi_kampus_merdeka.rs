use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "feeder_master", table_name = "konversi_kampus_merdeka")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub id_semester: Option<Uuid>,
    pub nama_semester: Option<String>,
    pub id_konversi_aktivitas: Option<Uuid>,
    pub id_matkul: Option<Uuid>,
    pub nama_mata_kuliah: Option<String>,
    pub id_aktivitas: Option<Uuid>,
    pub judul: Option<String>,
    pub id_anggota: Option<Uuid>,
    pub nama_mahasiswa: Option<String>,
    pub nim: Option<i32>,
    pub sks_mata_kuliah: Option<i32>,
    pub nilai_angka: Option<i32>,
    pub nilai_indeks: Option<i32>,
    pub nilai_huruf: Option<i32>,
    pub sync_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

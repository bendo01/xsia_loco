use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(
    schema_name = "feeder_master",
    table_name = "mahasiswa_lulusan_dropout"
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub id_registrasi_mahasiswa: Option<Uuid>,
    pub id_mahasiswa: Option<Uuid>,
    pub nim: Option<String>,
    pub nama_mahasiswa: Option<String>,
    pub id_jenis_keluar: Option<Uuid>,
    pub nama_jenis_keluar: Option<String>,
    pub tanggal_keluar: Option<Date>,
    pub keterangan: Option<String>,
    pub nomor_sk_yudisium: Option<String>,
    pub tanggal_sk_yudisium: Option<Date>,
    pub ipk: Option<i32>,
    pub nomor_ijazah: Option<i32>,
    pub jalur_skripsi: Option<String>,
    pub judul_skripsi: Option<String>,
    pub bulan_awal_bimbingan: Option<Date>,
    pub bulan_akhir_bimbingan: Option<Date>,
    pub id_dosen: Option<Uuid>,
    pub nidn: Option<String>,
    pub nama_dosen: Option<String>,
    pub pembimbing_ke: Option<i32>,
    pub id_periode_keluar: Option<Uuid>,
    pub sync_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

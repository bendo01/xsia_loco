use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "feeder_master", table_name = "riwayat_pendidikan_mahasiswa")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub id_registrasi_mahasiswa: Uuid,
    pub id_mahasiswa: Uuid,
    pub nim: String,
    pub nama_mahasiswa: String,
    pub id_jenis_daftar: i32,
    pub nama_jenis_daftar: String,
    pub id_jalur_daftar: i32,
    pub id_periode_masuk: String,
    pub nama_periode_masuk: String,
    pub id_jenis_keluar: i32,
    pub keterangan_keluar: String,
    pub id_perguruan_tinggi: Uuid,
    pub nama_perguruan_tinggi: String,
    pub id_prodi: Uuid,
    pub nama_program_studi: String,
    pub sks_diakui: i32,
    pub id_perguruan_tinggi_asal: Uuid,
    pub nama_perguruan_tinggi_asal: String,
    pub id_prodi_asal: Uuid,
    pub nama_program_studi_asal: String,
    pub jenis_kelamin: String,
    pub tanggal_daftar: Date,
    pub nama_ibu_kandung: String,
    pub id_pembiayaan: i32,
    pub nama_pembiayaan_awal: String,
    pub biaya_masuk: i32,
    pub id_bidang_minat: String,
    pub nm_bidang_minat: String,
    pub id_periode_keluar: String,
    pub tanggal_keluar: Date,
    pub last_update: Date,
    pub tgl_create: Date,
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

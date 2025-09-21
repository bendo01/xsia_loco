use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use uuid::Uuid;
use crate::library::deserialization::{
    deserialize_date_opt, de_opt_i32,
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "feeder_master", table_name = "riwayat_pendidikan_mahasiswa")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    id_registrasi_mahasiswa: Option<Uuid>,
    id_mahasiswa: Option<Uuid>,
    nim: Option<String>,
    nama_mahasiswa: Option<String>,
    #[serde(deserialize_with = "de_opt_i32")]
    id_jenis_daftar: Option<i32>,
    nama_jenis_daftar: Option<String>,
    #[serde(deserialize_with = "de_opt_i32")]
    id_jalur_daftar: Option<i32>,
    id_periode_masuk: Option<String>,
    nama_periode_masuk: Option<String>,
    #[serde(deserialize_with = "de_opt_i32")]
    id_jenis_keluar: Option<i32>,
    keterangan_keluar: Option<String>,
    id_perguruan_tinggi: Option<Uuid>,
    nama_perguruan_tinggi: Option<String>,
    id_prodi: Option<Uuid>,
    nama_program_studi: Option<String>,
    #[serde(deserialize_with = "de_opt_i32")]
    sks_diakui: Option<i32>,
    id_perguruan_tinggi_asal: Option<Uuid>,
    nama_perguruan_tinggi_asal: Option<String>,
    id_prodi_asal: Option<Uuid>,
    nama_program_studi_asal: Option<String>,
    jenis_kelamin: Option<String>,
    tanggal_daftar: Option<NaiveDate>,
    nama_ibu_kandung: Option<String>,
    #[serde(deserialize_with = "de_opt_i32")]
    id_pembiayaan: Option<i32>,
    nama_pembiayaan_awal: Option<String>,
    #[serde(deserialize_with = "de_opt_i32")]
    biaya_masuk: Option<i32>,
    id_bidang_minat: Option<String>,
    nm_bidang_minat: Option<String>,
    id_periode_keluar: Option<String>,
    #[serde(deserialize_with = "deserialize_date_opt")]
    tanggal_keluar: Option<NaiveDate>,
    #[serde(deserialize_with = "deserialize_date_opt")]
    last_update: Option<NaiveDate>,
    #[serde(deserialize_with = "deserialize_date_opt")]
    tgl_create: Option<NaiveDate>,
    status_sync: Option<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub sync_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

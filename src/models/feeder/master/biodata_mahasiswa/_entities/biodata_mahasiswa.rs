use crate::library::deserialization::{de_opt_boolish, de_opt_date_dmy, de_opt_i32};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "feeder_master", table_name = "biodata_mahasiswa")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub nama_mahasiswa: Option<String>,
    pub jenis_kelamin: Option<String>,
    pub tempat_lahir: Option<String>,
    #[serde(deserialize_with = "de_opt_date_dmy")]
    pub tanggal_lahir: Option<chrono::NaiveDate>,
    pub id_mahasiswa: Option<uuid::Uuid>,
    #[serde(deserialize_with = "de_opt_i32")]
    pub id_agama: Option<i32>,
    pub nama_agama: Option<String>,
    pub nik: Option<String>,
    pub nisn: Option<String>,
    pub npwp: Option<String>,
    pub id_negara: Option<String>,
    pub kewarganegaraan: Option<String>,
    pub jalan: Option<String>,
    pub dusun: Option<String>,
    #[serde(deserialize_with = "de_opt_i32")]
    pub rt: Option<i32>,
    #[serde(deserialize_with = "de_opt_i32")]
    pub rw: Option<i32>,
    pub kelurahan: Option<String>,
    pub kode_pos: Option<String>,
    pub id_wilayah: Option<String>,
    pub nama_wilayah: Option<String>,
    pub id_jenis_tinggal: Option<String>,
    pub nama_jenis_tinggal: Option<String>,
    pub id_alat_transportasi: Option<String>,
    pub nama_alat_transportasi: Option<String>,
    pub telepon: Option<String>,
    pub handphone: Option<String>,
    pub email: Option<String>,
    #[serde(deserialize_with = "de_opt_boolish")]
    pub penerima_kps: Option<bool>,
    pub nomor_kps: Option<String>,
    pub nik_ayah: Option<String>,
    pub nama_ayah: Option<String>,
    #[serde(deserialize_with = "de_opt_date_dmy")]
    pub tanggal_lahir_ayah: Option<chrono::NaiveDate>,
    #[serde(deserialize_with = "de_opt_i32")]
    pub id_pendidikan_ayah: Option<i32>,
    pub nama_pendidikan_ayah: Option<String>,
    #[serde(deserialize_with = "de_opt_i32")]
    pub id_pekerjaan_ayah: Option<i32>,
    pub nama_pekerjaan_ayah: Option<String>,
    #[serde(deserialize_with = "de_opt_i32")]
    pub id_penghasilan_ayah: Option<i32>,
    pub nama_penghasilan_ayah: Option<String>,
    pub nik_ibu: Option<String>,
    pub nama_ibu_kandung: Option<String>,
    #[serde(deserialize_with = "de_opt_date_dmy")]
    pub tanggal_lahir_ibu: Option<chrono::NaiveDate>,
    #[serde(deserialize_with = "de_opt_i32")]
    pub id_pendidikan_ibu: Option<i32>,
    pub nama_pendidikan_ibu: Option<String>,
    #[serde(deserialize_with = "de_opt_i32")]
    pub id_pekerjaan_ibu: Option<i32>,
    pub nama_pekerjaan_ibu: Option<String>,
    #[serde(deserialize_with = "de_opt_i32")]
    pub id_penghasilan_ibu: Option<i32>,
    pub nama_penghasilan_ibu: Option<String>,
    pub nama_wali: Option<String>,
    #[serde(deserialize_with = "de_opt_date_dmy")]
    pub tanggal_lahir_wali: Option<chrono::NaiveDate>,
    #[serde(deserialize_with = "de_opt_i32")]
    pub id_pendidikan_wali: Option<i32>,
    pub nama_pendidikan_wali: Option<String>,
    #[serde(deserialize_with = "de_opt_i32")]
    pub id_pekerjaan_wali: Option<i32>,
    pub nama_pekerjaan_wali: Option<String>,
    #[serde(deserialize_with = "de_opt_i32")]
    pub id_penghasilan_wali: Option<i32>,
    pub nama_penghasilan_wali: Option<String>,
    #[serde(deserialize_with = "de_opt_i32")]
    pub id_kebutuhan_khusus_mahasiswa: Option<i32>,
    pub nama_kebutuhan_khusus_mahasiswa: Option<String>,
    #[serde(deserialize_with = "de_opt_i32")]
    pub id_kebutuhan_khusus_ayah: Option<i32>,
    pub nama_kebutuhan_khusus_ayah: Option<String>,
    #[serde(deserialize_with = "de_opt_i32")]
    pub id_kebutuhan_khusus_ibu: Option<i32>,
    pub nama_kebutuhan_khusus_ibu: Option<String>,
    pub status_sync: Option<String>,
    pub sync_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

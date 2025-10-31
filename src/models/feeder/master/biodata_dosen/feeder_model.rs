use crate::library::deserialization::{de_opt_date_dmy, de_opt_iso_tanggal, de_opt_i32};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInput {
    pub id_dosen: Option<String>,
    pub nama_dosen: Option<String>,
    pub tempat_lahir: Option<String>,
    #[serde(deserialize_with = "de_opt_date_dmy")]
    pub tanggal_lahir: Option<chrono::NaiveDate>,
    pub jenis_kelamin: Option<String>,
    #[serde(deserialize_with = "de_opt_i32")]
    pub id_agama: Option<i32>,
    pub nama_agama: Option<String>,
    pub id_status_aktif: Option<String>,
    pub nama_status_aktif: Option<String>,
    pub nidn: Option<String>,
    pub nuptk: Option<String>,
    pub nama_ibu_kandung: Option<String>,
    pub nik: Option<String>,
    pub nip: Option<String>,
    pub npwp: Option<String>,
    pub id_jenis_sdm: Option<String>,
    pub nama_jenis_sdm: Option<String>,
    pub no_sk_cpns: Option<String>,
    #[serde(deserialize_with = "de_opt_iso_tanggal")]
    pub tanggal_sk_cpns: Option<chrono::NaiveDate>,
    pub no_sk_pengangkatan: Option<String>,
    #[serde(deserialize_with = "de_opt_iso_tanggal")]
    pub mulai_sk_pengangkatan: Option<chrono::NaiveDate>,
    pub id_lembaga_pengangkatan: Option<String>,
    pub nama_lembaga_pengangkatan: Option<String>,
    pub id_pangkat_golongan: Option<String>,
    pub nama_pangkat_golongan: Option<String>,
    pub id_sumber_gaji: Option<String>,
    pub nama_sumber_gaji: Option<String>,
    pub jalan: Option<String>,
    pub dusun: Option<String>,
    pub rt: Option<String>,
    pub rw: Option<String>,
    pub ds_kel: Option<String>,
    pub kode_pos: Option<String>,
    pub id_wilayah: Option<String>,
    pub nama_wilayah: Option<String>,
    pub telepon: Option<String>,
    pub handphone: Option<String>,
    pub email: Option<String>,
    pub status_pernikahan: Option<String>,
    pub nama_suami_istri: Option<String>,
    pub nip_suami_istri: Option<String>,
    #[serde(deserialize_with = "de_opt_iso_tanggal")]
    pub tanggal_mulai_pns: Option<chrono::NaiveDate>,
    #[serde(deserialize_with = "de_opt_i32")]
    pub id_pekerjaan_suami_istri: Option<i32>,
    pub nama_pekerjaan_suami_istri: Option<String>,
}

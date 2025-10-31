use crate::library::deserialization::{de_opt_date_dmy, de_opt_i32, de_opt_iso_tanggal};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInputDetailPenugasanDosen {
    pub id_registrasi_dosen: Option<Uuid>,
    pub id_tahun_ajaran: Option<String>,
    pub nama_tahun_ajaran: Option<String>,
    pub id_perguruan_tinggi: Option<Uuid>,
    pub nama_perguruan_tinggi: Option<String>,
    pub nidn: Option<String>,
    pub nuptk: Option<String>,
    pub id_dosen: Option<Uuid>,
    pub nama_dosen: Option<String>,
    pub id_prodi: Option<Uuid>,
    pub nama_program_studi: Option<String>,
    pub nomor_surat_tugas: Option<String>,
    #[serde(deserialize_with = "de_opt_date_dmy")]
    pub tanggal_surat_tugas: Option<NaiveDate>,
    #[serde(deserialize_with = "de_opt_date_dmy")]
    pub mulai_surat_tugas: Option<NaiveDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInputListPenugasanDosen {
    pub id_registrasi_dosen: Option<Uuid>,
    #[serde(rename = "jk")]
    pub jenis_kelamin: Option<String>,
    pub id_dosen: Option<Uuid>,
    pub nama_dosen: Option<String>,
    pub nidn: Option<String>,
    pub nuptk: Option<String>,
    pub id_tahun_ajaran: Option<String>,
    pub nama_tahun_ajaran: Option<String>,
    pub id_perguruan_tinggi: Option<Uuid>,
    pub nama_perguruan_tinggi: Option<String>,
    pub id_prodi: Option<Uuid>,
    pub nama_program_studi: Option<String>,
    pub nomor_surat_tugas: Option<String>,
    #[serde(deserialize_with = "de_opt_date_dmy")]
    pub tanggal_surat_tugas: Option<NaiveDate>,
    #[serde(deserialize_with = "de_opt_date_dmy")]
    pub mulai_surat_tugas: Option<NaiveDate>,
    #[serde(deserialize_with = "de_opt_date_dmy")]
    pub tgl_create: Option<NaiveDate>,
    #[serde(deserialize_with = "de_opt_iso_tanggal")]
    pub tgl_ptk_keluar: Option<NaiveDate>,
    #[serde(deserialize_with = "de_opt_i32")]
    pub id_stat_pegawai: Option<i32>,
    pub id_jns_keluar: Option<String>,
    pub id_ikatan_kerja: Option<String>,
    #[serde(rename = "a_sp_homebase")]
    pub apakah_homebase: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInputListPenugasanSemuaDosen {
    pub id_registrasi_dosen: Option<Uuid>,
    pub id_dosen: Option<Uuid>,
    pub nama_dosen: Option<String>,
    pub nuptk: Option<String>,
    pub jenis_kelamin: Option<String>,
    pub id_tahun_ajaran: Option<String>,
    pub nama_tahun_ajaran: Option<String>,
    pub id_prodi: Option<Uuid>,
    #[serde(rename = "program_studi")]
    pub nama_program_studi: Option<String>,
    pub nomor_surat_tugas: Option<String>,
    #[serde(deserialize_with = "de_opt_date_dmy")]
    pub tanggal_surat_tugas: Option<NaiveDate>,
    pub apakah_homebase: Option<String>,
}

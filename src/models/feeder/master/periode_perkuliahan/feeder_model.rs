use crate::library::deserialization::{de_opt_date_dmy, de_opt_date_time_iso, de_opt_i32};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Model for GetListPeriodePerkuliahan API response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInputListPeriodePerkuliahan {
    pub id_prodi: Uuid,
    pub nama_program_studi: String,
    pub id_semester: String,
    pub nama_semester: String,
    #[serde(deserialize_with = "de_opt_i32")]
    pub jumlah_target_mahasiswa_baru: Option<i32>,
    #[serde(deserialize_with = "de_opt_date_dmy")]
    pub tanggal_awal_perkuliahan: Option<NaiveDate>,
    #[serde(deserialize_with = "de_opt_date_dmy")]
    pub tanggal_akhir_perkuliahan: Option<NaiveDate>,
    #[serde(deserialize_with = "de_opt_i32")]
    pub calon_ikut_seleksi: Option<i32>,
    #[serde(deserialize_with = "de_opt_i32")]
    pub calon_lulus_seleksi: Option<i32>,
    #[serde(deserialize_with = "de_opt_i32")]
    pub daftar_sbg_mhs: Option<i32>,
    #[serde(deserialize_with = "de_opt_i32")]
    pub pst_undur_diri: Option<i32>,
    #[serde(deserialize_with = "de_opt_i32")]
    pub jml_mgu_kul: Option<i32>,
    pub metode_kul: Option<String>,
    pub metode_kul_eks: Option<String>,
    #[serde(deserialize_with = "de_opt_date_dmy")]
    pub tgl_create: Option<NaiveDate>,
    #[serde(deserialize_with = "de_opt_date_dmy")]
    pub last_update: Option<NaiveDate>,
    pub status_sync: String,
}

/// Model for GetDetailPeriodePerkuliahan API response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInputDetailPeriodePerkuliahan {
    pub id_prodi: Uuid,
    pub nama_program_studi: String,
    pub id_semester: String,
    pub nama_semester: String,
    #[serde(deserialize_with = "de_opt_i32")]
    pub jumlah_target_mahasiswa_baru: Option<i32>,
    #[serde(deserialize_with = "de_opt_i32")]
    pub jumlah_pendaftar_ikut_seleksi: Option<i32>,
    #[serde(deserialize_with = "de_opt_i32")]
    pub jumlah_pendaftar_lulus_seleksi: Option<i32>,
    #[serde(deserialize_with = "de_opt_i32")]
    pub jumlah_daftar_ulang: Option<i32>,
    #[serde(deserialize_with = "de_opt_i32")]
    pub jumlah_mengundurkan_diri: Option<i32>,
    #[serde(deserialize_with = "de_opt_date_dmy")]
    pub tanggal_awal_perkuliahan: Option<NaiveDate>,
    #[serde(deserialize_with = "de_opt_date_dmy")]
    pub tanggal_akhir_perkuliahan: Option<NaiveDate>,
    #[serde(deserialize_with = "de_opt_i32")]
    pub jumlah_minggu_pertemuan: Option<i32>,
    pub status_sync: String,
}

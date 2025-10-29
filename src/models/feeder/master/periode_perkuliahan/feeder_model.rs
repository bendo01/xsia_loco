use crate::library::deserialization::{de_opt_date_dmy, de_opt_date_time_iso, de_opt_i32};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInputListPeriodePerkuliahan {
    pub id_prodi: Option<String>,
    pub nama_program_studi: Option<String>,
    pub id_semester: Option<String>,
    pub nama_semester: Option<String>,
    pub jumlah_target_mahasiswa_baru: Option<String>,
    pub tanggal_awal_perkuliahan: Option<String>,
    pub tanggal_akhir_perkuliahan: Option<String>,
    pub calon_ikut_seleksi: Option<String>,
    pub calon_lulus_seleksi: Option<String>,
    pub daftar_sbg_mhs: Option<String>,
    pub pst_undur_diri: Option<String>,
    pub jml_mgu_kul: Option<String>,
    pub metode_kul: Option<String>,
    pub metode_kul_eks: Option<String>,
    pub tgl_create: Option<String>,
    pub last_update: Option<String>,
    pub status_sync: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInputPeriodePerkuliahan {
    pub id_prodi: Option<String>,
    pub nama_program_studi: Option<String>,
    pub id_semester: Option<String>,
    pub nama_semester: Option<String>,
    pub jumlah_target_mahasiswa_baru: Option<String>,
    pub tanggal_awal_perkuliahan: Option<String>,
    pub tanggal_akhir_perkuliahan: Option<String>,
    pub jumlah_pendaftar_ikut_seleksi: Option<String>,
    pub jumlah_pendaftar_lulus_seleksi: Option<String>,
    pub jumlah_daftar_ulang: Option<String>,
    pub jumlah_mengundurkan_diri: Option<String>,
    pub jumlah_minggu_pertemuan: Option<String>,
    pub metode_kul: Option<String>,
    pub metode_kul_eks: Option<String>,
    pub tgl_create: Option<String>,
    pub last_update: Option<String>,
    pub status_sync: Option<String>,
}
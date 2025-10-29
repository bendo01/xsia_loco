use crate::library::deserialization::{
    de_opt_date_dmy,
    de_opt_f32,
    // de_opt_i32, // <-- use i32 version
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid; // Removed unused `uuid` macro import

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelInput {
    pub id_registrasi_mahasiswa: Option<Uuid>,
    pub id_mahasiswa: Option<Uuid>,
    pub id_perguruan_tinggi: Option<Uuid>,
    pub id_prodi: Option<Uuid>,
    #[serde(deserialize_with = "de_opt_date_dmy")]
    pub tgl_masuk_sp: Option<NaiveDate>,
    #[serde(deserialize_with = "de_opt_date_dmy")]
    pub tgl_keluar: Option<NaiveDate>,
    pub skhun: Option<String>,
    pub no_peserta_ujian: Option<String>,
    pub no_seri_ijazah: Option<String>,
    #[serde(deserialize_with = "de_opt_date_dmy")]
    pub tgl_create: Option<NaiveDate>,
    #[serde(deserialize_with = "de_opt_f32")]
    pub sks_diakui: Option<f32>,
    pub jalur_skripsi: Option<String>,
    pub judul_skripsi: Option<String>,
    pub bln_awal_bimbingan: Option<String>,
    pub bln_akhir_bimbingan: Option<String>,
    pub sk_yudisium: Option<String>,
    #[serde(deserialize_with = "de_opt_date_dmy")]
    pub tgl_sk_yudisium: Option<NaiveDate>,
    #[serde(deserialize_with = "de_opt_f32")]
    pub ipk: Option<f32>,
    pub sert_prof: Option<String>,
    pub a_pindah_mhs_asing: Option<String>,
    pub id_pt_asal: Option<Uuid>,
    pub id_prodi_asal: Option<Uuid>,
    pub nm_pt_asal: Option<String>,
    pub nm_prodi_asal: Option<String>,
    pub id_jns_daftar: Option<String>,
    pub id_jns_keluar: Option<String>,
    pub id_jalur_masuk: Option<String>,
    pub id_pembiayaan: Option<String>,
    pub id_minat_bidang: Option<String>,
    pub bidang_mayor: Option<String>,
    pub bidang_minor: Option<String>,
    #[serde(deserialize_with = "de_opt_f32")]
    pub biaya_masuk_kuliah: Option<f32>,
    pub namapt: Option<String>,
    pub id_jur: Option<String>,
    pub nm_jns_daftar: Option<String>,
    pub nm_smt: Option<String>,
    pub nim: Option<String>,
    pub nama_mahasiswa: Option<String>,
    pub nama_program_studi: Option<String>,
    pub angkatan: Option<String>,
    pub id_jenis_keluar: Option<String>,
    pub nama_jenis_keluar: Option<String>,
    #[serde(deserialize_with = "de_opt_date_dmy")]
    pub tanggal_keluar: Option<NaiveDate>,
    pub id_periode_keluar: Option<String>,
    pub keterangan: Option<String>,
    pub no_sertifikat_profesi: Option<String>,
    #[serde(deserialize_with = "de_opt_date_dmy")]
    pub tanggal_terbit_ijazah: Option<NaiveDate>,
    pub status_sync: Option<String>,
}

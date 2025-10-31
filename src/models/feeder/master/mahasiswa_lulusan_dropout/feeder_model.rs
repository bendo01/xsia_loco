use crate::library::deserialization::{
    de_opt_date_dmy,
    de_opt_f32,
    // de_opt_i32, // <-- use i32 version
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid; // Removed unused `uuid` macro import

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelInputDetailMahasiswaLulusDO {
    pub id_registrasi_mahasiswa: Uuid,
    pub id_prodi: Uuid,
    pub nama_program_studi: String,
    pub id_mahasiswa: Uuid,
    pub nim: String,
    pub nama_mahasiswa: String,
    pub angkatan: String,
    pub id_jenis_keluar: String,
    pub nama_jenis_keluar: String,
    #[serde(deserialize_with = "de_opt_date_dmy")]
    pub tanggal_keluar: Option<NaiveDate>,
    pub keterangan: Option<String>,
    pub nomor_sk_yudisium: Option<String>,
    #[serde(deserialize_with = "de_opt_date_dmy")]
    pub tanggal_sk_yudisium: Option<NaiveDate>,
    #[serde(deserialize_with = "de_opt_f32")]
    pub ipk: Option<f32>,
    pub nomor_ijazah: Option<String>,
    pub jalur_skripsi: Option<String>,
    pub judul_skripsi: Option<String>,
    pub no_sertifikat_profesi: Option<String>,
    pub bulan_awal_bimbingan: Option<String>,
    pub bulan_akhir_bimbingan: Option<String>,
    pub id_dosen: Option<Uuid>,
    pub nidn: Option<String>,
    pub nuptk: Option<String>,
    pub nama_dosen: Option<String>,
    pub pembimbing_ke: Option<i32>,
    pub asal_ijazah: String,
    pub id_periode_keluar: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelInputListMahasiswaLulusDO {
    pub id_registrasi_mahasiswa: Uuid,
    pub id_mahasiswa: Uuid,
    pub id_perguruan_tinggi: Uuid,
    pub id_prodi: Uuid,
    #[serde(deserialize_with = "de_opt_date_dmy")]
    pub tgl_masuk_sp: Option<NaiveDate>,
    #[serde(deserialize_with = "de_opt_date_dmy")]
    pub tgl_keluar: Option<NaiveDate>,
    pub skhun: Option<String>,
    pub no_peserta_ujian: Option<String>,
    pub no_seri_ijazah: Option<String>,
    #[serde(deserialize_with = "de_opt_date_dmy")]
    pub tgl_create: Option<NaiveDate>,
    pub sks_diakui: Option<String>,
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
    pub id_jns_daftar: String,
    pub id_jns_keluar: String,
    pub id_jalur_masuk: String,
    pub id_pembiayaan: Option<String>,
    pub id_minat_bidang: Option<String>,
    pub bidang_mayor: Option<String>,
    pub bidang_minor: Option<String>,
    pub biaya_masuk_kuliah: String,
    pub namapt: String,
    pub id_jur: String,
    pub nm_jns_daftar: String,
    pub nm_smt: String,
    pub nim: String,
    pub nama_mahasiswa: String,
    pub nama_program_studi: String,
    pub angkatan: String,
    #[serde(deserialize_with = "de_opt_date_dmy")]
    pub tanggal_keluar: Option<NaiveDate>,
    pub id_periode_keluar: String,
    pub keterangan: Option<String>,
    pub no_sertifikat_profesi: Option<String>,
    #[serde(deserialize_with = "de_opt_date_dmy")]
    pub tanggal_terbit_ijazah: Option<NaiveDate>,
    pub status_sync: String,
    pub nama_jenis_keluar: String,
}
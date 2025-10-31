use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(
    schema_name = "feeder_master",
    table_name = "mahasiswa_lulusan_dropout"
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub sync_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    
    // Registration identifiers
    pub id_registrasi_mahasiswa: Option<Uuid>,
    pub id_mahasiswa: Option<Uuid>,
    pub id_perguruan_tinggi: Option<Uuid>,
    pub id_prodi: Option<Uuid>,
    
    // Student information
    pub nama_program_studi: Option<String>,
    pub nim: Option<String>,
    pub nama_mahasiswa: Option<String>,
    pub angkatan: Option<String>,
    
    // Entry dates
    pub tgl_masuk_sp: Option<Date>,
    pub tgl_create: Option<Date>,
    
    // Exit information
    pub tgl_keluar: Option<Date>,
    pub tanggal_keluar: Option<Date>,
    pub id_jenis_keluar: String,
    pub nama_jenis_keluar: String,
    pub id_periode_keluar: String,
    pub keterangan: Option<String>,
    
    // Graduation decree
    pub nomor_sk_yudisium: Option<String>,
    pub tanggal_sk_yudisium: Option<Date>,
    
    // Academic performance
    pub ipk: Option<f32>,
    
    // Certificate information
    pub nomor_ijazah: Option<String>,
    pub asal_ijazah: String,
    pub no_sertifikat_profesi: Option<String>,
    pub tanggal_terbit_ijazah: Option<Date>,
    
    // Thesis information
    pub jalur_skripsi: Option<String>,
    pub judul_skripsi: Option<String>,
    
    // Guidance period
    pub bulan_awal_bimbingan: Option<String>,
    pub bulan_akhir_bimbingan: Option<String>,
    
    // Advisor information
    pub id_dosen: Option<Uuid>,
    pub nidn: Option<String>,
    pub nuptk: Option<String>,
    pub nama_dosen: Option<String>,
    pub pembimbing_ke: Option<i32>,
    
    // Admission information
    pub skhun: Option<String>,
    pub no_peserta_ujian: Option<String>,
    pub sks_diakui: Option<String>,
    pub id_jns_daftar: Option<String>,
    pub nm_jns_daftar: Option<String>,
    pub id_jalur_masuk: Option<String>,
    pub id_pembiayaan: Option<String>,
    pub biaya_masuk_kuliah: Option<String>,
    
    // Interest fields
    pub id_minat_bidang: Option<String>,
    pub bidang_mayor: Option<String>,
    pub bidang_minor: Option<String>,
    
    // Transfer student information
    pub a_pindah_mhs_asing: Option<String>,
    pub id_pt_asal: Option<Uuid>,
    pub id_prodi_asal: Option<Uuid>,
    pub nm_pt_asal: Option<String>,
    pub nm_prodi_asal: Option<String>,
    
    // Institution information
    pub namapt: Option<String>,
    pub id_jur: Option<String>,
    pub nm_smt: Option<String>,
    
    // Sync status
    pub status_sync: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

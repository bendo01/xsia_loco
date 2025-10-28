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
    pub id_registrasi_mahasiswa: Option<Uuid>,
    pub id_mahasiswa: Option<Uuid>,
    pub id_perguruan_tinggi: Option<Uuid>,
    pub id_prodi: Option<Uuid>,
    pub tgl_masuk_sp: Option<Date>,
    pub tgl_keluar: Option<Date>,
    pub skhun: Option<String>,
    pub no_peserta_ujian: Option<String>,
    pub no_seri_ijazah: Option<String>,
    pub tgl_create: Option<Date>,
    pub sks_diakui: Option<f32>,
    pub jalur_skripsi: Option<String>,
    pub judul_skripsi: Option<String>,
    pub bln_awal_bimbingan: Option<String>,
    pub bln_akhir_bimbingan: Option<String>,
    pub sk_yudisium: Option<String>,
    pub tgl_sk_yudisium: Option<Date>,
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
    pub tanggal_keluar: Option<Date>,
    pub id_periode_keluar: Option<String>,
    pub keterangan: Option<String>,
    pub no_sertifikat_profesi: Option<String>,
    pub tanggal_terbit_ijazah: Option<Date>,
    pub status_sync: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

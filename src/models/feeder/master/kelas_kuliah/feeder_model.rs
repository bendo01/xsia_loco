use crate::library::deserialization::{de_opt_f32, de_opt_date_dmy, de_opt_i32};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelInputDetailKelasKuliah {
    pub id_kelas_kuliah: Uuid,
    pub id_prodi: Option<Uuid>,
    pub nama_program_studi: Option<String>,
    pub id_semester: Option<String>,
    pub nama_semester: Option<String>,
    pub id_matkul: Option<Uuid>,
    pub kode_mata_kuliah: Option<String>,
    pub nama_mata_kuliah: Option<String>,
    #[serde(deserialize_with = "de_opt_f32")]
    pub sks_mk: Option<f32>,
    #[serde(deserialize_with = "de_opt_f32")]
    pub sks_tm: Option<f32>,
    #[serde(deserialize_with = "de_opt_f32")]
    pub sks_prak: Option<f32>,
    #[serde(deserialize_with = "de_opt_f32")]
    pub sks_prak_lap: Option<f32>,
    #[serde(deserialize_with = "de_opt_f32")]
    pub sks_sim: Option<f32>,
    pub nama_kelas_kuliah: Option<String>,
    pub bahasan: Option<String>,
    #[serde(deserialize_with = "de_opt_date_dmy")]
    pub tanggal_mulai_efektif: Option<NaiveDate>,
    #[serde(deserialize_with = "de_opt_date_dmy")]
    pub tanggal_akhir_efektif: Option<NaiveDate>,
    #[serde(deserialize_with = "de_opt_i32")]
    pub kapasitas: Option<i32>,
    #[serde(deserialize_with = "de_opt_date_dmy")]
    pub tanggal_tutup_daftar: Option<NaiveDate>,
    pub prodi_penyelenggara: Option<String>,
    pub perguruan_tinggi_penyelenggara: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelInputListKelasKuliah {
    pub id_kelas_kuliah: Uuid,
    pub id_prodi: Option<Uuid>,
    pub nama_program_studi: Option<String>,
    pub id_semester: Option<String>,
    pub nama_semester: Option<String>,
    pub id_matkul: Option<Uuid>,
    pub kode_mata_kuliah: Option<String>,
    pub nama_mata_kuliah: Option<String>,
    pub nama_kelas_kuliah: Option<String>,
    #[serde(deserialize_with = "de_opt_f32")]
    pub sks: Option<f32>,
    pub id_dosen: Option<String>,
    pub nama_dosen: Option<String>,
    #[serde(deserialize_with = "de_opt_i32")]
    pub jumlah_mahasiswa: Option<i32>,
    #[serde(deserialize_with = "de_opt_i32")]
    pub apa_untuk_pditt: Option<i32>,
}
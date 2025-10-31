use crate::library::deserialization::de_opt_f32;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelInputDetailPerkuliahanMahasiswa {
    pub id_registrasi_mahasiswa: Option<Uuid>,
    pub id_prodi: Option<Uuid>,
    pub nama_program_studi: Option<String>,
    pub angkatan: Option<String>,
    pub id_semester: Option<String>,
    pub nim: Option<String>,
    pub nama_mahasiswa: Option<String>,
    pub nama_semester: Option<String>,
    pub id_status_mahasiswa: Option<String>,
    pub nama_status_mahasiswa: Option<String>,
    #[serde(deserialize_with = "de_opt_f32")]
    pub ips: Option<f32>,
    #[serde(deserialize_with = "de_opt_f32")]
    pub ipk: Option<f32>,
    #[serde(deserialize_with = "de_opt_f32")]
    pub sks_semester: Option<f32>,
    #[serde(deserialize_with = "de_opt_f32")]
    pub sks_total: Option<f32>,
    pub status_sync: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelInputListPerkuliahanMahasiswa {
    pub id_registrasi_mahasiswa: Option<Uuid>,
    pub nim: Option<String>,
    pub nama_mahasiswa: Option<String>,
    pub id_prodi: Option<Uuid>,
    pub nama_program_studi: Option<String>,
    pub angkatan: Option<String>,
    pub id_periode_masuk: Option<String>,
    pub id_semester: Option<String>,
    pub nama_semester: Option<String>,
    pub id_status_mahasiswa: Option<String>,
    pub nama_status_mahasiswa: Option<String>,
    #[serde(deserialize_with = "de_opt_f32")]
    pub ips: Option<f32>,
    #[serde(deserialize_with = "de_opt_f32")]
    pub ipk: Option<f32>,
    #[serde(deserialize_with = "de_opt_f32")]
    pub sks_semester: Option<f32>,
    #[serde(deserialize_with = "de_opt_f32")]
    pub sks_total: Option<f32>,
    #[serde(deserialize_with = "de_opt_f32")]
    pub biaya_kuliah_smt: Option<f32>,
    pub id_pembiayaan: Option<String>,
    pub status_sync: Option<String>,
}

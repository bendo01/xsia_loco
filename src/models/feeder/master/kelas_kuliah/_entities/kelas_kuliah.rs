use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "feeder_master", table_name = "kelas_kuliah")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub id_kelas_kuliah: Option<Uuid>,
    pub id_prodi: Option<Uuid>,
    pub nama_program_studi: Option<String>,
    pub id_semester: Option<String>,
    pub nama_semester: Option<String>,
    pub id_matkul: Option<Uuid>,
    pub kode_mata_kuliah: Option<String>,
    pub nama_mata_kuliah: Option<String>,
    pub nama_kelas_kuliah: Option<String>,
    pub sks: Option<f32>,
    #[sea_orm(column_type = "Text", nullable)]
    pub id_dosen: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub nama_dosen: Option<String>,
    pub jumlah_mahasiswa: Option<String>,
    pub apa_untuk_pditt: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub bahasan: Option<String>,
    pub tanggal_mulai_efektif: Option<Date>,
    pub tanggal_akhir_efektif: Option<Date>,
    pub kapasitas: Option<i32>,
    pub tanggal_tutup_daftar: Option<Date>,
    pub prodi_penyelenggara: Option<String>,
    pub perguruan_tinggi_penyelenggara: Option<String>,
    pub jumlah_mahasiswa_dapat_nilai: Option<String>,
    pub sks_tm: Option<f32>,
    pub sks_prak: Option<f32>,
    pub sks_prak_lap: Option<f32>,
    pub sks_sim: Option<f32>,
    pub a_selenggara_pditt: Option<String>,
    pub a_pengguna_pditt: Option<String>,
    pub kuota_pditt: Option<String>,
    pub tgl_mulai_koas: Option<Date>,
    pub tgl_selesai_koas: Option<Date>,
    pub id_mou: Option<String>,
    pub id_kls_pditt: Option<String>,
    pub tgl_create: Option<Date>,
    pub lingkup_kelas: Option<String>,
    pub mode_kuliah: Option<String>,
    pub status_sync: Option<String>,
    pub sync_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

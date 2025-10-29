use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "feeder_master", table_name = "periode_perkuliahan")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub sync_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub id_prodi: Option<Uuid>,
    pub nama_program_studi: Option<String>,
    pub id_semester: Option<String>,
    pub nama_semester: Option<String>,
    pub jumlah_target_mahasiswa_baru: Option<String>,
    pub tanggal_awal_perkuliahan: Option<Date>,
    pub tanggal_akhir_perkuliahan: Option<Date>,
    pub jumlah_pendaftar_ikut_seleksi: Option<String>,
    pub jumlah_pendaftar_lulus_seleksi: Option<String>,
    pub jumlah_daftar_ulang: Option<String>,
    pub jumlah_mengundurkan_diri: Option<String>,
    pub jumlah_minggu_pertemuan: Option<String>,
    pub metode_kul: Option<String>,
    pub metode_kul_eks: Option<String>,
    pub tgl_create: Option<Date>,
    pub last_update: Option<Date>,
    pub status_sync: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "feeder_master", table_name = "aktifitas_mahasiswa")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub asal_data: Option<String>,
    pub nm_asaldata: Option<String>,
    pub id_aktivitas: Option<Uuid>,
    pub jenis_anggota: Option<String>,
    pub nama_jenis_anggota: Option<String>,
    pub id_jenis_aktivitas: Option<Uuid>,
    pub nama_jenis_aktivitas: Option<String>,
    pub id_prodi: Option<Uuid>,
    pub nama_prodi: Option<String>,
    pub id_semester: Option<Uuid>,
    pub nama_semester: Option<String>,
    pub judul: Option<String>,
    pub keterangan: Option<String>,
    pub lokasi: Option<String>,
    pub sk_tugas: Option<String>,
    pub tanggal_sk_tugas: Option<Date>,
    pub untuk_kampus_merdeka: Option<i32>,
    pub tanggal_mulai: Option<Date>,
    pub tanggal_selesai: Option<Date>,
    pub sync_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

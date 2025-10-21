use crate::library::deserialization::de_opt_date_dmy;
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelInput {
    pub id_komponen_evaluasi: Uuid,
    pub id_kelas_kuliah: Uuid,
    pub id_jenis_evaluasi: i32,
    pub nama: Option<String>,
    pub nama_inggris: Option<String>,
    pub nomor_urut: i32,
    pub bobot_evaluasi: String,
    #[serde(deserialize_with = "de_opt_date_dmy", default)]
    pub last_update: Option<NaiveDate>,
    #[serde(deserialize_with = "de_opt_date_dmy", default)]
    pub tgl_create: Option<NaiveDate>,
}
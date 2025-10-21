use crate::library::deserialization::{de_opt_date_dmy, de_opt_i32};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_komponen_evaluasi: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_kelas_kuliah: Option<Uuid>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "de_opt_i32"
    )]
    pub id_jenis_evaluasi: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nama: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nama_inggris: Option<String>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "de_opt_i32"
    )]
    pub nomor_urut: Option<i32>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "de_opt_i32"
    )]
    pub bobot_evaluasi: Option<i32>,
    #[serde(deserialize_with = "de_opt_date_dmy", default)]
    pub last_update: Option<NaiveDate>,
    #[serde(deserialize_with = "de_opt_date_dmy", default)]
    pub tgl_create: Option<NaiveDate>,
}

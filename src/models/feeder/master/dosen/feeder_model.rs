use crate::library::deserialization::{de_opt_date_dmy, de_opt_i32};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelInput {
    // uuid in DB, required
    pub id_dosen: Uuid,

    pub nama_dosen: String,
    pub nidn: Option<String>,
    pub nuptk: Option<String>, // Not in DB but in API response
    pub nip: Option<String>,
    pub jenis_kelamin: Option<String>,

    // integer in DB -> i32
    #[serde(deserialize_with = "de_opt_i32")]
    pub id_agama: Option<i32>,

    pub nama_agama: Option<String>,

    #[serde(deserialize_with = "de_opt_date_dmy")]
    pub tanggal_lahir: Option<NaiveDate>,

    pub id_status_aktif: Option<String>,
    pub nama_status_aktif: Option<String>,
}

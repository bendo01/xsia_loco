use crate::library::deserialization::{de_opt_date_dmy, de_opt_date_time_iso, de_opt_i32};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInputListPeriodePerkuliahan {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInputDetailPeriodePerkuliahan {}

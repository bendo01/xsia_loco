use crate::library::deserialization::{
    de_opt_date_dmy,
    de_opt_f32,
    de_opt_i32,
};
use loco_rs::prelude::Date;
use serde::{Deserialize, Serialize};
use uuid::Uuid; // Removed unused `uuid` macro import

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelInput {
}
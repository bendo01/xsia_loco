use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "feeder_akumulasi", table_name = "estimasi")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub institution_id: Uuid,
    pub total_data_per_request: i32,
    pub last_offset: i32,
    pub total_data: i32,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub sync_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

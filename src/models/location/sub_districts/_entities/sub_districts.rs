use crate::models::location::regencies::_entities::regencies as LocationRegency;
use crate::models::location::villages::_entities::villages as LocationVillage;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "location", table_name = "sub_districts")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub code: Option<String>,
    pub name: String,
    pub dikti_code: Option<String>,
    pub regency_id: Uuid,
    pub slug: Option<String>,
    pub alt_slug: Option<String>,
    pub state_ministry_code: Option<String>,
    pub state_ministry_full_code: Option<String>,
    pub state_post_department_code: Option<String>,
    pub state_ministry_name: Option<String>,
    pub dikti_name: Option<String>,
    pub validation_code: Option<String>,
    pub agriculture_department_name: Option<String>,
    #[sea_orm(column_type = "Double", nullable)]
    pub latitude: Option<f64>,
    #[sea_orm(column_type = "Double", nullable)]
    pub longitude: Option<f64>,
    pub zoom: Option<i32>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Regency,
    Villages,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Regency => Entity::belongs_to(LocationRegency::Entity)
                .from(Column::RegencyId)
                .to(LocationRegency::Column::Id)
                .into(),
            Self::Villages => Entity::has_many(LocationVillage::Entity)
                .from(Column::Id)
                .to(LocationVillage::Column::SubDistrictId)
                .into(),
        }
    }
}

// `Related` trait has to be implemented by hand
impl Related<LocationRegency::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Regency.def()
    }
}

// `Related` trait has to be implemented by hand
impl Related<LocationVillage::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Villages.def()
    }
}

use crate::models::location::countries::_entities::countries as LocationCountry;
use crate::models::location::regencies::_entities::regencies as LocationRegency;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "location", table_name = "provinces")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub code: Option<String>,
    pub name: Option<String>,
    pub dikti_code: Option<String>,
    pub epsbed_code: Option<String>,
    pub slug: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
    pub alt_slug: Option<String>,
    pub state_ministry_code: Option<String>,
    pub state_ministry_full_code: Option<String>,
    pub state_post_department_code: Option<String>,
    pub state_ministry_name: Option<String>,
    pub dikti_name: Option<String>,
    pub validation_code: Option<String>,
    #[sea_orm(column_type = "Double", nullable)]
    pub latitude: Option<f64>,
    #[sea_orm(column_type = "Double", nullable)]
    pub longitude: Option<f64>,
    pub zoom: Option<i32>,
    pub country_id: Option<Uuid>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Country,
    Regencies,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Country => Entity::belongs_to(LocationCountry::Entity)
                .from(Column::CountryId)
                .to(LocationCountry::Column::Id)
                .into(),
            Self::Regencies => Entity::has_many(LocationRegency::Entity).into(),
        }
    }
}

// `Related` trait has to be implemented by hand
impl Related<LocationCountry::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Country.def()
    }
}

// `Related` trait has to be implemented by hand
impl Related<LocationRegency::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Regencies.def()
    }
}

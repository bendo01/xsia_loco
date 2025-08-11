use crate::models::location::continents::_entities::continents as LocationContinent;
use crate::models::location::provinces::_entities::provinces as LocationProvince;
use crate::models::location::regions::_entities::regions as LocationRegion;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "location", table_name = "countries")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub alpha2_code: String,
    pub alpha3_code: String,
    pub iso3166_2_code: String,
    pub dikti_code: Option<String>,
    pub continent_id: Option<Uuid>,
    pub region_id: Option<Uuid>,
    pub slug: Option<String>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Continent,
    Region,
    Provinces,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Continent => Entity::belongs_to(LocationContinent::Entity)
                .from(Column::ContinentId)
                .to(LocationContinent::Column::Id)
                .into(),
            Self::Region => Entity::belongs_to(LocationRegion::Entity)
                .from(Column::ContinentId)
                .to(LocationRegion::Column::Id)
                .into(),
            Self::Provinces => Entity::has_many(LocationProvince::Entity).into(),
        }
    }
}

// `Related` trait has to be implemented by hand
impl Related<LocationContinent::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Continent.def()
    }
}

// `Related` trait has to be implemented by hand
impl Related<LocationRegion::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Region.def()
    }
}

// `Related` trait has to be implemented by hand
impl Related<LocationProvince::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Provinces.def()
    }
}

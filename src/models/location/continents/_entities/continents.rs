use crate::models::location::countries::_entities::countries as LocationCountry;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "location", table_name = "continents")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub code: Option<i32>,
    pub alphabet_code: String,
    pub name: String,
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
    Countries,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Countries => Entity::has_many(LocationCountry::Entity)
                .from(Column::Id)
                .to(LocationCountry::Column::ContinentId)
                .into(),
        }
    }
}

impl Related<LocationCountry::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Countries.def()
    }
}

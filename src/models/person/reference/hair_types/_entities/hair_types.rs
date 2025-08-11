use crate::models::person::master::biodatas::_entities::biodatas as PersonMasterBiodata;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "person_reference", table_name = "hair_types")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub code: i32,
    pub alphabet_code: String,
    pub name: String,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Biodatas,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Biodatas => Entity::has_many(PersonMasterBiodata::Entity)
                .from(Column::Id)
                .to(PersonMasterBiodata::Column::HairTypeId)
                .into(),
        }
    }
}

// `Related` trait has to be implemented by hand
impl Related<PersonMasterBiodata::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Biodatas.def()
    }
}

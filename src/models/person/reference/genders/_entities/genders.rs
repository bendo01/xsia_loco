use crate::models::person::master::individuals::_entities::individuals as PersonMasterIndividual;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, Validate)]
#[sea_orm(schema_name = "person_reference", table_name = "genders")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[validate(range(min = 1))]
    pub code: i32,
    #[validate(length(min = 1, message = "Name must be at least 1 characters long."))]
    pub alphabet_code: String,
    #[validate(length(min = 1, message = "Name must be at least 1 characters long."))]
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
    Individuals,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Individuals => Entity::has_many(PersonMasterIndividual::Entity)
                .from(Column::Id)
                .to(PersonMasterIndividual::Column::GenderId)
                .into(),
        }
    }
}

// `Related` trait has to be implemented by hand
impl Related<PersonMasterIndividual::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Individuals.def()
    }
}

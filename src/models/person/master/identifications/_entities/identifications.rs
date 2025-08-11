use crate::models::person::master::individuals::_entities::individuals as PersonMasterIndividual;
use crate::models::person::reference::identification_types::_entities::identification_types as PersonReferenceIdentificationType;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "person_master", table_name = "identifications")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub individual_id: Uuid,
    pub identification_type_id: Uuid,
    pub code: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

// #[derive(Copy, Clone, Debug, EnumIter)]

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Individual,
    IdentificationType,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Individual => Entity::belongs_to(PersonMasterIndividual::Entity)
                .from(Column::IndividualId)
                .to(PersonMasterIndividual::Column::Id)
                .into(),
            Self::IdentificationType => {
                Entity::belongs_to(PersonReferenceIdentificationType::Entity)
                    .from(Column::IdentificationTypeId)
                    .to(PersonReferenceIdentificationType::Column::Id)
                    .into()
            }
        }
    }
}

// `Related` trait has to be implemented by hand
impl Related<PersonMasterIndividual::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Individual.def()
    }
}

// `Related` trait has to be implemented by hand
impl Related<PersonReferenceIdentificationType::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::IdentificationType.def()
    }
}

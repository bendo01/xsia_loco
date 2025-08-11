use crate::models::person::master::family_cards::_entities::family_cards as PersonMasterFamilyCard;
use crate::models::person::master::individuals::_entities::individuals as PersonMasterIndividual;
use crate::models::person::master::relatives::_entities::relatives as PersonMasterRelative;
use crate::models::person::reference::relative_types::_entities::relative_types as PersonReferenceRelativeType;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "person_master", table_name = "family_card_members")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub family_card_id: Uuid,
    pub individual_id: Uuid,
    pub relative_id: Uuid,
    pub relative_type_id: Uuid,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    FamilyCard,
    Individual,
    Relative,
    RelativeType,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::FamilyCard => Entity::belongs_to(PersonMasterFamilyCard::Entity)
                .from(Column::FamilyCardId)
                .to(PersonMasterFamilyCard::Column::Id)
                .into(),
            Self::Individual => Entity::belongs_to(PersonMasterIndividual::Entity)
                .from(Column::IndividualId)
                .to(PersonMasterIndividual::Column::Id)
                .into(),
            Self::Relative => Entity::belongs_to(PersonMasterRelative::Entity)
                .from(Column::RelativeId)
                .to(PersonMasterRelative::Column::Id)
                .into(),
            Self::RelativeType => Entity::belongs_to(PersonReferenceRelativeType::Entity)
                .from(Column::RelativeTypeId)
                .to(PersonReferenceRelativeType::Column::Id)
                .into(),
        }
    }
}

// `Related` trait has to be implemented by hand
impl Related<PersonMasterFamilyCard::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FamilyCard.def()
    }
}

// `Related` trait has to be implemented by hand
impl Related<PersonMasterIndividual::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Individual.def()
    }
}

// `Related` trait has to be implemented by hand
impl Related<PersonMasterRelative::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Relative.def()
    }
}

// `Related` trait has to be implemented by hand
impl Related<PersonReferenceRelativeType::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RelativeType.def()
    }
}

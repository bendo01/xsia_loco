use crate::models::person::master::biodatas::_entities::biodatas as PersonMasterBiodata;
use crate::models::person::master::family_card_members::_entities::family_card_members as PersonMasterFamilyCardMember;
use crate::models::person::reference::genders::_entities::genders as PersonReferenceGender;
use crate::models::person::reference::identification_types::_entities::identification_types as PersonReferenceIdentificationType;
use crate::models::person::reference::incomes::_entities::incomes as PersonReferenceIncome;
use crate::models::person::reference::marital_statuses::_entities::marital_statuses as PersonReferenceMaritalStatus;
use crate::models::person::reference::occupations::_entities::occupations as PersonReferenceOccupation;
use crate::models::person::reference::professions::_entities::professions as PersonReferenceProfession;
use crate::models::person::reference::religions::_entities::religions as PersonReferenceReligion;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "person_master", table_name = "individuals")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub birth_date: Date,
    pub birth_place: String,
    pub gender_id: Uuid,
    pub religion_id: Uuid,
    pub occupation_id: Option<Uuid>,
    pub education_id: Option<Uuid>,
    pub income_id: Option<Uuid>,
    pub identification_type_id: Uuid,
    pub marital_status_id: Uuid,
    pub profession_id: Uuid,
    pub is_special_need: bool,
    pub is_social_protection_card_recipient: bool,
    pub is_deceased: Option<bool>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub front_title: Option<String>,
    pub last_title: Option<String>,
    pub age_classification_id: Option<Uuid>,
}

// #[derive(Copy, Clone, Debug, EnumIter)]

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Gender,
    Religion,
    Occupation,
    Income,
    IdentificationType,
    MaritalStatus,
    Profession,
    Biodata,
    FamilyCardMembers,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Gender => Entity::belongs_to(PersonReferenceGender::Entity)
                .from(Column::GenderId)
                .to(PersonReferenceGender::Column::Id)
                .into(),
            Self::Religion => Entity::belongs_to(PersonReferenceReligion::Entity)
                .from(Column::ReligionId)
                .to(PersonReferenceReligion::Column::Id)
                .into(),
            Self::Occupation => Entity::belongs_to(PersonReferenceOccupation::Entity)
                .from(Column::OccupationId)
                .to(PersonReferenceOccupation::Column::Id)
                .into(),
            Self::Income => Entity::belongs_to(PersonReferenceIncome::Entity)
                .from(Column::IncomeId)
                .to(PersonReferenceIncome::Column::Id)
                .into(),
            Self::IdentificationType => {
                Entity::belongs_to(PersonReferenceIdentificationType::Entity)
                    .from(Column::IdentificationTypeId)
                    .to(PersonReferenceIdentificationType::Column::Id)
                    .into()
            }
            Self::Profession => Entity::belongs_to(PersonReferenceProfession::Entity)
                .from(Column::ProfessionId)
                .to(PersonReferenceProfession::Column::Id)
                .into(),
            Self::MaritalStatus => Entity::belongs_to(PersonReferenceMaritalStatus::Entity)
                .from(Column::MaritalStatusId)
                .to(PersonReferenceMaritalStatus::Column::Id)
                .into(),
            Self::Biodata => Entity::has_one(PersonMasterBiodata::Entity)
                .from(Column::Id)
                .to(PersonMasterBiodata::Column::IndividualId)
                .into(),
            Self::FamilyCardMembers => Entity::has_many(PersonMasterFamilyCardMember::Entity)
                .from(Column::Id)
                .to(PersonMasterFamilyCardMember::Column::IndividualId)
                .into(),
        }
    }
}

// `Related` trait has to be implemented by hand
impl Related<PersonReferenceGender::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Gender.def()
    }
}

// `Related` trait has to be implemented by hand
impl Related<PersonReferenceReligion::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Religion.def()
    }
}

// `Related` trait has to be implemented by hand
impl Related<PersonReferenceOccupation::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Occupation.def()
    }
}

// `Related` trait has to be implemented by hand
impl Related<PersonReferenceIncome::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Income.def()
    }
}

// `Related` trait has to be implemented by hand
impl Related<PersonReferenceIdentificationType::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::IdentificationType.def()
    }
}

// `Related` trait has to be implemented by hand
impl Related<PersonReferenceMaritalStatus::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MaritalStatus.def()
    }
}

// `Related` trait has to be implemented by hand
impl Related<PersonReferenceProfession::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Profession.def()
    }
}

impl Related<PersonMasterBiodata::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Biodata.def()
    }
}

// `Related` trait has to be implemented by hand
impl Related<PersonMasterFamilyCardMember::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FamilyCardMembers.def()
    }
}

use crate::models::person::master::individuals::_entities::individuals as PersonMasterIndividual;
use crate::models::person::master::relatives::_entities::relatives as PersonMasterRelative;
use crate::models::person::reference::blood_types::_entities::blood_types as PersonReferenceBloodType;
use crate::models::person::reference::eye_colors::_entities::eye_colors as PersonReferenceEyeColor;
use crate::models::person::reference::hair_colors::_entities::hair_colors as PersonReferenceHairColor;
use crate::models::person::reference::hair_types::_entities::hair_types as PersonReferenceHairType;

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "person_master", table_name = "biodatas")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(column_type = "Double", nullable)]
    pub height: Option<f64>,
    #[sea_orm(column_type = "Double", nullable)]
    pub weight: Option<f64>,
    pub is_positive_blood_rhesus: bool,
    pub blood_type_id: Uuid,
    pub hair_type_id: Uuid,
    pub hair_color_id: Uuid,
    pub eye_color_id: Uuid,
    pub individual_id: Uuid,
    #[sea_orm(column_type = "Double")]
    pub bust: f64,
    #[sea_orm(column_type = "Double")]
    pub waist: f64,
    #[sea_orm(column_type = "Double")]
    pub hip: f64,
    #[sea_orm(column_type = "Double")]
    pub arm_circumference: f64,
    pub menarche_age: i32,
    pub menopause_age: i32,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    PersonReferenceBloodType,
    PersonReferenceEyeColor,
    PersonReferenceHairColor,
    PersonReferenceHairType,
    PersonMasterIndividual,
    PersonMasterRelative,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::PersonReferenceBloodType => Entity::belongs_to(PersonReferenceBloodType::Entity)
                .from(Column::BloodTypeId)
                .to(PersonReferenceBloodType::Column::Id)
                .into(),
            Self::PersonReferenceEyeColor => Entity::belongs_to(PersonReferenceEyeColor::Entity)
                .from(Column::EyeColorId)
                .to(PersonReferenceEyeColor::Column::Id)
                .into(),
            Self::PersonReferenceHairColor => Entity::belongs_to(PersonReferenceHairColor::Entity)
                .from(Column::HairColorId)
                .to(PersonReferenceHairColor::Column::Id)
                .into(),
            Self::PersonReferenceHairType => Entity::belongs_to(PersonReferenceHairType::Entity)
                .from(Column::HairTypeId)
                .to(PersonReferenceHairType::Column::Id)
                .into(),
            Self::PersonMasterIndividual => Entity::belongs_to(PersonMasterIndividual::Entity)
                .from(Column::IndividualId)
                .to(PersonMasterIndividual::Column::Id)
                .into(),
            Self::PersonMasterRelative => Entity::belongs_to(PersonMasterRelative::Entity)
                .from(Column::IndividualId)
                .to(PersonMasterRelative::Column::Id)
                .into(),
        }
    }
}

// `Related` trait has to be implemented by hand
impl Related<PersonReferenceBloodType::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PersonReferenceBloodType.def()
    }
}

// `Related` trait has to be implemented by hand
impl Related<PersonReferenceEyeColor::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PersonReferenceEyeColor.def()
    }
}

// `Related` trait has to be implemented by hand
impl Related<PersonReferenceHairColor::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PersonReferenceHairColor.def()
    }
}

// `Related` trait has to be implemented by hand
impl Related<PersonReferenceHairType::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PersonReferenceHairType.def()
    }
}

// `Related` trait has to be implemented by hand
impl Related<PersonMasterIndividual::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PersonMasterIndividual.def()
    }
}

// `Related` trait has to be implemented by hand
impl Related<PersonMasterRelative::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PersonMasterRelative.def()
    }
}

use crate::models::academic::candidate::reference::registration_types::_entities::registration_types as AcademicCandidateReferenceRegistrationType;
use crate::models::academic::prior_learning_recognition::transaction::recognitions::_entities::recognitions as AcademicPriorLearningRecognitionTransactionRecognition;
use crate::models::academic::survey::master::bundles::_entities::bundles as AcademicSurveyMasterBundle;
use crate::models::institution::master::institutions::_entities::institutions as InstitutionMasterInstitution;
use crate::models::institution::master::staffes::_entities::staffes as InstitutionMasterStaff;
use crate::models::institution::reference::unit_types::_entities::unit_types as InstitutionReferenceUnitType;
use crate::models::literate::educations::_entities::educations as LiterateEducation;

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "institution_master", table_name = "units")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub is_active: bool,
    pub unit_type_id: Uuid,
    pub institution_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub education_id: Option<Uuid>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub feeder_id: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    UnitType,
    Institution,
    Education,
    Staffes,
    Bundles,
    RegistrationTypes,
    Recognitions,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::UnitType => Entity::belongs_to(InstitutionReferenceUnitType::Entity)
                .from(Column::UnitTypeId)
                .to(InstitutionReferenceUnitType::Column::Id)
                .into(),
            Self::Institution => Entity::belongs_to(InstitutionMasterInstitution::Entity)
                .from(Column::InstitutionId)
                .to(InstitutionMasterInstitution::Column::Id)
                .into(),
            Self::Education => Entity::belongs_to(LiterateEducation::Entity)
                .from(Column::EducationId)
                .to(LiterateEducation::Column::GroupId)
                .into(),
            Self::Staffes => Entity::has_many(InstitutionMasterStaff::Entity)
                .from(Column::Id)
                .to(InstitutionMasterStaff::Column::UnitId)
                .into(),
            Self::Bundles => Entity::has_many(AcademicSurveyMasterBundle::Entity)
                .from(Column::Id)
                .to(AcademicSurveyMasterBundle::Column::UnitId)
                .into(),
            Self::RegistrationTypes => {
                Entity::has_many(AcademicCandidateReferenceRegistrationType::Entity)
                    .from(Column::Id)
                    .to(AcademicCandidateReferenceRegistrationType::Column::UnitId)
                    .into()
            }
            Self::Recognitions => {
                Entity::has_many(AcademicPriorLearningRecognitionTransactionRecognition::Entity)
                    .from(Column::Id)
                    .to(AcademicPriorLearningRecognitionTransactionRecognition::Column::UnitId)
                    .into()
            }
        }
    }
}

impl Related<InstitutionReferenceUnitType::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UnitType.def()
    }
}

impl Related<InstitutionMasterInstitution::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Institution.def()
    }
}

impl Related<LiterateEducation::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Education.def()
    }
}

impl Related<InstitutionMasterStaff::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Staffes.def()
    }
}

impl Related<AcademicSurveyMasterBundle::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Bundles.def()
    }
}

impl Related<AcademicCandidateReferenceRegistrationType::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RegistrationTypes.def()
    }
}

impl Related<AcademicPriorLearningRecognitionTransactionRecognition::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Recognitions.def()
    }
}

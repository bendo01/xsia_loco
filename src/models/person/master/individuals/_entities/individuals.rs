use crate::models::academic::candidate::master::candidates::_entities::candidates as AcademicCandidateMasterCandidate;
use crate::models::academic::lecturer::master::lecturers::_entities::lecturers as AcademicLecturerMasterLecturer;
use crate::models::academic::student::master::students::_entities::students as AcademicStudentMasterStudent;
use crate::models::institution::master::employees::_entities::employees as InstitutionMasterEmployee;
use crate::models::person::master::biodatas::_entities::biodatas as PersonMasterBiodata;
use crate::models::person::master::family_card_members::_entities::family_card_members as PersonMasterFamilyCardMember;
use crate::models::person::master::images::_entities::images as PersonMasterImage;
use crate::models::person::reference::age_classifications::_entities::age_classifications as PersonReferenceAgeClassification;
use crate::models::person::reference::genders::_entities::genders as PersonReferenceGender;
use crate::models::person::reference::identification_types::_entities::identification_types as PersonReferenceIdentificationType;
use crate::models::person::reference::incomes::_entities::incomes as PersonReferenceIncome;
use crate::models::person::reference::marital_statuses::_entities::marital_statuses as PersonReferenceMaritalStatus;
use crate::models::person::reference::occupations::_entities::occupations as PersonReferenceOccupation;
use crate::models::person::reference::professions::_entities::professions as PersonReferenceProfession;
use crate::models::person::reference::religions::_entities::religions as PersonReferenceReligion;
use crate::models::academic::prior_learning_recognition::transaction::evaluators::_entities::evaluators as AcademicPriorLearningRecognitionTransactionEvaluators;

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
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
    #[sea_orm(nullable)]
    pub occupation_id: Option<Uuid>,
    #[sea_orm(nullable)]
    pub education_id: Option<Uuid>,
    #[sea_orm(nullable)]
    pub income_id: Option<Uuid>,
    pub identification_type_id: Uuid,
    pub marital_status_id: Uuid,
    pub profession_id: Uuid,
    pub is_special_need: bool,
    pub is_social_protection_card_recipient: bool,
    pub is_deceased: bool,
    #[sea_orm(nullable)]
    pub front_title: Option<String>,
    #[sea_orm(nullable)]
    pub last_title: Option<String>,
    #[sea_orm(nullable)]
    pub age_classification_id: Option<Uuid>,
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
    AgeClassification,
    Gender,
    Religion,
    Occupation,
    Income,
    IdentificationType,
    MaritalStatus,
    Profession,
    Biodata,
    FamilyCardMembers,
    Employees,
    Lecturer,
    Candidate,
    Students,
    Picture,
    Evaluators,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::AgeClassification => Entity::belongs_to(PersonReferenceAgeClassification::Entity)
                .from(Column::AgeClassificationId)
                .to(PersonReferenceAgeClassification::Column::Id)
                .into(),
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
            Self::MaritalStatus => Entity::belongs_to(PersonReferenceMaritalStatus::Entity)
                .from(Column::MaritalStatusId)
                .to(PersonReferenceMaritalStatus::Column::Id)
                .into(),

            Self::Profession => Entity::belongs_to(PersonReferenceProfession::Entity)
                .from(Column::ProfessionId)
                .to(PersonReferenceProfession::Column::Id)
                .into(),

            Self::Biodata => Entity::has_one(PersonMasterBiodata::Entity)
                .from(Column::Id)
                .to(PersonMasterBiodata::Column::IndividualId)
                .into(),
            Self::Picture => Entity::has_one(PersonMasterImage::Entity)
                .from(Column::Id)
                .to(PersonMasterImage::Column::IndividualId)
                .into(),
            Self::Lecturer => Entity::has_one(AcademicLecturerMasterLecturer::Entity)
                .from(Column::Id)
                .to(AcademicLecturerMasterLecturer::Column::IndividualId)
                .into(),
            Self::Candidate => Entity::has_one(AcademicCandidateMasterCandidate::Entity)
                .from(Column::Id)
                .to(AcademicCandidateMasterCandidate::Column::IndividualId)
                .into(),
            Self::FamilyCardMembers => Entity::has_many(PersonMasterFamilyCardMember::Entity)
                .from(Column::Id)
                .to(PersonMasterFamilyCardMember::Column::IndividualId)
                .into(),
            Self::Employees => Entity::has_many(InstitutionMasterEmployee::Entity)
                .from(Column::Id)
                .to(InstitutionMasterEmployee::Column::IndividualId)
                .into(),
            Self::Students => Entity::has_many(AcademicStudentMasterStudent::Entity)
                .from(Column::Id)
                .to(AcademicStudentMasterStudent::Column::IndividualId)
                .into(),
            Self::Evaluators => Entity::has_many(AcademicPriorLearningRecognitionTransactionEvaluators::Entity)
                .from(Column::Id)
                .to(AcademicPriorLearningRecognitionTransactionEvaluators::Column::IndividualId)
                .into(),
        }
    }
}

// `Related` trait has to be implemented by hand
impl Related<PersonReferenceAgeClassification::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AgeClassification.def()
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

// `Related` trait has to be implemented by hand
impl Related<InstitutionMasterEmployee::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Employees.def()
    }
}

// `Related` trait has to be implemented by hand
impl Related<AcademicLecturerMasterLecturer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Lecturer.def()
    }
}

// `Related` trait has to be implemented by hand
impl Related<PersonMasterImage::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Picture.def()
    }
}

// `Related` trait has to be implemented by hand
impl Related<AcademicStudentMasterStudent::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Students.def()
    }
}

impl Related<AcademicPriorLearningRecognitionTransactionEvaluators::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Evaluators.def()
    }
}

use crate::models::academic::general::reference::academic_years::_entities::academic_years as AcademicGeneralReferenceAcademicYear;
use crate::models::academic::student::master::students::_entities::students as AcademicStudentMasterStudent;
// use crate::models::academic::student::reference::registrations::_entities::registrations as AcademicStudentReferenceRegistration;
use crate::models::academic::candidate::reference::registration_types::_entities::registration_types as AcademicCandidateReferenceRegistrationType;
use crate::models::academic::prior_learning_recognition::transaction::recognitions::_entities::recognitions as AcademicPriorLearningRecognitionTransactionRecognition;
use crate::models::auth::users::_entities::users as AuthUser;
use crate::models::location::regencies::_entities::regencies as LocationSchoolRegency;
use crate::models::person::master::individuals::_entities::individuals as PersonMasterIndividual;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "academic_candidate_master", table_name = "candidates")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub thread: i32,
    pub code: String,
    pub name: String,
    pub student_national_number: Option<String>,
    pub school_name: Option<String>,
    pub school_regency_id: Uuid,
    pub state_smart_card_number: Option<String>,
    pub individual_id: Uuid,
    pub student_id: Option<Uuid>,
    pub academic_year_id: Uuid,
    pub institution_id: Uuid,
    pub user_id: Uuid,
    pub registration_type_id: Uuid,
    pub guidence_name: Option<String>,
    pub guidence_phone_number: Option<String>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    AcademicYear,
    Regency,
    Individual,
    Student,
    RegistrationType,
    User,
    Recognition,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::AcademicYear => Entity::belongs_to(AcademicGeneralReferenceAcademicYear::Entity)
                .from(Column::AcademicYearId)
                .to(AcademicGeneralReferenceAcademicYear::Column::Id)
                .into(),
            Self::Regency => Entity::belongs_to(LocationSchoolRegency::Entity)
                .from(Column::SchoolRegencyId)
                .to(LocationSchoolRegency::Column::Id)
                .into(),
            Self::Individual => Entity::belongs_to(PersonMasterIndividual::Entity)
                .from(Column::IndividualId)
                .to(PersonMasterIndividual::Column::Id)
                .into(),
            Self::Student => Entity::belongs_to(AcademicStudentMasterStudent::Entity)
                .from(Column::StudentId)
                .to(AcademicStudentMasterStudent::Column::Id)
                .into(),
            Self::RegistrationType => {
                Entity::belongs_to(AcademicCandidateReferenceRegistrationType::Entity)
                    .from(Column::RegistrationTypeId)
                    .to(AcademicCandidateReferenceRegistrationType::Column::Id)
                    .into()
            }
            Self::User => Entity::belongs_to(AuthUser::Entity)
                .from(Column::UserId)
                .to(AuthUser::Column::Id)
                .into(),
            Self::Recognition => Entity::has_one(AcademicPriorLearningRecognitionTransactionRecognition::Entity)
                .from(Column::Id)
                .to(AcademicPriorLearningRecognitionTransactionRecognition::Column::CandidateId)
                .into(),
        }
    }
}

impl Related<AcademicGeneralReferenceAcademicYear::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AcademicYear.def()
    }
}

impl Related<LocationSchoolRegency::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Regency.def()
    }
}

impl Related<PersonMasterIndividual::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Individual.def()
    }
}

impl Related<AcademicStudentMasterStudent::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Student.def()
    }
}

impl Related<AcademicCandidateReferenceRegistrationType::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RegistrationType.def()
    }
}

impl Related<AcademicPriorLearningRecognitionTransactionRecognition::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Recognition.def()
    }
}

impl Related<AuthUser::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

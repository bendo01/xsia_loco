use crate::models::academic::candidate::master::candidates::_entities::candidates as AcademicCandidateMasterCandidate;
use crate::models::academic::candidate::reference::phases::_entities::phases as AcademicCandidateReferencePhase;
use crate::models::academic::candidate::reference::registration_categories::_entities::registration_categories as AcademicCandidateReferenceRegistrationCategory;
use crate::models::academic::student::reference::registrations::_entities::registrations as AcademicStudentReferenceRegistration;
use crate::models::institution::master::units::_entities::units as InstitutionMasterUnit;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Deserialize, Serialize)]
#[sea_orm(
    schema_name = "academic_candidate_transaction",
    table_name = "candidate_unit_choices"
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub candidate_id: Uuid,
    pub unit_id: Uuid,
    pub student_registration_id: Uuid,
    pub registration_category_id: Uuid,
    pub phase_id: Uuid,
    pub priority: i32,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Candidate,
    Phase,
    Unit,
    StudentRegistration,
    RegistrationCategory,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Candidate => Entity::belongs_to(AcademicCandidateMasterCandidate::Entity)
                .from(Column::CandidateId)
                .to(AcademicCandidateMasterCandidate::Column::Id)
                .into(),
            Self::Phase => Entity::belongs_to(AcademicCandidateReferencePhase::Entity)
                .from(Column::PhaseId)
                .to(AcademicCandidateReferencePhase::Column::Id)
                .into(),
            Self::Unit => Entity::belongs_to(InstitutionMasterUnit::Entity)
                .from(Column::UnitId)
                .to(InstitutionMasterUnit::Column::Id)
                .into(),
            Self::StudentRegistration => {
                Entity::belongs_to(AcademicStudentReferenceRegistration::Entity)
                    .from(Column::RegistrationCategoryId)
                    .to(AcademicStudentReferenceRegistration::Column::Id)
                    .into()
            }
            Self::RegistrationCategory => {
                Entity::belongs_to(AcademicCandidateReferenceRegistrationCategory::Entity)
                    .from(Column::RegistrationCategoryId)
                    .to(AcademicCandidateReferenceRegistrationCategory::Column::Id)
                    .into()
            }
        }
    }
}

impl Related<AcademicCandidateMasterCandidate::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Candidate.def()
    }
}

impl Related<AcademicCandidateReferencePhase::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Phase.def()
    }
}

impl Related<InstitutionMasterUnit::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Unit.def()
    }
}

impl Related<AcademicStudentReferenceRegistration::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::StudentRegistration.def()
    }
}

impl Related<AcademicCandidateReferenceRegistrationCategory::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RegistrationCategory.def()
    }
}

use crate::models::academic::candidate::master::candidates::_entities::candidates as AcademicCandidateMasterCandidate;
use crate::models::academic::course::master::curriculums::_entities::curriculums as AcademicCourseMasterCurriculum;
use crate::models::academic::prior_learning_recognition::transaction::evaluators::_entities::evaluators as AcademicPriorLearningRecognitionTransactionEvaluators;
use crate::models::institution::master::units::_entities::units as InstitutionMasterUnit;

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(
    schema_name = "academic_prior_learning_recognition_transaction",
    table_name = "recognitions"
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub candidate_id: Uuid,
    pub curriculum_id: Uuid,
    pub unit_id: Uuid,
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
    Curriculum,
    Evaluators,
    Unit,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Candidate => Entity::belongs_to(AcademicCandidateMasterCandidate::Entity)
                .from(Column::CandidateId)
                .to(AcademicCandidateMasterCandidate::Column::Id)
                .into(),
            Self::Curriculum => Entity::belongs_to(AcademicCourseMasterCurriculum::Entity)
                .from(Column::CurriculumId)
                .to(AcademicCourseMasterCurriculum::Column::Id)
                .into(),
            Self::Unit => Entity::belongs_to(InstitutionMasterUnit::Entity)
                .from(Column::UnitId)
                .to(InstitutionMasterUnit::Column::Id)
                .into(),
            Self::Evaluators => Entity::has_many(
                AcademicPriorLearningRecognitionTransactionEvaluators::Entity,
            )
            .from(Column::Id)
            .to(AcademicPriorLearningRecognitionTransactionEvaluators::Column::RecognitionId)
            .into(),
        }
    }
}

impl Related<AcademicCandidateMasterCandidate::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Candidate.def()
    }
}

impl Related<AcademicCourseMasterCurriculum::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Curriculum.def()
    }
}

impl Related<InstitutionMasterUnit::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Unit.def()
    }
}

impl Related<AcademicPriorLearningRecognitionTransactionEvaluators::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Evaluators.def()
    }
}

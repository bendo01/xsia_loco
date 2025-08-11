use crate::models::academic::candidate::master::candidates::_entities::candidates as AcademicCandidateMasterCandidate;
use crate::models::academic::candidate::master::exam_classes::_entities::exam_classes as AcademicCandidateMasterExamClass;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "academic_candidate_transaction", table_name = "exams")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub candidate_id: Uuid,
    pub exam_class_id: Uuid,
    #[sea_orm(column_type = "Double")]
    pub score: f64,
    pub is_present: Option<bool>,
    pub is_pass: bool,
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
    ExamPhase,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Candidate => Entity::belongs_to(AcademicCandidateMasterCandidate::Entity)
                .from(Column::CandidateId)
                .to(AcademicCandidateMasterCandidate::Column::Id)
                .into(),
            Self::ExamPhase => Entity::belongs_to(AcademicCandidateMasterExamClass::Entity)
                .from(Column::ExamClassId)
                .to(AcademicCandidateMasterExamClass::Column::Id)
                .into(),
        }
    }
}

impl Related<AcademicCandidateMasterCandidate::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Candidate.def()
    }
}

impl Related<AcademicCandidateMasterExamClass::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ExamPhase.def()
    }
}

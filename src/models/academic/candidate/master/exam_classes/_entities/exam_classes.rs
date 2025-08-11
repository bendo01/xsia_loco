use crate::models::academic::candidate::reference::phases::_entities::phases as AcademicCandidateReferencePhase;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "academic_candidate_master", table_name = "exam_classes")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub code: Option<i32>,
    pub alphabet_code: Option<String>,
    pub name: String,
    pub phase_id: Option<Uuid>,
    pub start_date: DateTime,
    pub end_date: Option<DateTime>,
    pub start_time: Option<Time>,
    pub end_time: Option<Time>,
    pub capacity: i32,
    pub lms_category: Option<i32>,
    pub is_online: Option<bool>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Phase,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Phase => Entity::belongs_to(AcademicCandidateReferencePhase::Entity)
                .from(Column::PhaseId)
                .to(AcademicCandidateReferencePhase::Column::Id)
                .into(),
        }
    }
}

impl Related<AcademicCandidateReferencePhase::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Phase.def()
    }
}

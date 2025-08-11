use crate::models::academic::candidate::master::candidates::_entities::candidates as AcademicCandidateMasterCandidate;
use crate::models::academic::candidate::reference::document_types::_entities::document_types as AcademicCandidateReferenceDocumentType;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(
    schema_name = "academic_candidate_transaction",
    table_name = "documents"
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub candidate_id: Uuid,
    pub document_type_id: Uuid,
    pub filename: Option<String>,
    pub dir: Option<String>,
    pub r#type: Option<String>,
    pub size: Option<i32>,
    pub is_verified: Option<bool>,
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
    DocumentType,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Candidate => Entity::belongs_to(AcademicCandidateMasterCandidate::Entity)
                .from(Column::CandidateId)
                .to(AcademicCandidateMasterCandidate::Column::Id)
                .into(),
            Self::DocumentType => {
                Entity::belongs_to(AcademicCandidateReferenceDocumentType::Entity)
                    .from(Column::DocumentTypeId)
                    .to(AcademicCandidateReferenceDocumentType::Column::Id)
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

impl Related<AcademicCandidateReferenceDocumentType::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DocumentType.def()
    }
}

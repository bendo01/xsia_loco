use crate::models::academic::prior_learning_recognition::reference::evidence_types::_entities::evidence_types as AcademicPriorLearningRecognitionReferenceEvidenceType;
use crate::models::academic::prior_learning_recognition::transaction::evaluations::_entities::evaluations as AcademicPriorLearningRecognitionTransactionEvaluation;
use crate::models::document::transaction::archives::_entities::archives as DocumentTransactionArchive;

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(
    schema_name = "academic_prior_learning_recognition_transaction",
    table_name = "evaluation_details"
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub evaluation_id: Uuid,
    pub archive_id: Uuid,
    pub evidence_type_id: Uuid,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Evaluation,
    EvidenceType,
    Archive,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Evaluation => {
                Entity::belongs_to(AcademicPriorLearningRecognitionTransactionEvaluation::Entity)
                    .from(Column::EvaluationId)
                    .to(AcademicPriorLearningRecognitionTransactionEvaluation::Column::Id)
                    .into()
            }
            Self::EvidenceType => {
                Entity::belongs_to(AcademicPriorLearningRecognitionReferenceEvidenceType::Entity)
                    .from(Column::EvidenceTypeId)
                    .to(AcademicPriorLearningRecognitionReferenceEvidenceType::Column::Id)
                    .into()
            }
            Self::Archive => Entity::belongs_to(DocumentTransactionArchive::Entity)
                .from(Column::ArchiveId)
                .to(DocumentTransactionArchive::Column::Id)
                .into(),
        }
    }
}

// `Related` trait has to be implemented by hand
impl Related<AcademicPriorLearningRecognitionTransactionEvaluation::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Evaluation.def()
    }
}

// `Related` trait has to be implemented by hand
impl Related<AcademicPriorLearningRecognitionReferenceEvidenceType::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EvidenceType.def()
    }
}

// `Related` trait has to be implemented by hand
impl Related<DocumentTransactionArchive::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Archive.def()
    }
}

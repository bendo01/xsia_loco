use crate::models::document::reference::archive_types::_entities::archive_types as DocumentReferenceArchiveType;
use crate::models::academic::prior_learning_recognition::transaction::evaluation_details::_entities::evaluation_details as AcademicPriorLearningRecognitionTransactionEvaluationDetail;

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(
    schema_name = "document_transaction",
    table_name = "archives"
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub dir: String,
    pub mimetype: String,
    pub size: i32,
    pub archive_type_id: Uuid,
    pub archiveable_id: Uuid,
    pub archiveable_type: String,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    ArchiveType,
    EvaluationDetails,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::ArchiveType => Entity::belongs_to(DocumentReferenceArchiveType::Entity)
                .from(Column::ArchiveTypeId)
                .to(DocumentReferenceArchiveType::Column::Id)
                .into(),
            Self::EvaluationDetails => Entity::has_many(AcademicPriorLearningRecognitionTransactionEvaluationDetail::Entity)
                .from(Column::Id)
                .to(AcademicPriorLearningRecognitionTransactionEvaluationDetail::Column::ArchiveId)
                .into(),
        }
    }
}

// `Related` trait has to be implemented by hand
impl Related<DocumentReferenceArchiveType::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ArchiveType.def()
    }
}

// `Related` trait has to be implemented by hand
impl Related<AcademicPriorLearningRecognitionTransactionEvaluationDetail::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EvaluationDetails.def()
    }
}
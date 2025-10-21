use crate::models::academic::prior_learning_recognition::transaction::evaluation_details::_entities::evaluation_details as AcademicPriorLearningRecognitionTransactionEvaluationDetail;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(
    schema_name = "academic_prior_learning_recognition_reference",
    table_name = "evidence_types"
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub code: i32,
    pub alphabet_code: String,
    pub name: String,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    EvaluationDetails,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::EvaluationDetails => Entity::has_many(
                AcademicPriorLearningRecognitionTransactionEvaluationDetail::Entity,
            )
            .from(Column::Id)
            .to(AcademicPriorLearningRecognitionTransactionEvaluationDetail::Column::EvidenceTypeId)
            .into(),
        }
    }
}

impl Related<AcademicPriorLearningRecognitionTransactionEvaluationDetail::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EvaluationDetails.def()
    }
}

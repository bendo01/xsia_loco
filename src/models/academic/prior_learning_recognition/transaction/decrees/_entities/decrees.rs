use crate::models::academic::prior_learning_recognition::transaction::evaluations::_entities::evaluations as AcademicPriorLearningRecognitionTransactionEvaluation;

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(
    schema_name = "academic_prior_learning_recognition_transaction",
    table_name = "decrees"
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub decree_number: String,
    pub decree_date: Date,
    pub evaluation_id: Uuid,
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
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Evaluation => Entity::belongs_to(AcademicPriorLearningRecognitionTransactionEvaluation::Entity)
                .from(Column::EvaluationId)
                .to(AcademicPriorLearningRecognitionTransactionEvaluation::Column::Id)
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
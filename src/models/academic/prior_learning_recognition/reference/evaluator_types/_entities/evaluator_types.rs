use crate::models::academic::prior_learning_recognition::transaction::evaluators::_entities::evaluators as AcademicPriorLearningRecognitionTransactionEvaluators;

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(
    schema_name = "academic_prior_learning_recognition_reference",
    table_name = "evaluator_types"
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
    Evaluators,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Evaluators => Entity::has_many(AcademicPriorLearningRecognitionTransactionEvaluators::Entity)
                .from(Column::Id)
                .to(AcademicPriorLearningRecognitionTransactionEvaluators::Column::EvaluatorTypeId)
                .into(),
        }
    }
}

// `Related` trait has to be implemented by hand
impl Related<AcademicPriorLearningRecognitionTransactionEvaluators::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Evaluators.def()
    }
}

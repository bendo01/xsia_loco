use crate::models::academic::prior_learning_recognition::reference::evaluator_types::_entities::evaluator_types as AcademicPriorLearningRecognitionReferenceEvaluatorTypes;
use crate::models::academic::prior_learning_recognition::transaction::recognitions::_entities::recognitions as AcademicPriorLearningRecognitionTransactionRecognotions;
use crate::models::person::master::individuals::_entities::individuals as PersonMasterIndividuals;

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(
    schema_name = "academic_prior_learning_recognition_transaction",
    table_name = "evaluators"
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub individual_id: Uuid,
    pub evaluator_type_id: Uuid,
    pub recognition_id: Uuid,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    EvaluatorType,
    Recognotion,
    Individual,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::EvaluatorType => {
                Entity::belongs_to(AcademicPriorLearningRecognitionReferenceEvaluatorTypes::Entity)
                    .from(Column::EvaluatorTypeId)
                    .to(AcademicPriorLearningRecognitionReferenceEvaluatorTypes::Column::Id)
                    .into()
            }
            Self::Recognotion => {
                Entity::belongs_to(AcademicPriorLearningRecognitionTransactionRecognotions::Entity)
                    .from(Column::EvaluatorTypeId)
                    .to(AcademicPriorLearningRecognitionTransactionRecognotions::Column::Id)
                    .into()
            }
            Self::Individual => Entity::belongs_to(PersonMasterIndividuals::Entity)
                .from(Column::IndividualId)
                .to(PersonMasterIndividuals::Column::Id)
                .into(),
        }
    }
}

// `Related` trait has to be implemented by hand
impl Related<AcademicPriorLearningRecognitionReferenceEvaluatorTypes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EvaluatorType.def()
    }
}

// `Related` trait has to be implemented by hand
impl Related<AcademicPriorLearningRecognitionTransactionRecognotions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Recognotion.def()
    }
}

impl Related<PersonMasterIndividuals::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Individual.def()
    }
}

use crate::models::academic::prior_learning_recognition::transaction::recognitions::_entities::recognitions as AcademicPriorLearningRecognitionTransactionRecognition;
use crate::models::academic::course::master::course_evaluation_plannings::_entities::course_evaluation_plannings as AcademicMasterCourseEvaluationPlanningCourseEvaluationPlanning;
use crate::models::academic::prior_learning_recognition::reference::professionalisms::_entities::professionalisms as AcademicPriorLearningRecognitionReferenceProfessionalism;
use crate::models::academic::prior_learning_recognition::reference::evidence_types::_entities::evidence_types as AcademicPriorLearningRecognitionReferenceEvidenceType;
use crate::models::academic::prior_learning_recognition::transaction::decrees::_entities::decrees as AcademicPriorLearningRecognitionTransactionDecree;
use crate::models::academic::prior_learning_recognition::transaction::evaluation_details::_entities::evaluation_details as AcademicPriorLearningRecognitionTransactionEvaluationDetail;

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(
    schema_name = "academic_prior_learning_recognition_transaction",
    table_name = "evaluations"
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub recognition_id: Uuid,
    pub course_evaluation_planning_id: Uuid,
    pub professionalism_id: Uuid,
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
    Recognition,
    CourseEvaluationPlanning,
    Professionalism,
    EvidenceType,
    Decree,
    EvaluationDetails,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Recognition => Entity::belongs_to(AcademicPriorLearningRecognitionTransactionRecognition::Entity)
                .from(Column::RecognitionId)
                .to(AcademicPriorLearningRecognitionTransactionRecognition::Column::Id)
                .into(),
            Self::Professionalism => Entity::belongs_to(AcademicPriorLearningRecognitionReferenceProfessionalism::Entity)
                .from(Column::RecognitionId)
                .to(AcademicPriorLearningRecognitionReferenceProfessionalism::Column::Id)
                .into(),
            Self::CourseEvaluationPlanning => Entity::belongs_to(AcademicMasterCourseEvaluationPlanningCourseEvaluationPlanning::Entity)
                .from(Column::CourseEvaluationPlanningId)
                .to(AcademicMasterCourseEvaluationPlanningCourseEvaluationPlanning::Column::Id)
                .into(),
            Self::EvidenceType => Entity::belongs_to(AcademicPriorLearningRecognitionReferenceEvidenceType::Entity)
                .from(Column::EvidenceTypeId)
                .to(AcademicPriorLearningRecognitionReferenceEvidenceType::Column::Id)
                .into(),
            Self::Decree => Entity::has_one(AcademicPriorLearningRecognitionTransactionDecree::Entity)
                .from(Column::Id)
                .to(AcademicPriorLearningRecognitionTransactionDecree::Column::EvaluationId)
                .into(),
            Self::EvaluationDetails => Entity::has_many(AcademicPriorLearningRecognitionTransactionEvaluationDetail::Entity)
                .from(Column::Id)
                .to(AcademicPriorLearningRecognitionTransactionEvaluationDetail::Column::EvaluationId)
                .into(),
        }
    }
}

// `Related` trait has to be implemented by hand
impl Related<AcademicPriorLearningRecognitionTransactionRecognition::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Recognition.def()
    }
}

impl Related<AcademicPriorLearningRecognitionReferenceProfessionalism::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Professionalism.def()
    }
}

impl Related<AcademicMasterCourseEvaluationPlanningCourseEvaluationPlanning::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CourseEvaluationPlanning.def()
    }
}

impl Related<AcademicPriorLearningRecognitionReferenceEvidenceType::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EvidenceType.def()
    }
}

impl Related<AcademicPriorLearningRecognitionTransactionDecree::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Decree.def()
    }
}
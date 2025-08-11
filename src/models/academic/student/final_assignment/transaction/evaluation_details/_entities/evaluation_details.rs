use crate::models::academic::student::final_assignment::transaction::evaluation_summaries::_entities::evaluation_summaries as AcademicStudentFinalAssignmentTransactionEvaluationSummary;
use crate::models::academic::campaign::transaction::grades::_entities::grades as AcademicCampaignTransactionGrade;
use crate::models::academic::student::final_assignment::transaction::advisers::_entities::advisers as AcademicCampaignTransactionAdviser;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(
    schema_name = "academic_student_final_assignment_transaction",
    table_name = "evaluation_details"
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub evaluation_summary_id: Uuid,
    pub adviser_id: Uuid,
    #[sea_orm(column_type = "Float", nullable)]
    pub mark: Option<f32>,
    pub grade_id: Option<Uuid>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    EvaluationSummary,
    Grade,
    Adviser,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::EvaluationSummary => Entity::belongs_to(
                AcademicStudentFinalAssignmentTransactionEvaluationSummary::Entity,
            )
            .from(Column::EvaluationSummaryId)
            .to(AcademicStudentFinalAssignmentTransactionEvaluationSummary::Column::Id)
            .into(),
            Self::Grade => Entity::belongs_to(AcademicCampaignTransactionGrade::Entity)
                .from(Column::GradeId)
                .to(AcademicCampaignTransactionGrade::Column::Id)
                .into(),
            Self::Adviser => Entity::belongs_to(AcademicCampaignTransactionAdviser::Entity)
                .from(Column::AdviserId)
                .to(AcademicCampaignTransactionAdviser::Column::Id)
                .into(),
        }
    }
}

impl Related<AcademicStudentFinalAssignmentTransactionEvaluationSummary::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EvaluationSummary.def()
    }
}

impl Related<AcademicCampaignTransactionGrade::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Grade.def()
    }
}

impl Related<AcademicCampaignTransactionAdviser::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Adviser.def()
    }
}

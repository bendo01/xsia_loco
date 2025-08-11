use crate::models::academic::campaign::transaction::grades::_entities::grades as AcademicCampaignTransactionGrade;
use crate::models::academic::student::campaign::detail_activities::_entities::detail_activities as AcademicStudentCampaignDetailActivity;
use crate::models::academic::student::final_assignment::reference::stages::_entities::stages as AcademicStudentFinalAssignmentReferenceStage;
use crate::models::academic::student::final_assignment::transaction::submissions::_entities::submissions as AcademicStudentFinalAssignmentTransactionSubmission;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(
    schema_name = "academic_student_final_assignment_transaction",
    table_name = "evaluation_summaries"
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub submission_id: Option<Uuid>,
    pub detail_activity_id: Uuid,
    pub stage_id: Uuid,
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
    DetailActivity,
    Grade,
    Submission,
    Stage,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::DetailActivity => {
                Entity::belongs_to(AcademicStudentCampaignDetailActivity::Entity)
                    .from(Column::DetailActivityId)
                    .to(AcademicStudentCampaignDetailActivity::Column::Id)
                    .into()
            }
            Self::Grade => Entity::belongs_to(AcademicCampaignTransactionGrade::Entity)
                .from(Column::GradeId)
                .to(AcademicCampaignTransactionGrade::Column::Id)
                .into(),
            Self::Submission => {
                Entity::belongs_to(AcademicStudentFinalAssignmentTransactionSubmission::Entity)
                    .from(Column::SubmissionId)
                    .to(AcademicStudentFinalAssignmentTransactionSubmission::Column::Id)
                    .into()
            }
            Self::Stage => Entity::belongs_to(AcademicStudentFinalAssignmentReferenceStage::Entity)
                .from(Column::StageId)
                .to(AcademicStudentFinalAssignmentReferenceStage::Column::Id)
                .into(),
        }
    }
}

impl Related<AcademicStudentCampaignDetailActivity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DetailActivity.def()
    }
}

impl Related<AcademicCampaignTransactionGrade::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Grade.def()
    }
}

impl Related<AcademicStudentFinalAssignmentTransactionSubmission::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Submission.def()
    }
}

impl Related<AcademicStudentFinalAssignmentReferenceStage::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Stage.def()
    }
}

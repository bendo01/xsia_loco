use crate::models::academic::student::final_assignment::transaction::schedules::_entities::schedules as AcademicStudentFinalAssignmentTransactionSchedule;
use crate::models::academic::student::final_assignment::transaction::submissions::_entities::submissions as AcademicStudentFinalAssignmentTransactionSubmission;
use crate::models::academic::student::final_assignment::transaction::prerequisites::_entities::prerequisites as AcademicStudentFinalAssignmentTransactionPrerequisite;
use crate::models::academic::student::final_assignment::transaction::evaluation_summaries::_entities::evaluation_summaries as AcademicStudentFinalAssignmentTransactionEvaluationSummary;

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(
    schema_name = "academic_student_final_assignment_reference",
    table_name = "stages"
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
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Submissions,
    Prerequisites,
    EvaluationSummaries,
    Schedules,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Submissions => {
                Entity::has_many(AcademicStudentFinalAssignmentTransactionSubmission::Entity)
                    .from(Column::Id)
                    .to(AcademicStudentFinalAssignmentTransactionSubmission::Column::StageId)
                    .into()
            }
            Self::Prerequisites => {
                Entity::has_many(AcademicStudentFinalAssignmentTransactionPrerequisite::Entity)
                    .from(Column::Id)
                    .to(AcademicStudentFinalAssignmentTransactionPrerequisite::Column::StageId)
                    .into()
            }
            Self::EvaluationSummaries => {
                Entity::has_many(AcademicStudentFinalAssignmentTransactionEvaluationSummary::Entity)
                    .from(Column::Id)
                    .to(AcademicStudentFinalAssignmentTransactionEvaluationSummary::Column::StageId)
                    .into()
            }
            Self::Schedules => {
                Entity::has_many(AcademicStudentFinalAssignmentTransactionSchedule::Entity)
                    .from(Column::Id)
                    .to(AcademicStudentFinalAssignmentTransactionSchedule::Column::StageId)
                    .into()
            }
        }
    }
}

impl Related<AcademicStudentFinalAssignmentTransactionSubmission::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Submissions.def()
    }
}

impl Related<AcademicStudentFinalAssignmentTransactionPrerequisite::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Prerequisites.def()
    }
}

impl Related<AcademicStudentFinalAssignmentTransactionEvaluationSummary::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EvaluationSummaries.def()
    }
}

impl Related<AcademicStudentFinalAssignmentTransactionSchedule::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Schedules.def()
    }
}

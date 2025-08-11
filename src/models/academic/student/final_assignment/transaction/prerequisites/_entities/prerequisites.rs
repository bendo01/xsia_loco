use crate::models::academic::student::final_assignment::reference::approval_types::_entities::approval_types as AcademicStudentFinalAssignmentReferenceApprovalType;
use crate::models::academic::student::final_assignment::reference::requirements::_entities::requirements as AcademicStudentFinalAssignmentReferenceRequirement;
use crate::models::academic::student::final_assignment::reference::stages::_entities::stages as AcademicStudentFinalAssignmentReferenceStage;
use crate::models::academic::student::final_assignment::transaction::submissions::_entities::submissions as AcademicStudentFinalAssignmentTransactionSubmission;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(
    schema_name = "academic_student_final_assignment_transaction",
    table_name = "prerequisites"
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub thread: i32,
    pub requirement_id: Uuid,
    pub submission_id: Uuid,
    pub approval_type_id: Uuid,
    pub stage_id: Uuid,
    pub filename: Option<String>,
    pub dir: Option<String>,
    pub r#type: Option<String>,
    pub filesize: Option<i32>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Requirement,
    Submission,
    Stage,
    ApprovalType,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Requirement => {
                Entity::belongs_to(AcademicStudentFinalAssignmentReferenceRequirement::Entity)
                    .from(Column::RequirementId)
                    .to(AcademicStudentFinalAssignmentReferenceRequirement::Column::Id)
                    .into()
            }
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
            Self::ApprovalType => {
                Entity::belongs_to(AcademicStudentFinalAssignmentReferenceApprovalType::Entity)
                    .from(Column::ApprovalTypeId)
                    .to(AcademicStudentFinalAssignmentReferenceApprovalType::Column::Id)
                    .into()
            }
        }
    }
}

impl Related<AcademicStudentFinalAssignmentReferenceRequirement::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Requirement.def()
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

impl Related<AcademicStudentFinalAssignmentReferenceApprovalType::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ApprovalType.def()
    }
}

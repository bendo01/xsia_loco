use crate::models::academic::student::campaign::detail_activities::_entities::detail_activities as AcademicStudentCampaignDetailActivity;
use crate::models::academic::student::final_assignment::reference::stages::_entities::stages as AcademicStudentFinalAssignmentReferenceStage;
use crate::models::academic::student::final_assignment::transaction::submissions::_entities::submissions as AcademicStudentFinalAssignmentTransactionSubmission;
use crate::models::building::master::rooms::_entities::rooms as BuildingMasterRoom;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(
    schema_name = "academic_student_final_assignment_transaction",
    table_name = "schedules"
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub decree_number: Option<String>,
    pub schedule_date: Option<Date>,
    pub schedule_time: Option<Time>,
    pub submission_id: Option<Uuid>,
    pub detail_activity_id: Uuid,
    pub stage_id: Uuid,
    pub room_id: Option<Uuid>,
    #[sea_orm(column_type = "Text", nullable)]
    pub zoom_meeting: Option<String>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Submission,
    DetailActivity,
    Stage,
    Room,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Submission => {
                Entity::belongs_to(AcademicStudentFinalAssignmentTransactionSubmission::Entity)
                    .from(Column::SubmissionId)
                    .to(AcademicStudentFinalAssignmentTransactionSubmission::Column::Id)
                    .into()
            }
            Self::DetailActivity => {
                Entity::belongs_to(AcademicStudentCampaignDetailActivity::Entity)
                    .from(Column::DetailActivityId)
                    .to(AcademicStudentCampaignDetailActivity::Column::Id)
                    .into()
            }
            Self::Stage => Entity::belongs_to(AcademicStudentFinalAssignmentReferenceStage::Entity)
                .from(Column::StageId)
                .to(AcademicStudentFinalAssignmentReferenceStage::Column::Id)
                .into(),
            Self::Room => Entity::belongs_to(BuildingMasterRoom::Entity)
                .from(Column::RoomId)
                .to(BuildingMasterRoom::Column::Id)
                .into(),
        }
    }
}

impl Related<AcademicStudentFinalAssignmentTransactionSubmission::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Submission.def()
    }
}

impl Related<AcademicStudentCampaignDetailActivity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DetailActivity.def()
    }
}

impl Related<AcademicStudentFinalAssignmentReferenceStage::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Stage.def()
    }
}

impl Related<BuildingMasterRoom::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Room.def()
    }
}

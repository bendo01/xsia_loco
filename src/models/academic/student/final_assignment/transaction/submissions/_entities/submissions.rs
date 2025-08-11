use crate::models::academic::student::campaign::detail_activities::_entities::detail_activities as AcademicStudentCampaignDetailActivity;
use crate::models::academic::student::final_assignment::reference::approval_types::_entities::approval_types as AcademicStudentFinalAssignmentReferenceApprovalType;
use crate::models::academic::student::final_assignment::reference::categories::_entities::categories as AcademicStudentFinalAssignmentReferenceCategory;
use crate::models::academic::student::final_assignment::reference::stages::_entities::stages as AcademicStudentFinalAssignmentReferenceStage;
use crate::models::academic::student::final_assignment::transaction::final_assignment_decrees::_entities::final_assignment_decrees as AcademicStudentFinalAssignmentTransactionFinalAssignmentDecree;
use crate::models::academic::student::master::students::_entities::students as AcademicStudentMasterStudent;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(
    schema_name = "academic_student_final_assignment_transaction",
    table_name = "submissions"
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(column_type = "Text")]
    pub title: String,
    pub student_id: Uuid,
    pub approval_type_id: Option<Uuid>,
    pub category_id: Option<Uuid>,
    pub stage_id: Option<Uuid>,
    pub final_assignment_decree_id: Option<Uuid>,
    pub detail_activity_id: Uuid,
    pub is_taken: Option<DateTime>,
    pub is_lock: Option<DateTime>,
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
    Student,
    ApprovalType,
    Category,
    Stage,
    FinalAssignmentDecree,
    DetailActivity,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Student => Entity::belongs_to(AcademicStudentMasterStudent::Entity)
                .from(Column::StudentId)
                .to(AcademicStudentMasterStudent::Column::Id)
                .into(),
            Self::ApprovalType => {
                Entity::belongs_to(AcademicStudentFinalAssignmentReferenceApprovalType::Entity)
                    .from(Column::ApprovalTypeId)
                    .to(AcademicStudentFinalAssignmentReferenceApprovalType::Column::Id)
                    .into()
            }
            Self::Category => {
                Entity::belongs_to(AcademicStudentFinalAssignmentReferenceCategory::Entity)
                    .from(Column::CategoryId)
                    .to(AcademicStudentFinalAssignmentReferenceCategory::Column::Id)
                    .into()
            }
            Self::Stage => Entity::belongs_to(AcademicStudentFinalAssignmentReferenceStage::Entity)
                .from(Column::StageId)
                .to(AcademicStudentFinalAssignmentReferenceStage::Column::Id)
                .into(),
            Self::FinalAssignmentDecree => Entity::belongs_to(
                AcademicStudentFinalAssignmentTransactionFinalAssignmentDecree::Entity,
            )
            .from(Column::FinalAssignmentDecreeId)
            .to(AcademicStudentFinalAssignmentTransactionFinalAssignmentDecree::Column::Id)
            .into(),
            Self::DetailActivity => {
                Entity::belongs_to(AcademicStudentCampaignDetailActivity::Entity)
                    .from(Column::DetailActivityId)
                    .to(AcademicStudentCampaignDetailActivity::Column::Id)
                    .into()
            }
        }
    }
}

impl Related<AcademicStudentMasterStudent::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Student.def()
    }
}

impl Related<AcademicStudentFinalAssignmentReferenceApprovalType::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ApprovalType.def()
    }
}

impl Related<AcademicStudentFinalAssignmentReferenceCategory::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Category.def()
    }
}

impl Related<AcademicStudentFinalAssignmentReferenceStage::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Stage.def()
    }
}

impl Related<AcademicStudentFinalAssignmentTransactionFinalAssignmentDecree::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FinalAssignmentDecree.def()
    }
}

impl Related<AcademicStudentCampaignDetailActivity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DetailActivity.def()
    }
}

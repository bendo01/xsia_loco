use crate::models::academic::lecturer::master::lecturers::_entities::lecturers as AcademicLecturerMasterLecturer;
use crate::models::academic::student::campaign::detail_activities::_entities::detail_activities as AcademicStudentCampaignDetailActivity;
use crate::models::academic::student::final_assignment::reference::adviser_categories::_entities::adviser_categories as AcademicStudentFinalAssignmentReferenceAdviserCategory;
use crate::models::academic::student::final_assignment::transaction::submissions::_entities::submissions as AcademicStudentFinalAssignmentTransactionSubmission;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(
    schema_name = "academic_student_final_assignment_transaction",
    table_name = "advisers"
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub thread: i32,
    pub lecturer_id: Uuid,
    pub detail_activity_id: Uuid,
    pub submission_id: Option<Uuid>,
    pub adviser_category_id: Uuid,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Lecturer,
    DetailActivity,
    Submission,
    AdviserCategory,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Lecturer => Entity::belongs_to(AcademicLecturerMasterLecturer::Entity)
                .from(Column::LecturerId)
                .to(AcademicLecturerMasterLecturer::Column::Id)
                .into(),
            Self::DetailActivity => {
                Entity::belongs_to(AcademicStudentCampaignDetailActivity::Entity)
                    .from(Column::DetailActivityId)
                    .to(AcademicStudentCampaignDetailActivity::Column::Id)
                    .into()
            }
            Self::Submission => {
                Entity::belongs_to(AcademicStudentFinalAssignmentTransactionSubmission::Entity)
                    .from(Column::SubmissionId)
                    .to(AcademicStudentFinalAssignmentTransactionSubmission::Column::Id)
                    .into()
            }
            Self::AdviserCategory => {
                Entity::belongs_to(AcademicStudentFinalAssignmentReferenceAdviserCategory::Entity)
                    .from(Column::AdviserCategoryId)
                    .to(AcademicStudentFinalAssignmentReferenceAdviserCategory::Column::Id)
                    .into()
            }
        }
    }
}

impl Related<AcademicLecturerMasterLecturer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Lecturer.def()
    }
}

impl Related<AcademicStudentCampaignDetailActivity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DetailActivity.def()
    }
}

impl Related<AcademicStudentFinalAssignmentTransactionSubmission::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Submission.def()
    }
}

impl Related<AcademicStudentFinalAssignmentReferenceAdviserCategory::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AdviserCategory.def()
    }
}

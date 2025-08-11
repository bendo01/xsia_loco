use crate::models::academic::lecturer::master::lecturers::_entities::lecturers as AcademicLecturerMasterLecturer;
use crate::models::academic::student::campaign::detail_activities::_entities::detail_activities as AcademicStudentCampaignDetailActivity;
use crate::models::academic::student::final_assignment::reference::adviser_categories::_entities::adviser_categories as AcademicStudentFinalAssignmentReferenceAdviserCategory;
use crate::models::academic::student::final_assignment::transaction::advisers::_entities::advisers as AcademicStudentFinalAssignmentTransactionAdviser;
use crate::models::academic::student::final_assignment::transaction::submissions::_entities::submissions as AcademicStudentFinalAssignmentTransactionSubmission;
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(clippy::too_many_lines)]
pub struct AdviserDataObject {
    pub adviser: AcademicStudentFinalAssignmentTransactionAdviser::Model,
    pub lecturer: Option<AcademicLecturerMasterLecturer::Model>,
    pub detail_activity: Option<AcademicStudentCampaignDetailActivity::Model>,
    pub submission: Option<AcademicStudentFinalAssignmentTransactionSubmission::Model>,
    pub adviser_category: Option<AcademicStudentFinalAssignmentReferenceAdviserCategory::Model>,
}

impl AdviserDataObject {
    #[allow(clippy::missing_errors_doc)]
    #[allow(clippy::too_many_lines)]
    pub async fn get_by_id(ctx: &AppContext, id: Uuid) -> Result<Option<Self>> {
        let adviser = AcademicStudentFinalAssignmentTransactionAdviser::Entity::find_by_id(id)
            .filter(AcademicStudentFinalAssignmentTransactionAdviser::Column::DeletedAt.is_null())
            .one(&ctx.db)
            .await?;
        if let Some(adviser) = adviser {
            let lecturer = adviser
                .find_related(AcademicLecturerMasterLecturer::Entity)
                .filter(AcademicLecturerMasterLecturer::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            let detail_activity = adviser
                .find_related(AcademicStudentCampaignDetailActivity::Entity)
                .filter(AcademicStudentCampaignDetailActivity::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            let submission = adviser
                .find_related(AcademicStudentFinalAssignmentTransactionSubmission::Entity)
                .filter(
                    AcademicStudentFinalAssignmentTransactionSubmission::Column::DeletedAt
                        .is_null(),
                )
                .one(&ctx.db)
                .await?;

            let adviser_category = adviser
                .find_related(AcademicStudentFinalAssignmentReferenceAdviserCategory::Entity)
                .filter(
                    AcademicStudentFinalAssignmentReferenceAdviserCategory::Column::DeletedAt
                        .is_null(),
                )
                .one(&ctx.db)
                .await?;

            Ok(Some(Self {
                adviser,
                lecturer,
                detail_activity,
                submission,
                adviser_category,
            }))
        } else {
            Ok(None)
        }
    }
}

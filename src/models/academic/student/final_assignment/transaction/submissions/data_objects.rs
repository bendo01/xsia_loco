use crate::models::academic::student::campaign::detail_activities::_entities::detail_activities as AcademicStudentCampaignDetailActivity;
use crate::models::academic::student::final_assignment::reference::approval_types::_entities::approval_types as AcademicStudentFinalAssignmentReferenceApprovalType;
use crate::models::academic::student::final_assignment::reference::categories::_entities::categories as AcademicStudentFinalAssignmentReferenceCategory;
use crate::models::academic::student::final_assignment::reference::stages::_entities::stages as AcademicStudentFinalAssignmentReferenceStage;
use crate::models::academic::student::final_assignment::transaction::advisers::_entities::advisers as AcademicStudentFinalAssignmentTransactionAdviser;
use crate::models::academic::student::final_assignment::transaction::final_assignment_decrees::_entities::final_assignment_decrees as AcademicStudentFinalAssignmentTransactionFinalAssignmentDecree;
use crate::models::academic::student::final_assignment::transaction::submissions::_entities::submissions as AcademicStudentFinalAssignmentTransactionSubmission;
use crate::models::academic::student::master::students::_entities::students as AcademicStudentMasterStudent;
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(clippy::too_many_lines)]
pub struct SubmissionDataObject {
    pub submission: AcademicStudentFinalAssignmentTransactionSubmission::Model,
    pub student: Option<AcademicStudentMasterStudent::Model>,
    pub approval_type: Option<AcademicStudentFinalAssignmentReferenceApprovalType::Model>,
    pub category: Option<AcademicStudentFinalAssignmentReferenceCategory::Model>,
    pub stage: Option<AcademicStudentFinalAssignmentReferenceStage::Model>,
    pub final_assignment_decree:
        Option<AcademicStudentFinalAssignmentTransactionFinalAssignmentDecree::Model>,
    pub detail_activity: Option<AcademicStudentCampaignDetailActivity::Model>,
    pub advisers: Option<Vec<AcademicStudentFinalAssignmentTransactionAdviser::Model>>,
}

impl SubmissionDataObject {
    #[allow(clippy::missing_errors_doc)]
    #[allow(clippy::too_many_lines)]
    pub async fn get_by_id(
        ctx: &AppContext,
        id: Uuid,
        with_has_many_relationships: bool,
    ) -> Result<Option<Self>> {
        let submission =
            AcademicStudentFinalAssignmentTransactionSubmission::Entity::find_by_id(id)
                .filter(
                    AcademicStudentFinalAssignmentTransactionSubmission::Column::DeletedAt
                        .is_null(),
                )
                .one(&ctx.db)
                .await?;
        if let Some(submission) = submission {
            let student = submission
                .find_related(AcademicStudentMasterStudent::Entity)
                .filter(AcademicStudentMasterStudent::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            let approval_type = submission
                .find_related(AcademicStudentFinalAssignmentReferenceApprovalType::Entity)
                .filter(
                    AcademicStudentFinalAssignmentReferenceApprovalType::Column::DeletedAt
                        .is_null(),
                )
                .one(&ctx.db)
                .await?;

            let category = submission
                .find_related(AcademicStudentFinalAssignmentReferenceCategory::Entity)
                .filter(
                    AcademicStudentFinalAssignmentReferenceCategory::Column::DeletedAt.is_null(),
                )
                .one(&ctx.db)
                .await?;

            let stage = submission
                .find_related(AcademicStudentFinalAssignmentReferenceStage::Entity)
                .filter(AcademicStudentFinalAssignmentReferenceStage::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            let final_assignment_decree = submission
                .find_related(AcademicStudentFinalAssignmentTransactionFinalAssignmentDecree::Entity)
                .filter(AcademicStudentFinalAssignmentTransactionFinalAssignmentDecree::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            let detail_activity = submission
                .find_related(AcademicStudentCampaignDetailActivity::Entity)
                .filter(AcademicStudentCampaignDetailActivity::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            let advisers = if with_has_many_relationships {
                // Using reverse relation since adviser has submission_id
                let advisers_list =
                    AcademicStudentFinalAssignmentTransactionAdviser::Entity::find()
                        .filter(
                            AcademicStudentFinalAssignmentTransactionAdviser::Column::SubmissionId
                                .eq(submission.id),
                        )
                        .filter(
                            AcademicStudentFinalAssignmentTransactionAdviser::Column::DeletedAt
                                .is_null(),
                        )
                        .all(&ctx.db)
                        .await?;

                if advisers_list.is_empty() {
                    None
                } else {
                    Some(advisers_list)
                }
            } else {
                None
            };

            Ok(Some(Self {
                submission,
                student,
                approval_type,
                category,
                stage,
                final_assignment_decree,
                detail_activity,
                advisers,
            }))
        } else {
            Ok(None)
        }
    }
}

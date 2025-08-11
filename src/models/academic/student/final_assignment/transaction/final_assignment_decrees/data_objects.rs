use crate::models::academic::campaign::transaction::activities::_entities::activities as AcademicCampaignTransactionActivity;
use crate::models::academic::student::final_assignment::transaction::final_assignment_decrees::_entities::final_assignment_decrees as AcademicStudentFinalAssignmentTransactionFinalAssignmentDecree;
use crate::models::institution::master::staffes::_entities::staffes as InstitutionMasterStaff;
use crate::models::institution::master::units::_entities::units as InstitutionMasterUnit;
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(clippy::too_many_lines)]
pub struct FinalAssignmentDecreeDataObject {
    pub final_assignment_decree:
        AcademicStudentFinalAssignmentTransactionFinalAssignmentDecree::Model,
    pub unit: Option<InstitutionMasterUnit::Model>,
    pub activity: Option<AcademicCampaignTransactionActivity::Model>,
    pub staff: Option<InstitutionMasterStaff::Model>,
}

impl FinalAssignmentDecreeDataObject {
    #[allow(clippy::missing_errors_doc)]
    #[allow(clippy::too_many_lines)]
    pub async fn get_by_id(ctx: &AppContext, id: Uuid) -> Result<Option<Self>> {
        let final_assignment_decree = AcademicStudentFinalAssignmentTransactionFinalAssignmentDecree::Entity::find_by_id(id)
            .filter(AcademicStudentFinalAssignmentTransactionFinalAssignmentDecree::Column::DeletedAt.is_null())
            .one(&ctx.db)
            .await?;
        if let Some(final_assignment_decree) = final_assignment_decree {
            let unit = final_assignment_decree
                .find_related(InstitutionMasterUnit::Entity)
                .filter(InstitutionMasterUnit::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            let activity = final_assignment_decree
                .find_related(AcademicCampaignTransactionActivity::Entity)
                .filter(AcademicCampaignTransactionActivity::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            let staff = final_assignment_decree
                .find_related(InstitutionMasterStaff::Entity)
                .filter(InstitutionMasterStaff::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            Ok(Some(Self {
                final_assignment_decree,
                unit,
                activity,
                staff,
            }))
        } else {
            Ok(None)
        }
    }
}

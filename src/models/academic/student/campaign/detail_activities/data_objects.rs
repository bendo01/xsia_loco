use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

use crate::models::academic::campaign::transaction::grades::_entities::grades as AcademicCampaignTransactionGrade;
use crate::models::academic::campaign::transaction::teaches::data_objects::DataObject as AcademicCampaignTransactionTeachDataObject;
use crate::models::academic::course::master::courses::_entities::courses as AcademicCourseMasterCourse;
use crate::models::academic::student::campaign::activities::_entities::activities as AcademicStudentCampaignActivity;
use crate::models::academic::student::campaign::detail_activities::_entities::detail_activities as AcademicStudentCampaignDetailActivity;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(clippy::too_many_lines)]
pub struct DataObject {
    pub detail_activity: AcademicStudentCampaignDetailActivity::Model,
    pub activity: Option<AcademicStudentCampaignActivity::Model>,
    pub course: Option<AcademicCourseMasterCourse::Model>,
    pub grade: Option<AcademicCampaignTransactionGrade::Model>,
    pub teach_activity: Option<AcademicCampaignTransactionTeachDataObject>,
}

impl DataObject {
    /// Retrieves a `DataObject` by its ID.
    ///
    /// # Errors
    /// Returns an error if there's a database connection issue or if the query fails.
    #[allow(clippy::too_many_lines)]
    pub async fn get_by_id(
        ctx: &AppContext,
        id: Uuid,
        _with_has_many_relationships: bool,
    ) -> Result<Option<Self>> {
        let detail_activity = AcademicStudentCampaignDetailActivity::Entity::find_by_id(id)
            .filter(AcademicStudentCampaignDetailActivity::Column::DeletedAt.is_null())
            .one(&ctx.db)
            .await?;
        if let Some(detail_activity) = detail_activity {
            let activity = detail_activity
                .find_related(AcademicStudentCampaignActivity::Entity)
                .filter(AcademicStudentCampaignActivity::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;
            let course = detail_activity
                .find_related(AcademicCourseMasterCourse::Entity)
                .filter(AcademicCourseMasterCourse::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;
            let grade = detail_activity
                .find_related(AcademicCampaignTransactionGrade::Entity)
                .filter(AcademicCampaignTransactionGrade::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;
            let teach_activity = AcademicCampaignTransactionTeachDataObject::get_by_id(
                ctx,
                detail_activity.teach_id,
                false,
            )
            .await?;
            Ok(Some(Self {
                detail_activity,
                activity,
                course,
                grade,
                teach_activity,
            }))
        } else {
            Ok(None)
        }
    }
}

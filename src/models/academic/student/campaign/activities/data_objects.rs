use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

use crate::models::academic::campaign::transaction::activities::data_objects::DataObject as AcademicCampaignTransactionActivityDataObject;
use crate::models::academic::student::campaign::activities::_entities::activities as AcademicStudentCampaignActivity;
use crate::models::academic::student::campaign::detail_activities::_entities::detail_activities as AcademicStudentCampaignDetailActivity;
use crate::models::academic::student::campaign::detail_activities::data_objects::DataObject as AcademicStudentCampaignDetailActivityDataObject;
use crate::models::academic::student::master::students::_entities::students as AcademicStudentMasterStudent;
use crate::models::academic::student::reference::finances::_entities::finances as AcademicStudentReferenceFinance;
use crate::models::academic::student::reference::resign_statuses::_entities::resign_statuses as AcademicStudentReferenceResignStatus;
use crate::models::academic::student::reference::statuses::_entities::statuses as AcademicStudentReferenceStatus;
use crate::models::institution::master::units::_entities::units as InstitutionMasterUnit;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(clippy::too_many_lines)]
pub struct DataObject {
    pub activity: AcademicStudentCampaignActivity::Model,
    pub unit_activity: Option<AcademicCampaignTransactionActivityDataObject>,
    pub student: Option<AcademicStudentMasterStudent::Model>,
    pub finance: Option<AcademicStudentReferenceFinance::Model>,
    pub resign_status: Option<AcademicStudentReferenceResignStatus::Model>,
    pub status: Option<AcademicStudentReferenceStatus::Model>,
    pub unit: Option<InstitutionMasterUnit::Model>,
    pub detail_activities: Option<Vec<AcademicStudentCampaignDetailActivityDataObject>>,
}

impl DataObject {
    /// Retrieves a `DataObject` by its ID.
    ///
    /// # Arguments
    /// * `ctx` - The application context containing the database connection
    /// * `id` - The UUID of the activity to find
    /// * `with_has_many_relationships` - Whether to load related detail activities
    ///
    /// # Returns
    /// An `Option<Self>` which is `Some` if the activity exists and `None` otherwise
    ///
    /// # Errors
    /// This function will return an error if:
    /// * There's a database connection error
    /// * A query operation fails
    #[allow(clippy::too_many_lines)]
    pub async fn get_by_id(
        ctx: &AppContext,
        id: Uuid,
        with_has_many_relationships: bool,
    ) -> Result<Option<Self>> {
        let activity = AcademicStudentCampaignActivity::Entity::find_by_id(id)
            .filter(AcademicStudentCampaignActivity::Column::DeletedAt.is_null())
            .one(&ctx.db)
            .await?;
        if let Some(activity) = activity {
            let student = activity
                .find_related(AcademicStudentMasterStudent::Entity)
                .filter(AcademicStudentMasterStudent::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;
            let finance = activity
                .find_related(AcademicStudentReferenceFinance::Entity)
                .filter(AcademicStudentReferenceFinance::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;
            let resign_status = activity
                .find_related(AcademicStudentReferenceResignStatus::Entity)
                .filter(AcademicStudentReferenceResignStatus::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;
            let status = activity
                .find_related(AcademicStudentReferenceStatus::Entity)
                .filter(AcademicStudentReferenceStatus::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;
            let unit = activity
                .find_related(InstitutionMasterUnit::Entity)
                .filter(InstitutionMasterUnit::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;
            // let unit_activity = activity
            //     .find_related(AcademicCampaignTransactionActivity::Entity)
            //     .filter(AcademicCampaignTransactionActivity::Column::DeletedAt.is_null())
            //     .one(&ctx.db)
            //     .await?;

            let unit_activity = AcademicCampaignTransactionActivityDataObject::get_by_id(
                ctx,
                activity.unit_activity_id,
                false,
            )
            .await?;

            // Conditionally load has-many relationships
            let detail_activities = if with_has_many_relationships {
                let list = activity
                    .find_related(AcademicStudentCampaignDetailActivity::Entity)
                    .filter(AcademicStudentCampaignDetailActivity::Column::DeletedAt.is_null())
                    .all(&ctx.db)
                    .await?;

                if list.is_empty() {
                    None
                } else {
                    let mut items = Vec::new();
                    for data in list {
                        if let Some(item_object) =
                            AcademicStudentCampaignDetailActivityDataObject::get_by_id(
                                ctx, data.id, false,
                            )
                            .await?
                        {
                            items.push(item_object);
                        }
                    }

                    if items.is_empty() { None } else { Some(items) }
                }
            } else {
                None
            };

            Ok(Some(Self {
                activity,
                unit_activity,
                student,
                finance,
                resign_status,
                status,
                unit,
                detail_activities,
            }))
        } else {
            Ok(None)
        }
    }
}

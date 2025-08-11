use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

use crate::models::academic::campaign::transaction::activities::_entities::activities as AcademicCampaignTransactionActivity;
use crate::models::academic::general::reference::academic_years::_entities::academic_years as AcademicGeneralReferenceAcademicYear;
use crate::models::institution::master::units::_entities::units as InstitutionMasterUnit;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(clippy::too_many_lines)]
pub struct DataObject {
    pub activity: AcademicCampaignTransactionActivity::Model,
    pub academic_year: Option<AcademicGeneralReferenceAcademicYear::Model>,
    pub unit: Option<InstitutionMasterUnit::Model>,
}

impl DataObject {
    /// Retrieves a data object by its ID.
    ///
    /// # Arguments
    /// * `ctx` - The application context
    /// * `id` - The UUID of the activity to retrieve
    /// * `_with_has_many_relationships` - Whether to include has-many relationships
    ///
    /// # Returns
    /// * `Ok(Some(DataObject))` - If the activity exists and was successfully retrieved
    /// * `Ok(None)` - If no activity with the given ID exists
    ///
    /// # Errors
    /// This function will return an error if database operations fail.
    #[allow(clippy::too_many_lines)]
    pub async fn get_by_id(
        ctx: &AppContext,
        id: Uuid,
        _with_has_many_relationships: bool,
    ) -> Result<Option<Self>> {
        let activity = AcademicCampaignTransactionActivity::Entity::find_by_id(id)
            .filter(AcademicCampaignTransactionActivity::Column::DeletedAt.is_null())
            .one(&ctx.db)
            .await?;
        if let Some(activity) = activity {
            let academic_year = activity
                .find_related(AcademicGeneralReferenceAcademicYear::Entity)
                .filter(AcademicGeneralReferenceAcademicYear::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;
            let unit = activity
                .find_related(InstitutionMasterUnit::Entity)
                .filter(InstitutionMasterUnit::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;
            Ok(Some(Self {
                activity,
                academic_year,
                unit,
            }))
        } else {
            Ok(None)
        }
    }
}

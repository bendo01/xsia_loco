use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

use crate::models::academic::campaign::reference::encounter_categories::_entities::encounter_categories as AcademicCampaignnReferenceEncounterCategory;
use crate::models::academic::campaign::reference::scopes::_entities::scopes as AcademicCampaignReferenceScope;
use crate::models::academic::campaign::transaction::activities::_entities::activities as AcademicCampaignTransactionActivity;
use crate::models::academic::campaign::transaction::class_codes::_entities::class_codes as AcademicCampaignTransactionClassCode;
use crate::models::academic::campaign::transaction::teach_decrees::_entities::teach_decrees as AcademicCampaignTransactionTeachDecree;
use crate::models::academic::campaign::transaction::teaches::_entities::teaches as AcademicCampaignTransactionTeach;
use crate::models::academic::course::master::courses::_entities::courses as AcademicCourseMasterCourse;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(clippy::too_many_lines)]
pub struct DataObject {
    pub teach: AcademicCampaignTransactionTeach::Model,
    pub encounter_category: Option<AcademicCampaignnReferenceEncounterCategory::Model>,
    pub scope: Option<AcademicCampaignReferenceScope::Model>,
    pub activity: Option<AcademicCampaignTransactionActivity::Model>,
    pub course: Option<AcademicCourseMasterCourse::Model>,
    pub teach_decree: Option<AcademicCampaignTransactionTeachDecree::Model>,
    pub class_code: Option<AcademicCampaignTransactionClassCode::Model>,
}

impl DataObject {
    #[allow(clippy::too_many_lines)]
    /// Retrieves a `DataObject` by its ID.
    ///
    /// # Arguments
    ///
    /// * `ctx` - The application context containing the database connection
    /// * `id` - The UUID of the teach record to retrieve
    /// * `_with_has_many_relationships` - Flag to include has-many relationships (currently unused)
    ///
    /// # Returns
    ///
    /// * `Ok(Some(DataObject))` - If the teach record is found
    /// * `Ok(None)` - If no teach record with the given ID exists
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// * There is a database connection error
    /// * There is an error executing the database queries
    pub async fn get_by_id(
        ctx: &AppContext,
        id: Uuid,
        _with_has_many_relationships: bool,
    ) -> Result<Option<Self>> {
        let teach = AcademicCampaignTransactionTeach::Entity::find_by_id(id)
            .filter(AcademicCampaignTransactionTeach::Column::DeletedAt.is_null())
            .one(&ctx.db)
            .await?;
        if let Some(teach) = teach {
            let encounter_category =
                AcademicCampaignnReferenceEncounterCategory::Entity::find_by_id(
                    teach.encounter_category_id,
                )
                .filter(AcademicCampaignnReferenceEncounterCategory::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;
            let scope = AcademicCampaignReferenceScope::Entity::find_by_id(teach.scope_id)
                .filter(AcademicCampaignReferenceScope::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;
            let activity =
                AcademicCampaignTransactionActivity::Entity::find_by_id(teach.activity_id)
                    .filter(AcademicCampaignTransactionActivity::Column::DeletedAt.is_null())
                    .one(&ctx.db)
                    .await?;
            let course = AcademicCourseMasterCourse::Entity::find_by_id(teach.course_id)
                .filter(AcademicCourseMasterCourse::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;
            let teach_decree =
                AcademicCampaignTransactionTeachDecree::Entity::find_by_id(teach.teach_decree_id)
                    .filter(AcademicCampaignTransactionTeachDecree::Column::DeletedAt.is_null())
                    .one(&ctx.db)
                    .await?;
            let class_code =
                AcademicCampaignTransactionClassCode::Entity::find_by_id(teach.class_code_id)
                    .filter(AcademicCampaignTransactionClassCode::Column::DeletedAt.is_null())
                    .one(&ctx.db)
                    .await?;
            Ok(Some(Self {
                teach,
                encounter_category,
                scope,
                activity,
                course,
                teach_decree,
                class_code,
            }))
        } else {
            Ok(None)
        }
    }
}

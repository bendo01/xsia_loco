use crate::models::literate::categories::_entities::categories as LiterateCategory;
use crate::models::literate::educations::_entities::educations as LiterateEducation;
use crate::models::literate::groups::_entities::groups as LiterateGroup;
use crate::models::literate::levels::_entities::levels as LiterateLevel;
use crate::models::literate::varieties::_entities::varieties as LiterateVariety;
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(clippy::too_many_lines)]
pub struct LiterateEducationDataObject {
    pub education: LiterateEducation::Model,
    pub level: Option<LiterateLevel::Model>,
    pub group: Option<LiterateGroup::Model>,
    pub category: Option<LiterateCategory::Model>,
    pub variety: Option<LiterateVariety::Model>,
}

impl LiterateEducationDataObject {
    /// Retrieves an attend type by ID if it exists and is not deleted
    ///
    /// # Arguments
    /// * `ctx` - The application context containing the database connection
    /// * `id` - The UUID of the attend type to retrieve
    ///
    /// # Returns
    /// An optional `DataObject` wrapped in a `Result` - returns `None` if no matching record is found
    ///
    /// # Errors
    /// This function will return an error if there's a database connection issue or query execution problem
    #[allow(clippy::too_many_lines)]
    pub async fn get_by_id(ctx: &AppContext, id: Uuid) -> Result<Option<Self>> {
        let education = LiterateEducation::Entity::find_by_id(id)
            .filter(LiterateEducation::Column::DeletedAt.is_null())
            .one(&ctx.db)
            .await?;
        if let Some(education) = education {
            let level = education
                .find_related(LiterateLevel::Entity)
                .filter(LiterateLevel::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            let group = education
                .find_related(LiterateGroup::Entity)
                .filter(LiterateGroup::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            let category = education
                .find_related(LiterateCategory::Entity)
                .filter(LiterateCategory::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            let variety = education
                .find_related(LiterateVariety::Entity)
                .filter(LiterateVariety::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            Ok(Some(Self {
                education,
                level,
                group,
                category,
                variety,
            }))
        } else {
            Ok(None)
        }
    }
}

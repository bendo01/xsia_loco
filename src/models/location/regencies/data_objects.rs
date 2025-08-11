use crate::models::location::provinces::_entities::provinces as LocationProvince;
use crate::models::location::regencies::_entities::regencies as LocationRegency;
use crate::models::location::regency_types::_entities::regency_types as LocationRegencyType;
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(clippy::too_many_lines)]
pub struct LocationRegencyDataObject {
    pub regency: LocationRegency::Model,
    pub province: Option<LocationProvince::Model>,
    pub regency_type: Option<LocationRegencyType::Model>,
}

impl LocationRegencyDataObject {
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
        let regency = LocationRegency::Entity::find_by_id(id)
            .filter(LocationRegency::Column::DeletedAt.is_null())
            .one(&ctx.db)
            .await?;
        if let Some(regency) = regency {
            let province = regency
                .find_related(LocationProvince::Entity)
                .filter(LocationProvince::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            let regency_type = regency
                .find_related(LocationRegencyType::Entity)
                .filter(LocationRegencyType::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            Ok(Some(Self {
                regency,
                province,
                regency_type,
            }))
        } else {
            Ok(None)
        }
    }
}

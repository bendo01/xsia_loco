use crate::models::location::regencies::_entities::regencies as LocationRegency;
use crate::models::location::sub_districts::_entities::sub_districts as LocationSubDistrict;
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(clippy::too_many_lines)]
pub struct LocationSubDistrictDataObject {
    pub sub_district: LocationSubDistrict::Model,
    pub regency: Option<LocationRegency::Model>,
}

impl LocationSubDistrictDataObject {
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
        let sub_district = LocationSubDistrict::Entity::find_by_id(id)
            .filter(LocationSubDistrict::Column::DeletedAt.is_null())
            .one(&ctx.db)
            .await?;
        if let Some(sub_district) = sub_district {
            let regency = sub_district
                .find_related(LocationRegency::Entity)
                .filter(LocationRegency::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;
            Ok(Some(Self {
                sub_district,
                regency,
            }))
        } else {
            Ok(None)
        }
    }
}

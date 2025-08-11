use crate::models::location::countries::_entities::countries as LocationCountry;
use crate::models::location::provinces::_entities::provinces as LocationProvince;
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(clippy::too_many_lines)]
pub struct LocationProvinceDataObject {
    pub province: LocationProvince::Model,
    pub country: Option<LocationCountry::Model>,
}

impl LocationProvinceDataObject {
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
        let province = LocationProvince::Entity::find_by_id(id)
            .filter(LocationProvince::Column::DeletedAt.is_null())
            .one(&ctx.db)
            .await?;
        if let Some(province) = province {
            let country = province
                .find_related(LocationCountry::Entity)
                .filter(LocationCountry::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;
            Ok(Some(Self { province, country }))
        } else {
            Ok(None)
        }
    }
}

use crate::models::location::sub_districts::_entities::sub_districts as LocationSubDistrict;
use crate::models::location::villages::_entities::villages as LocationVillage;
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(clippy::too_many_lines)]
pub struct LocationVillageDataObject {
    pub village: LocationVillage::Model,
    pub sub_district: Option<LocationSubDistrict::Model>,
}

impl LocationVillageDataObject {
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
        let village = LocationVillage::Entity::find_by_id(id)
            .filter(LocationVillage::Column::DeletedAt.is_null())
            .one(&ctx.db)
            .await?;
        if let Some(village) = village {
            let sub_district = village
                .find_related(LocationSubDistrict::Entity)
                .filter(LocationSubDistrict::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;
            Ok(Some(Self {
                village,
                sub_district,
            }))
        } else {
            Ok(None)
        }
    }
}

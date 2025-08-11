use crate::models::location::continents::_entities::continents as LocationContinent;
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(clippy::too_many_lines)]
pub struct LocationContinentDataObject {
    pub continent: LocationContinent::Model,
}

impl LocationContinentDataObject {
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
        let continent = LocationContinent::Entity::find_by_id(id)
            .filter(LocationContinent::Column::DeletedAt.is_null())
            .one(&ctx.db)
            .await?;
        // if let Some(continent) = continent {
        //     Ok(Some(Self { continent }))
        // } else {
        //     Ok(None)
        // }
        continent.map_or_else(|| Ok(None), |continent| Ok(Some(Self { continent })))
    }
}

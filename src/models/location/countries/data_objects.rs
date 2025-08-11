use crate::models::location::continents::_entities::continents as LocationContinent;
use crate::models::location::countries::_entities::countries as LocationCountry;
use crate::models::location::regions::_entities::regions as LocationRegion;
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(clippy::too_many_lines)]
pub struct LocationCountryDataObject {
    pub country: LocationCountry::Model,
    pub continent: Option<LocationContinent::Model>,
    pub region: Option<LocationRegion::Model>,
}

impl LocationCountryDataObject {
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
        let country = LocationCountry::Entity::find_by_id(id)
            .filter(LocationCountry::Column::DeletedAt.is_null())
            .one(&ctx.db)
            .await?;
        if let Some(country) = country {
            let continent = country
                .find_related(LocationContinent::Entity)
                .filter(LocationContinent::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            let region = country
                .find_related(LocationRegion::Entity)
                .filter(LocationRegion::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            Ok(Some(Self {
                country,
                continent,
                region,
            }))
        } else {
            Ok(None)
        }
    }
}

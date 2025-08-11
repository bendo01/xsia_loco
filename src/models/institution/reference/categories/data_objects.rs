use crate::models::institution::reference::categories::_entities::categories as ReferenceModel;
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataObject {
    pub model: ReferenceModel::Model,
}

impl DataObject {
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
    pub async fn get_by_id(ctx: &AppContext, id: Uuid) -> Result<Option<Self>> {
        let model = ReferenceModel::Entity::find_by_id(id)
            .filter(ReferenceModel::Column::DeletedAt.is_null())
            .one(&ctx.db)
            .await?;

        model.map_or_else(|| Ok(None), |model| Ok(Some(Self { model })))
    }
}

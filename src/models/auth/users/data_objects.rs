use crate::models::auth::roles::_entities::roles as AuthRole;
use crate::models::auth::users::_entities::users as ReferenceModel;
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ReturnedUser {
    pub email: String,
    pub name: String,
    pub current_role_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataObject {
    pub user: ReturnedUser,
    pub role: Option<AuthRole::Model>,
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
        let user = ReferenceModel::Entity::find_by_id(id).one(&ctx.db).await?;

        match user {
            Some(user) => {
                // Then fetch roles separately
                let role = AuthRole::Entity::find()
                    .filter(AuthRole::Column::Id.eq(user.current_role_id))
                    .one(&ctx.db)
                    .await?;

                Ok(Some(Self {
                    user: ReturnedUser {
                        email: user.email,
                        name: user.name,
                        current_role_id: user.current_role_id,
                    },
                    role,
                }))
            }
            None => Ok(None),
        }
    }
}

use crate::models::contact::master::websites::_entities::websites as ContactMasterWebsite;
use crate::models::contact::reference::website_types::_entities::website_types as ContactReferenceWebsiteType;
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(clippy::too_many_lines)]
pub struct ContactWebsiteDataObject {
    pub website: ContactMasterWebsite::Model,
    pub website_type: Option<ContactReferenceWebsiteType::Model>,
}

impl ContactWebsiteDataObject {
    #[allow(clippy::missing_errors_doc)]
    #[allow(clippy::too_many_lines)]
    pub async fn get_by_id(ctx: &AppContext, id: Uuid) -> Result<Option<Self>> {
        let website = ContactMasterWebsite::Entity::find_by_id(id)
            .filter(ContactMasterWebsite::Column::DeletedAt.is_null())
            .one(&ctx.db)
            .await?;
        if let Some(website) = website {
            let website_type = website
                .find_related(ContactReferenceWebsiteType::Entity)
                .filter(ContactReferenceWebsiteType::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;
            Ok(Some(Self {
                website,
                website_type,
            }))
        } else {
            Ok(None)
        }
    }
}

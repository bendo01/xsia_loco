use crate::models::contact::master::phones::_entities::phones as ContactMasterPhone;
use crate::models::contact::reference::phone_types::_entities::phone_types as ContactReferencePhoneType;
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(clippy::too_many_lines)]
pub struct ContactPhoneDataObject {
    pub phone: ContactMasterPhone::Model,
    pub phone_type: Option<ContactReferencePhoneType::Model>,
}

impl ContactPhoneDataObject {
    #[allow(clippy::missing_errors_doc)]
    #[allow(clippy::too_many_lines)]
    pub async fn get_by_id(ctx: &AppContext, id: Uuid) -> Result<Option<Self>> {
        let phone = ContactMasterPhone::Entity::find_by_id(id)
            .filter(ContactMasterPhone::Column::DeletedAt.is_null())
            .one(&ctx.db)
            .await?;
        if let Some(phone) = phone {
            let phone_type = phone
                .find_related(ContactReferencePhoneType::Entity)
                .filter(ContactReferencePhoneType::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;
            Ok(Some(Self { phone, phone_type }))
        } else {
            Ok(None)
        }
    }
}

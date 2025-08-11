use crate::models::contact::master::electronic_mails::_entities::electronic_mails as ContactMasterElectronicMail;
use crate::models::contact::reference::electronic_mail_types::_entities::electronic_mail_types as ContactReferenceElectronicMailType;
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(clippy::too_many_lines)]
pub struct ContactElectronicMailDataObject {
    pub electronic_mail: ContactMasterElectronicMail::Model,
    pub electronic_mail_type: Option<ContactReferenceElectronicMailType::Model>,
}

impl ContactElectronicMailDataObject {
    #[allow(clippy::missing_errors_doc)]
    #[allow(clippy::too_many_lines)]
    pub async fn get_by_id(ctx: &AppContext, id: Uuid) -> Result<Option<Self>> {
        let electronic_mail = ContactMasterElectronicMail::Entity::find_by_id(id)
            .filter(ContactMasterElectronicMail::Column::DeletedAt.is_null())
            .one(&ctx.db)
            .await?;
        if let Some(electronic_mail) = electronic_mail {
            let electronic_mail_type = electronic_mail
                .find_related(ContactReferenceElectronicMailType::Entity)
                .filter(ContactReferenceElectronicMailType::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;
            Ok(Some(Self {
                electronic_mail,
                electronic_mail_type,
            }))
        } else {
            Ok(None)
        }
    }
}

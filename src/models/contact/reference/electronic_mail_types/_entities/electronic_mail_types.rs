use crate::models::contact::master::electronic_mails::_entities::electronic_mails as ContactMasterElectronicMail;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(
    schema_name = "contact_reference",
    table_name = "electronic_mail_types"
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub code: i32,
    pub alphabet_code: String,
    pub name: String,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    ElectronicMails,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::ElectronicMails => Entity::has_many(ContactMasterElectronicMail::Entity).into(),
        }
    }
}

impl Related<ContactMasterElectronicMail::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ElectronicMails.def()
    }
}

use crate::models::contact::reference::electronic_mail_types::_entities::electronic_mail_types as ElectronicMailType;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "contact_master", table_name = "electronic_mails")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub email_address: String,
    pub electronic_mail_type_id: Option<Uuid>,
    pub electronic_mailable_id: Uuid,
    pub electronic_mailable_type: String,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    ElectronicMailType,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::ElectronicMailType => Entity::belongs_to(ElectronicMailType::Entity)
                .from(Column::ElectronicMailTypeId)
                .to(ElectronicMailType::Column::Id)
                .into(),
        }
    }
}

impl Related<ElectronicMailType::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ElectronicMailType.def()
    }
}

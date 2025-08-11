use crate::models::contact::reference::phone_types::_entities::phone_types as PhoneType;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "contact_master", table_name = "phones")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub phone_number: String,
    pub phone_type_id: Uuid,
    pub phoneable_id: Uuid,
    pub phoneable_type: String,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    PhoneType,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::PhoneType => Entity::belongs_to(PhoneType::Entity)
                .from(Column::PhoneTypeId)
                .to(PhoneType::Column::Id)
                .into(),
        }
    }
}

impl Related<PhoneType::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PhoneType.def()
    }
}

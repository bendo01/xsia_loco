use crate::models::contact::reference::website_types::_entities::website_types as WebsiteType;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "contact_master", table_name = "websites")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub website_url: String,
    pub website_type_id: Option<Uuid>,
    pub websiteable_id: Uuid,
    pub websiteable_type: String,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    WebsiteType,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::WebsiteType => Entity::belongs_to(WebsiteType::Entity)
                .from(Column::WebsiteTypeId)
                .to(WebsiteType::Column::Id)
                .into(),
        }
    }
}

impl Related<WebsiteType::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::WebsiteType.def()
    }
}

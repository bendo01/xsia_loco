use crate::models::person::master::family_card_members::_entities::family_card_members as PersonMasterFamilyCardMember;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "person_reference", table_name = "relative_types")]
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
    FamilyCardMembers,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::FamilyCardMembers => Entity::has_many(PersonMasterFamilyCardMember::Entity)
                .from(Column::Id)
                .to(PersonMasterFamilyCardMember::Column::RelativeTypeId)
                .into(),
        }
    }
}

impl Related<PersonMasterFamilyCardMember::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FamilyCardMembers.def()
    }
}

use crate::models::auth::users::_entities::users as AuthUsers;
use crate::models::institution::reference::position_types::_entities::position_types as InstitutionReferencePositionType;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "auth", table_name = "user_position_type")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub position_type_id: Uuid,
    pub user_id: Uuid,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    User,
    PositionType,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::User => Entity::belongs_to(AuthUsers::Entity)
                .from(Column::UserId)
                .to(AuthUsers::Column::Id)
                .into(),
            Self::PositionType => Entity::belongs_to(InstitutionReferencePositionType::Entity)
                .from(Column::PositionTypeId)
                .to(InstitutionReferencePositionType::Column::Id)
                .into(),
        }
    }
}

impl Related<AuthUsers::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<InstitutionReferencePositionType::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PositionType.def()
    }
}

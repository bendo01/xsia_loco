use crate::models::auth::permissions::_entities::permissions as AuthPermissions;
use crate::models::auth::users::_entities::users as AuthUsers;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "auth", table_name = "permission_user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub permission_id: Uuid,
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
    Permission,
    User,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Permission => Entity::belongs_to(AuthPermissions::Entity)
                .from(Column::PermissionId)
                .to(AuthPermissions::Column::Id)
                .into(),
            Self::User => Entity::belongs_to(AuthUsers::Entity)
                .from(Column::UserId)
                .to(AuthUsers::Column::Id)
                .into(),
        }
    }
}

impl Related<AuthPermissions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Permission.def()
    }
}

impl Related<AuthUsers::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

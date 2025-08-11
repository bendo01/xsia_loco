use crate::models::auth::permission_user::_entities::permission_user as AuthPermissionUser;
use crate::models::auth::permissions::_entities::permissions as AuthPermission;
use crate::models::auth::roles::_entities::roles as AuthRole;
use crate::models::auth::user_position_type::_entities::user_position_type as AuthUserPositionType;
use crate::models::institution::reference::position_types::_entities::position_types as InstitutionReferencePositionType;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "auth", table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub pid: Uuid,
    #[sea_orm(unique)]
    pub email: String,
    pub password: String,
    #[sea_orm(unique)]
    pub api_key: String,
    pub name: String,
    // pub remember_token: String,
    #[sea_orm(default_value = "00000000-0000-0000-0000-000000000000")]
    pub individual_id: Uuid,
    #[sea_orm(default_value = false)]
    pub is_active: bool,
    #[sea_orm(default_value = "00000000-0000-0000-0000-000000000000")]
    pub current_role_id: Uuid,
    pub reset_token: Option<String>,
    pub reset_sent_at: Option<DateTime>,
    pub email_verification_token: Option<String>,
    pub email_verification_sent_at: Option<DateTime>,
    pub email_verified_at: Option<DateTime>,
    pub magic_link_token: Option<String>,
    pub magic_link_expiration: Option<DateTime>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Roles,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Roles => Entity::has_many(AuthRole::Entity)
                .from(Column::Id)
                .to(AuthRole::Column::UserId)
                .into(),
        }
    }
}

impl Related<AuthRole::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Roles.def()
    }
}

impl Related<AuthPermission::Entity> for Entity {
    fn to() -> RelationDef {
        AuthPermissionUser::Relation::Permission.def()
    }

    fn via() -> Option<RelationDef> {
        Some(AuthPermissionUser::Relation::User.def().rev())
    }
}

impl Related<InstitutionReferencePositionType::Entity> for Entity {
    fn to() -> RelationDef {
        AuthUserPositionType::Relation::PositionType.def()
    }

    fn via() -> Option<RelationDef> {
        Some(AuthUserPositionType::Relation::User.def().rev())
    }
}

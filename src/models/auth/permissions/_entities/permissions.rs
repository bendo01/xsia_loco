use crate::models::auth::permission_position_type::_entities::permission_position_type as AuthPermissionPositionType;
use crate::models::auth::permission_user::_entities::permission_user as AuthPermissionUser;
use crate::models::auth::users::_entities::users as AuthUser;
use crate::models::institution::reference::position_types::_entities::position_types as InstitutionReferencePositionType;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "auth", table_name = "permissions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub is_open: bool,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    // PermissionUser,
    // PermissionPositionType,
}

impl RelationTrait for Relation {
    // fn def(&self) -> RelationDef {
    //     match self {
    //         Self::PermissionUser => Entity::has_many(AuthPermissionUser::Entity)
    //             .from(Column::Id)
    //             .to(AuthPermissionUser::Column::PermissionId)
    //             .into(),
    //         Self::PermissionPositionType => Entity::has_many(AuthPermissionPositionType::Entity)
    //             .from(Column::Id)
    //             .to(AuthPermissionPositionType::Column::PermissionId)
    //             .into(),
    //     }
    // }

    fn def(&self) -> RelationDef {
        panic!()
    }
}

// impl Related<AuthPermissionUser::Entity> for Entity {
//     fn to() -> RelationDef {
//         Relation::PermissionUser.def()
//     }
// }

// impl Related<AuthPermissionPositionType::Entity> for Entity {
//     fn to() -> RelationDef {
//         Relation::PermissionPositionType.def()
//     }
// }

impl Related<AuthUser::Entity> for Entity {
    fn to() -> RelationDef {
        AuthPermissionUser::Relation::User.def()
    }

    fn via() -> Option<RelationDef> {
        Some(AuthPermissionUser::Relation::Permission.def().rev())
    }
}

impl Related<InstitutionReferencePositionType::Entity> for Entity {
    fn to() -> RelationDef {
        AuthPermissionPositionType::Relation::PositionType.def()
    }

    fn via() -> Option<RelationDef> {
        Some(AuthPermissionPositionType::Relation::Permission.def().rev())
    }
}

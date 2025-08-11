use crate::models::auth::permission_position_type::_entities::permission_position_type as AuthPermissionPositionType;
use crate::models::auth::permissions::_entities::permissions as AuthPermission;
use crate::models::auth::user_position_type::_entities::user_position_type as AuthUserPositionType;
use crate::models::auth::users::_entities::users as AuthUser;
use crate::models::institution::master::staffes::_entities::staffes as InstitutionMasterStaff;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "institution_reference", table_name = "position_types")]
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
    Staffes,
    // UserPositionType,
    // PermissionPositionType,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Staffes => Entity::has_many(InstitutionMasterStaff::Entity)
                .from(Column::Id)
                .to(InstitutionMasterStaff::Column::PositionTypeId)
                .into(),
            // Self::UserPositionType => Entity::has_many(AuthUserPositionType::Entity)
            //     .from(Column::Id)
            //     .to(AuthUserPositionType::Column::PositionTypeId)
            //     .into(),
            // Self::PermissionPositionType => Entity::has_many(AuthPermissionPositionType::Entity)
            //     .from(Column::Id)
            //     .to(AuthPermissionPositionType::Column::PositionTypeId)
            //     .into(),
        }
    }
}

// `Related` trait has to be implemented by hand
impl Related<InstitutionMasterStaff::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Staffes.def()
    }
}

// // `Related` trait has to be implemented by hand
// impl Related<AuthUserPositionType::Entity> for Entity {
//     fn to() -> RelationDef {
//         Relation::UserPositionType.def()
//     }
// }

// // `Related` trait has to be implemented by hand
// impl Related<AuthPermissionPositionType::Entity> for Entity {
//     fn to() -> RelationDef {
//         Relation::PermissionPositionType.def()
//     }
// }

impl Related<AuthPermission::Entity> for Entity {
    fn to() -> RelationDef {
        AuthPermissionPositionType::Relation::Permission.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            AuthPermissionPositionType::Relation::PositionType
                .def()
                .rev(),
        )
    }
}

impl Related<AuthUser::Entity> for Entity {
    fn to() -> RelationDef {
        AuthUserPositionType::Relation::User.def()
    }

    fn via() -> Option<RelationDef> {
        Some(AuthUserPositionType::Relation::PositionType.def().rev())
    }
}

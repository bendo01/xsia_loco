use crate::models::auth::users::_entities::users as AuthUser;
use crate::models::institution::reference::position_types::_entities::position_types as InstitutionReferencePositionType;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "auth", table_name = "roles")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub name: String,
    pub user_id: Uuid,
    pub position_type_id: Uuid,
    pub roleable_id: Uuid,
    pub roleable_type: String,
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
            Self::User => Entity::belongs_to(AuthUser::Entity)
                .from(Column::UserId)
                .to(AuthUser::Column::Id)
                .into(),
            Self::PositionType => Entity::belongs_to(InstitutionReferencePositionType::Entity)
                .from(Column::UserId)
                .to(InstitutionReferencePositionType::Column::Id)
                .into(),
        }
    }
}

// `Related` trait has to be implemented by hand
impl Related<AuthUser::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

// `Related` trait has to be implemented by hand
impl Related<InstitutionReferencePositionType::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PositionType.def()
    }
}

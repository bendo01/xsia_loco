use crate::models::academic::lecturer::transaction::academic_groups::_entities::academic_groups as AcademicLecturerTransactionAcademicGroup;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "academic_lecturer_reference", table_name = "groups")]
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
    AcademicGroups,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::AcademicGroups => {
                Entity::has_many(AcademicLecturerTransactionAcademicGroup::Entity)
                    .from(Column::Id)
                    .to(AcademicLecturerTransactionAcademicGroup::Column::GroupId)
                    .into()
            }
        }
    }
}

impl Related<AcademicLecturerTransactionAcademicGroup::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AcademicGroups.def()
    }
}

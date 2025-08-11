use crate::models::academic::lecturer::master::lecturers::_entities::lecturers as AcademicLecturerMasterLecturer;
use crate::models::academic::lecturer::reference::groups::_entities::groups as AcademicLecturerReferenceGroup;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(
    schema_name = "academic_lecturer_transaction",
    table_name = "academic_groups"
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub decree_number: Option<String>,
    pub decree_date: Option<Date>,
    pub lecturer_id: Uuid,
    pub group_id: Uuid,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub start_date: Option<Date>,
    pub end_date: Option<Date>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Group,
    Lecturer,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Group => Entity::belongs_to(AcademicLecturerReferenceGroup::Entity)
                .from(Column::GroupId)
                .to(AcademicLecturerReferenceGroup::Column::Id)
                .into(),
            Self::Lecturer => Entity::belongs_to(AcademicLecturerMasterLecturer::Entity)
                .from(Column::LecturerId)
                .to(AcademicLecturerMasterLecturer::Column::Id)
                .into(),
        }
    }
}

impl Related<AcademicLecturerReferenceGroup::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Group.def()
    }
}

impl Related<AcademicLecturerMasterLecturer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Lecturer.def()
    }
}

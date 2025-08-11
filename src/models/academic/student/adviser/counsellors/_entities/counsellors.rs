use crate::models::academic::lecturer::master::lecturers::_entities::lecturers as AcademicLecturerMasterLecturer;
use crate::models::academic::student::adviser::decrees::_entities::decrees as AcademicStudentAdviserDecree;
use crate::models::academic::student::master::students::_entities::students as AcademicStudentMasterStudent;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "academic_student_adviser", table_name = "counsellors")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub decree_id: Uuid,
    pub student_id: Uuid,
    pub lecturer_id: Uuid,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Decree,
    Student,
    Lecturer,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Student => Entity::belongs_to(AcademicStudentMasterStudent::Entity)
                .from(Column::StudentId)
                .to(AcademicStudentMasterStudent::Column::Id)
                .into(),
            Self::Decree => Entity::belongs_to(AcademicStudentAdviserDecree::Entity)
                .from(Column::DecreeId)
                .to(AcademicStudentAdviserDecree::Column::Id)
                .into(),
            Self::Lecturer => Entity::belongs_to(AcademicLecturerMasterLecturer::Entity)
                .from(Column::LecturerId)
                .to(AcademicLecturerMasterLecturer::Column::Id)
                .into(),
        }
    }
}

impl Related<AcademicStudentAdviserDecree::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Decree.def()
    }
}

impl Related<AcademicLecturerMasterLecturer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Lecturer.def()
    }
}

impl Related<AcademicStudentMasterStudent::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Student.def()
    }
}

use crate::models::academic::student::master::students::_entities::students as AcademicStudentMasterStudent;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "academic_student_master", table_name = "images")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub student_id: Uuid,
    pub filename: String,
    pub dir: String,
    pub mimetype: Option<String>,
    pub size: Option<i64>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Student,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Student => Entity::belongs_to(AcademicStudentMasterStudent::Entity)
                .from(Column::StudentId)
                .to(AcademicStudentMasterStudent::Column::Id)
                .into(),
        }
    }
}

impl Related<AcademicStudentMasterStudent::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Student.def()
    }
}

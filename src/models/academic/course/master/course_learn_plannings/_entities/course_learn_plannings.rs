use crate::models::academic::course::master::courses::_entities::courses as AcademicCourseMasterCourse;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(
    schema_name = "academic_course_master",
    table_name = "course_learn_plannings"
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub code: i32,
    pub title: String,
    pub name: String,
    #[sea_orm(column_type = "Text")]
    pub decription_indonesian: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub decription_english: Option<String>,
    pub course_id: Uuid,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Course,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Course => Entity::belongs_to(AcademicCourseMasterCourse::Entity)
                .from(Column::CourseId)
                .to(AcademicCourseMasterCourse::Column::Id)
                .into(),
        }
    }
}

impl Related<AcademicCourseMasterCourse::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Course.def()
    }
}

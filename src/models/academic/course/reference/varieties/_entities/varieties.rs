use crate::models::academic::course::master::courses::_entities::courses as AcademicCourseMasterCourse;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "academic_course_reference", table_name = "varieties")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub code: i32,
    pub alphabet_code: String,
    pub name: String,
    pub curriculum_type_id: Option<Uuid>,
    pub start_effective_date: Option<Date>,
    pub end_effective_date: Option<Date>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Courses,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Courses => Entity::has_many(AcademicCourseMasterCourse::Entity)
                .from(Column::Id)
                .to(AcademicCourseMasterCourse::Column::VarietyId)
                .into(),
        }
    }
}

impl Related<AcademicCourseMasterCourse::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Courses.def()
    }
}

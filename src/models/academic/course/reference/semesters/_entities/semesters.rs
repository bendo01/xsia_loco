use crate::models::academic::course::master::curriculum_details::_entities::curriculum_details as AcademicCourseMasterCurriculumDetail;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "academic_course_reference", table_name = "semesters")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub code: i32,
    pub alphabet_code: String,
    pub name: String,
    #[sea_orm(default_value = false)]
    pub is_odd: bool,
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
    AcademicCourseMasterCurriculumDetails,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::AcademicCourseMasterCurriculumDetails => {
                Entity::has_many(AcademicCourseMasterCurriculumDetail::Entity)
                    .from(Column::Id)
                    .to(AcademicCourseMasterCurriculumDetail::Column::SemesterId)
                    .into()
            }
        }
    }
}

impl Related<AcademicCourseMasterCurriculumDetail::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AcademicCourseMasterCurriculumDetails.def()
    }
}

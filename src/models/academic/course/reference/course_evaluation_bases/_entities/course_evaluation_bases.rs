use crate::models::academic::course::master::course_evaluation_plannings::_entities::course_evaluation_plannings as AcademicCourseMasterCourseEvaluationPlanning;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(
    schema_name = "academic_course_reference",
    table_name = "course_evaluation_bases"
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub code: i32,
    pub evaluation_base: String,
    pub component_evaluation: String,
    pub start_effective_date: Option<Date>,
    pub end_effective_date: Option<Date>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    CourseEvaluationPlannings,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::CourseEvaluationPlannings => Entity::has_many(
                AcademicCourseMasterCourseEvaluationPlanning::Entity,
            )
            .from(Column::Id)
            .to(AcademicCourseMasterCourseEvaluationPlanning::Column::CourseEvaluationBaseId)
            .into(),
        }
    }
}

impl Related<AcademicCourseMasterCourseEvaluationPlanning::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CourseEvaluationPlannings.def()
    }
}

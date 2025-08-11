use crate::models::academic::course::reference::course_evaluation_bases::_entities::course_evaluation_bases as AcademicCourseReferenceCourseEvaluationBase;
use crate::models::academic::student::campaign::detail_activities::_entities::detail_activities as AcademicStudentCampaignDetailActivities;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(
    schema_name = "academic_student_campaign",
    table_name = "detail_activity_evaluation_components"
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(column_type = "Text", nullable)]
    pub name: Option<String>,
    pub detail_activity_id: Uuid,
    pub course_evaluation_planning_id: Uuid,
    #[sea_orm(column_type = "Float", nullable)]
    pub mark: Option<f32>,
    #[sea_orm(column_type = "Float", nullable)]
    pub percentage: Option<f32>,
    #[sea_orm(column_type = "Float", nullable)]
    pub total: Option<f32>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub deleted_at: Option<DateTime>,
    pub code: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    DetailActivity,
    CourseEvaluationPlanning,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::DetailActivity => {
                Entity::belongs_to(AcademicStudentCampaignDetailActivities::Entity)
                    .from(Column::DetailActivityId)
                    .to(AcademicStudentCampaignDetailActivities::Column::Id)
                    .into()
            }
            Self::CourseEvaluationPlanning => {
                Entity::belongs_to(AcademicCourseReferenceCourseEvaluationBase::Entity)
                    .from(Column::CourseEvaluationPlanningId)
                    .to(AcademicCourseReferenceCourseEvaluationBase::Column::Id)
                    .into()
            }
        }
    }
}

impl Related<AcademicStudentCampaignDetailActivities::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DetailActivity.def()
    }
}

impl Related<AcademicCourseReferenceCourseEvaluationBase::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CourseEvaluationPlanning.def()
    }
}

use crate::models::academic::campaign::transaction::grades::_entities::grades as AcademicCampaignTransactionGrade;
use crate::models::academic::campaign::transaction::teaches::_entities::teaches as AcademicCampaignTransactionTeach;
use crate::models::academic::course::master::courses::_entities::courses as AcademicCourseMasterCourse;
use crate::models::academic::student::campaign::activities::_entities::activities as AcademicStudentCampaignActivity;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(
    schema_name = "academic_student_campaign",
    table_name = "detail_activities"
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: Option<String>,
    pub feeder_id: Uuid,
    pub feeder_grade_id: Uuid,
    pub curiculum_detail_sequence: i32,
    #[sea_orm(column_type = "Double", nullable)]
    pub mark: f64,
    #[sea_orm(column_type = "Double", nullable)]
    pub credit: f64,
    pub grade_id: Uuid,
    pub course_id: Uuid,
    pub activity_id: Uuid,
    pub teach_id: Uuid,
    pub is_lock: bool,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Grade,
    Course,
    Activity,
    Teach,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Activity => Entity::belongs_to(AcademicStudentCampaignActivity::Entity)
                .from(Column::ActivityId)
                .to(AcademicStudentCampaignActivity::Column::Id)
                .into(),
            Self::Grade => Entity::belongs_to(AcademicCampaignTransactionGrade::Entity)
                .from(Column::GradeId)
                .to(AcademicCampaignTransactionGrade::Column::Id)
                .into(),
            Self::Course => Entity::belongs_to(AcademicCourseMasterCourse::Entity)
                .from(Column::CourseId)
                .to(AcademicCourseMasterCourse::Column::Id)
                .into(),
            Self::Teach => Entity::belongs_to(AcademicCampaignTransactionTeach::Entity)
                .from(Column::TeachId)
                .to(AcademicCampaignTransactionTeach::Column::Id)
                .into(),
        }
    }
}

impl Related<AcademicStudentCampaignActivity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Activity.def()
    }
}

impl Related<AcademicCampaignTransactionGrade::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Grade.def()
    }
}

impl Related<AcademicCourseMasterCourse::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Course.def()
    }
}

impl Related<AcademicCampaignTransactionTeach::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Teach.def()
    }
}

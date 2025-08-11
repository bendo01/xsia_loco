use crate::models::academic::campaign::transaction::grades::_entities::grades as AcademicCampaignTransactionGrade;
use crate::models::academic::course::master::courses::_entities::courses as AcademicCourseMasterCourse;
use crate::models::academic::general::reference::academic_years::_entities::academic_years as AcademicGeneralReferenceAcademicYear;
use crate::models::academic::student::master::students::_entities::students as AcademicStudentMasterStudent;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "academic_student_campaign", table_name = "convertions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub academic_year_id: Option<Uuid>,
    pub student_id: Uuid,
    pub course_id: Uuid,
    pub grade_id: Uuid,
    pub transfer_code: String,
    pub transfer_name: String,
    #[sea_orm(column_type = "Double")]
    pub transfer_credit: f64,
    pub transfer_grade: String,
    pub is_lock: Option<DateTime>,
    pub feeder_id: Option<Uuid>,
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
    Course,
    Grade,
    AcademicYear,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Student => Entity::belongs_to(AcademicStudentMasterStudent::Entity)
                .from(Column::StudentId)
                .to(AcademicStudentMasterStudent::Column::Id)
                .into(),
            Self::Course => Entity::belongs_to(AcademicCourseMasterCourse::Entity)
                .from(Column::CourseId)
                .to(AcademicCourseMasterCourse::Column::Id)
                .into(),
            Self::Grade => Entity::belongs_to(AcademicCampaignTransactionGrade::Entity)
                .from(Column::GradeId)
                .to(AcademicCampaignTransactionGrade::Column::Id)
                .into(),
            Self::AcademicYear => Entity::belongs_to(AcademicGeneralReferenceAcademicYear::Entity)
                .from(Column::AcademicYearId)
                .to(AcademicGeneralReferenceAcademicYear::Column::Id)
                .into(),
        }
    }
}

impl Related<AcademicStudentMasterStudent::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Student.def()
    }
}

impl Related<AcademicCourseMasterCourse::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Course.def()
    }
}

impl Related<AcademicCampaignTransactionGrade::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Grade.def()
    }
}

impl Related<AcademicGeneralReferenceAcademicYear::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AcademicYear.def()
    }
}

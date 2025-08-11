use crate::models::academic::course::master::concentrations::_entities::concentrations as AcademicCourseMasterConcentration;
use crate::models::academic::course::master::courses::_entities::courses as AcademicCourseMasterCourse;
use crate::models::academic::course::master::curriculums::_entities::curriculums as AcademicCourseMasterCurriculum;
use crate::models::academic::course::reference::semesters::_entities::semesters as AcademicCourseReferenceSemester;
use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(
    schema_name = "academic_course_master",
    table_name = "curriculum_details"
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub code: i32,
    pub name: Option<String>,
    #[sea_orm(column_type = "Double")]
    pub credit: f64,
    pub curriculum_id: Uuid,
    pub semester_id: Uuid,
    pub course_id: Uuid,
    pub concentration_id: Uuid,
    pub is_convertable_to_mbkm: bool,
    pub is_convertable_to_prior_learning_recognition: bool,
    pub feeder_id: Uuid,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Curriculum,
    Semester,
    Course,
    Concentration,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Curriculum => Entity::belongs_to(AcademicCourseMasterCurriculum::Entity)
                .from(Column::CurriculumId)
                .to(AcademicCourseMasterCurriculum::Column::Id)
                .into(),
            Self::Semester => Entity::belongs_to(AcademicCourseReferenceSemester::Entity)
                .from(Column::SemesterId)
                .to(AcademicCourseReferenceSemester::Column::Id)
                .into(),
            Self::Course => Entity::belongs_to(AcademicCourseMasterCourse::Entity)
                .from(Column::CourseId)
                .to(AcademicCourseMasterCourse::Column::Id)
                .into(),
            Self::Concentration => Entity::belongs_to(AcademicCourseMasterConcentration::Entity)
                .from(Column::ConcentrationId)
                .to(AcademicCourseMasterConcentration::Column::Id)
                .into(),
        }
    }
}

impl Related<AcademicCourseMasterCurriculum::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Curriculum.def()
    }
}

impl Related<AcademicCourseReferenceSemester::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Semester.def()
    }
}

impl Related<AcademicCourseMasterCourse::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Course.def()
    }
}

impl Related<AcademicCourseMasterConcentration::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Concentration.def()
    }
}

use crate::models::academic::course::master::curriculum_details::_entities::curriculum_details as AcademicCourseMasterCurriculumDetail;
use crate::models::academic::course::reference::competences::_entities::competences as AcademicCourseReferenceCompetence;
use crate::models::academic::course::reference::groups::_entities::groups as AcademicCourseReferenceGroup;
use crate::models::academic::course::reference::varieties::_entities::varieties as AcademicCourseReferenceVariety;
use crate::models::institution::master::units::_entities::units as InstitutionMasterUnit;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "academic_course_master", table_name = "courses")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub code: String,
    pub name: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub implementation_method: Option<String>,
    #[sea_orm(column_type = "Float")]
    pub total_credit: f32,
    #[sea_orm(column_type = "Float")]
    pub lecture_credit: f32,
    #[sea_orm(column_type = "Float")]
    pub practice_credit: f32,
    #[sea_orm(column_type = "Float")]
    pub field_practice_credit: f32,
    #[sea_orm(column_type = "Float")]
    pub simulation_credit: f32,
    pub has_unit: bool,
    pub has_syllabus: bool,
    pub has_material: bool,
    pub has_practice: bool,
    pub has_dictation: bool,
    pub group_id: Option<Uuid>,
    pub variety_id: Uuid,
    pub unit_id: Uuid,
    pub competence_id: Option<Uuid>,
    pub feeder_course_group_id: Option<Uuid>,
    pub feeder_course_type_id: Option<Uuid>,
    pub feeder_course_id: Option<Uuid>,
    pub start_date: Option<Date>,
    pub end_date: Option<Date>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Group,
    Variety,
    Unit,
    Competence,
    CurriculumDetails,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Group => Entity::belongs_to(AcademicCourseReferenceGroup::Entity)
                .from(Column::GroupId)
                .to(AcademicCourseReferenceGroup::Column::Id)
                .into(),
            Self::Variety => Entity::belongs_to(AcademicCourseReferenceVariety::Entity)
                .from(Column::VarietyId)
                .to(AcademicCourseReferenceVariety::Column::Id)
                .into(),
            Self::Unit => Entity::belongs_to(InstitutionMasterUnit::Entity)
                .from(Column::UnitId)
                .to(InstitutionMasterUnit::Column::Id)
                .into(),
            Self::Competence => Entity::belongs_to(AcademicCourseReferenceCompetence::Entity)
                .from(Column::CompetenceId)
                .to(AcademicCourseReferenceCompetence::Column::Id)
                .into(),
            Self::CurriculumDetails => {
                Entity::has_many(AcademicCourseMasterCurriculumDetail::Entity)
                    .from(Column::Id)
                    .to(AcademicCourseMasterCurriculumDetail::Column::CourseId)
                    .into()
            }
        }
    }
}

impl Related<AcademicCourseReferenceGroup::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Group.def()
    }
}

impl Related<AcademicCourseReferenceVariety::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Variety.def()
    }
}

impl Related<InstitutionMasterUnit::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Unit.def()
    }
}

impl Related<AcademicCourseReferenceCompetence::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Competence.def()
    }
}

impl Related<AcademicCourseMasterCurriculumDetail::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CurriculumDetails.def()
    }
}

use crate::models::academic::course::master::curriculum_details::_entities::curriculum_details as AcademicCourseMasterCurriculumDetail;
use crate::models::academic::course::reference::curriculum_types::_entities::curriculum_types as AcademicCourseReferenceCurriculumType;
use crate::models::academic::general::reference::academic_years::_entities::academic_years as AcademicGeneralReferenceAcademicYear;
use crate::models::institution::master::units::_entities::units as InstitutionMasterUnit;
use crate::models::academic::prior_learning_recognition::transaction::recognitions::_entities::recognitions as AcademicPriorLearningRecognitionTransactionRecognition;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "academic_course_master", table_name = "curriculums")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub unit_id: Uuid,
    pub academic_year_id: Uuid,
    pub curriculum_type_id: Uuid,
    #[sea_orm(column_type = "Float")]
    pub total_credit: f32,
    #[sea_orm(column_type = "Float")]
    pub mandatory_course_credit: f32,
    #[sea_orm(column_type = "Float")]
    pub optional_course_credit: f32,
    pub feeder_id: Option<Uuid>,
    pub is_active: bool,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub start_date: Option<Date>,
    pub end_date: Option<Date>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    AcademicYear,
    Unit,
    CurriculumType,
    CurriculumDetails,
    Recognitions,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::AcademicYear => Entity::belongs_to(AcademicGeneralReferenceAcademicYear::Entity)
                .from(Column::AcademicYearId)
                .to(AcademicGeneralReferenceAcademicYear::Column::Id)
                .into(),
            Self::Unit => Entity::belongs_to(InstitutionMasterUnit::Entity)
                .from(Column::UnitId)
                .to(InstitutionMasterUnit::Column::Id)
                .into(),
            Self::CurriculumType => {
                Entity::belongs_to(AcademicCourseReferenceCurriculumType::Entity)
                    .from(Column::CurriculumTypeId)
                    .to(AcademicCourseReferenceCurriculumType::Column::Id)
                    .into()
            }
            Self::CurriculumDetails => {
                Entity::has_many(AcademicCourseMasterCurriculumDetail::Entity)
                    .from(Column::Id)
                    .to(AcademicCourseMasterCurriculumDetail::Column::CurriculumId)
                    .into()
            }
            Self::Recognitions => {
                Entity::has_many(AcademicPriorLearningRecognitionTransactionRecognition::Entity)
                    .from(Column::Id)
                    .to(AcademicPriorLearningRecognitionTransactionRecognition::Column::CurriculumId)
                    .into()
            }
        }
    }
}

impl Related<AcademicGeneralReferenceAcademicYear::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AcademicYear.def()
    }
}

impl Related<InstitutionMasterUnit::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Unit.def()
    }
}

impl Related<AcademicCourseReferenceCurriculumType::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CurriculumType.def()
    }
}

impl Related<AcademicCourseMasterCurriculumDetail::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CurriculumDetails.def()
    }
}

impl Related<AcademicPriorLearningRecognitionTransactionRecognition::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Recognitions.def()
    }
}

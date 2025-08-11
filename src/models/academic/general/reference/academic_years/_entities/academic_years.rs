use crate::models::academic::campaign::transaction::activities::_entities::activities as AcademicGeneralReferenceActivity;
use crate::models::academic::campaign::transaction::calendars::_entities::calendars as AcademicGeneralReferenceCalendar;
use crate::models::academic::candidate::master::candidates::_entities::candidates as AcademicCandidateMasterCandidate;
use crate::models::academic::course::master::curriculums::_entities::curriculums as AcademicCourseMasterCurriculum;
use crate::models::academic::general::reference::academic_year_categories::_entities::academic_year_categories as AcademicGeneralReferenceAcademicYearCategory;
use crate::models::academic::student::campaign::convertions::_entities::convertions as AcademicStudentCampaignConversion;
use crate::models::academic::student::master::students::_entities::students as AcademicStudentMasterStudent;
use crate::models::academic::survey::master::bundles::_entities::bundles as AcademicSurveyMasterBundle;
use crate::models::institution::master::institutions::_entities::institutions as InstitutionMasterInstitution;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(
    schema_name = "academic_general_reference",
    table_name = "academic_years"
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub code: i32,
    pub year: i32,
    pub name: String,
    pub feeder_name: String,
    pub academic_year_category_id: Uuid,
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
    AcademicYearCategory,
    AcademicCampaignTransactionActivities,
    AcademicCampaignTransactionCalendars,
    AcademicCandidateMasterCandidates,
    AcademicCourseMasterCurriculums,
    AcademicStudentCampaignConversions,
    AcademicStudentMasterStudents,
    AcademicSurveyMasterBundles,
    InstitutionMasterInstitutions,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::AcademicYearCategory => {
                Entity::belongs_to(AcademicGeneralReferenceAcademicYearCategory::Entity)
                    .from(Column::AcademicYearCategoryId)
                    .to(AcademicGeneralReferenceAcademicYearCategory::Column::Id)
                    .into()
            }
            Self::AcademicCampaignTransactionActivities => {
                Entity::has_many(AcademicGeneralReferenceActivity::Entity)
                    .from(Column::Id)
                    .to(AcademicGeneralReferenceActivity::Column::AcademicYearId)
                    .into()
            }
            Self::AcademicCampaignTransactionCalendars => {
                Entity::has_many(AcademicGeneralReferenceCalendar::Entity)
                    .from(Column::Id)
                    .to(AcademicGeneralReferenceCalendar::Column::AcademicYearId)
                    .into()
            }
            Self::AcademicCandidateMasterCandidates => {
                Entity::has_many(AcademicCandidateMasterCandidate::Entity)
                    .from(Column::Id)
                    .to(AcademicCandidateMasterCandidate::Column::AcademicYearId)
                    .into()
            }
            Self::AcademicCourseMasterCurriculums => {
                Entity::has_many(AcademicCourseMasterCurriculum::Entity)
                    .from(Column::Id)
                    .to(AcademicCourseMasterCurriculum::Column::AcademicYearId)
                    .into()
            }
            Self::AcademicStudentCampaignConversions => {
                Entity::has_many(AcademicStudentCampaignConversion::Entity)
                    .from(Column::Id)
                    .to(AcademicStudentCampaignConversion::Column::AcademicYearId)
                    .into()
            }
            Self::AcademicStudentMasterStudents => {
                Entity::has_many(AcademicStudentMasterStudent::Entity)
                    .from(Column::Id)
                    .to(AcademicStudentMasterStudent::Column::AcademicYearId)
                    .into()
            }
            Self::AcademicSurveyMasterBundles => {
                Entity::has_many(AcademicSurveyMasterBundle::Entity)
                    .from(Column::Id)
                    .to(AcademicSurveyMasterBundle::Column::AcademicYearId)
                    .into()
            }
            Self::InstitutionMasterInstitutions => {
                Entity::has_many(InstitutionMasterInstitution::Entity)
                    .from(Column::Id)
                    .to(InstitutionMasterInstitution::Column::AcademicYearId)
                    .into()
            }
        }
    }
}

impl Related<AcademicGeneralReferenceAcademicYearCategory::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AcademicYearCategory.def()
    }
}

impl Related<AcademicGeneralReferenceActivity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AcademicCampaignTransactionActivities.def()
    }
}

impl Related<AcademicGeneralReferenceCalendar::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AcademicCampaignTransactionCalendars.def()
    }
}

impl Related<AcademicCandidateMasterCandidate::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AcademicCandidateMasterCandidates.def()
    }
}

impl Related<AcademicCourseMasterCurriculum::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AcademicCourseMasterCurriculums.def()
    }
}

impl Related<AcademicStudentCampaignConversion::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AcademicStudentCampaignConversions.def()
    }
}

impl Related<AcademicStudentMasterStudent::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AcademicStudentMasterStudents.def()
    }
}

use crate::models::academic::campaign::reference::encounter_categories::_entities::encounter_categories as AcademicCampaignReferenceEncounterCategory;
use crate::models::academic::campaign::reference::scopes::_entities::scopes as AcademicCampaignReferenceScope;
use crate::models::academic::campaign::transaction::activities::_entities::activities as AcademicCampaignTransactionActivity;
use crate::models::academic::campaign::transaction::class_codes::_entities::class_codes as AcademicCampaignTransactionClassCode;
use crate::models::academic::campaign::transaction::teach_decrees::_entities::teach_decrees as AcademicCampaignTransactionTeachDecree;
use crate::models::academic::course::master::courses::_entities::courses as AcademicCourseMasterCourse;
use crate::models::academic::course::master::curriculum_details::_entities::curriculum_details as AcademicCourseMasterCurriculumDetail;
use crate::models::academic::survey::transaction::conducts::_entities::conducts as AcademicSurveyTransactionConduct;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "academic_campaign_transaction", table_name = "teaches")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(column_type = "Text", nullable)]
    pub name: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
    pub start_date: Option<Date>,
    pub end_date: Option<Date>,
    pub practice_start_date: Option<Date>,
    pub practice_end_date: Option<Date>,
    pub is_lecturer_credit_sum_problem: bool,
    pub is_lock: bool,
    pub max_member: i32,
    pub class_code_id: Uuid,
    pub course_id: Uuid,
    pub activity_id: Uuid,
    pub scope_id: Uuid,
    pub curriculum_detail_id: Uuid,
    pub teach_decree_id: Uuid,
    pub encounter_category_id: Uuid,
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
    AcademicCampaignTransactionActivity,
    AcademicCourseMasterCurriculumDetail,
    AcademicCampaignReferenceEncounterCategory,
    AcademicCampaignTransactionTeachDecree,
    AcademicCourseMasterCourse,
    AcademicCampaignReferenceScope,
    AcademicSurveyTransactionConducts,
    AcademicCampaignTransactionClassCode,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::AcademicCourseMasterCourse => {
                Entity::belongs_to(AcademicCourseMasterCourse::Entity)
                    .from(Column::CourseId)
                    .to(AcademicCourseMasterCourse::Column::Id)
                    .into()
            }
            Self::AcademicCampaignTransactionActivity => {
                Entity::belongs_to(AcademicCampaignTransactionActivity::Entity)
                    .from(Column::ActivityId)
                    .to(AcademicCampaignTransactionActivity::Column::Id)
                    .into()
            }
            Self::AcademicCourseMasterCurriculumDetail => {
                Entity::belongs_to(AcademicCourseMasterCurriculumDetail::Entity)
                    .from(Column::CurriculumDetailId)
                    .to(AcademicCourseMasterCurriculumDetail::Column::Id)
                    .into()
            }
            Self::AcademicCampaignReferenceEncounterCategory => {
                Entity::belongs_to(AcademicCampaignReferenceEncounterCategory::Entity)
                    .from(Column::EncounterCategoryId)
                    .to(AcademicCampaignReferenceEncounterCategory::Column::Id)
                    .into()
            }
            Self::AcademicCampaignTransactionTeachDecree => {
                Entity::belongs_to(AcademicCampaignTransactionTeachDecree::Entity)
                    .from(Column::TeachDecreeId)
                    .to(AcademicCampaignTransactionTeachDecree::Column::Id)
                    .into()
            }
            Self::AcademicCampaignReferenceScope => {
                Entity::belongs_to(AcademicCampaignReferenceScope::Entity)
                    .from(Column::ScopeId)
                    .to(AcademicCampaignReferenceScope::Column::Id)
                    .into()
            }
            Self::AcademicCampaignTransactionClassCode => {
                Entity::belongs_to(AcademicCampaignTransactionClassCode::Entity)
                    .from(Column::ClassCodeId)
                    .to(AcademicCampaignTransactionClassCode::Column::Id)
                    .into()
            }
            Self::AcademicSurveyTransactionConducts => {
                Entity::has_many(AcademicSurveyTransactionConduct::Entity)
                    .from(Column::Id)
                    .to(AcademicSurveyTransactionConduct::Column::TeachId)
                    .into()
            }
        }
    }
}

impl Related<AcademicCourseMasterCourse::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AcademicCourseMasterCourse.def()
    }
}

impl Related<AcademicCampaignTransactionActivity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AcademicCampaignTransactionActivity.def()
    }
}

impl Related<AcademicCourseMasterCurriculumDetail::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AcademicCourseMasterCurriculumDetail.def()
    }
}

impl Related<AcademicCampaignReferenceEncounterCategory::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AcademicCampaignReferenceEncounterCategory.def()
    }
}

impl Related<AcademicCampaignTransactionTeachDecree::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AcademicCampaignTransactionTeachDecree.def()
    }
}

impl Related<AcademicCampaignReferenceScope::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AcademicCampaignReferenceScope.def()
    }
}

impl Related<AcademicSurveyTransactionConduct::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AcademicSurveyTransactionConducts.def()
    }
}

impl Related<AcademicCampaignTransactionClassCode::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AcademicCampaignTransactionClassCode.def()
    }
}

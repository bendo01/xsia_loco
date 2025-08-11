use crate::models::academic::campaign::transaction::activities::_entities::activities as AcademicCampaignTransactionActivity;
use crate::models::academic::campaign::transaction::teaches::_entities::teaches as AcademicCampaignTransactionTeach;
use crate::models::academic::survey::master::bundles::_entities::bundles as AcademicSurveyMasterBundle;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "academic_survey_transaction", table_name = "conducts")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub code: Option<i32>,
    pub alphabet_code: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    pub bundle_id: Uuid,
    pub activity_id: Uuid,
    pub teach_id: Uuid,
    pub conductable_type: String,
    pub conductable_id: Uuid,
    pub is_finish: bool,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    AcademicCampaignTransactionActivity,
    AcademicCampaignTransactionTeach,
    AcademicSurveyMasterBundle,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::AcademicCampaignTransactionActivity => {
                Entity::belongs_to(AcademicCampaignTransactionActivity::Entity)
                    .from(Column::ActivityId)
                    .to(AcademicCampaignTransactionActivity::Column::Id)
                    .into()
            }
            Self::AcademicCampaignTransactionTeach => {
                Entity::belongs_to(AcademicCampaignTransactionTeach::Entity)
                    .from(Column::TeachId)
                    .to(AcademicCampaignTransactionTeach::Column::Id)
                    .into()
            }
            Self::AcademicSurveyMasterBundle => {
                Entity::belongs_to(AcademicSurveyMasterBundle::Entity)
                    .from(Column::BundleId)
                    .to(AcademicSurveyMasterBundle::Column::Id)
                    .into()
            }
        }
    }
}

impl Related<AcademicCampaignTransactionActivity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AcademicCampaignTransactionActivity.def()
    }
}

impl Related<AcademicCampaignTransactionTeach::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AcademicCampaignTransactionTeach.def()
    }
}

impl Related<AcademicSurveyMasterBundle::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AcademicSurveyMasterBundle.def()
    }
}

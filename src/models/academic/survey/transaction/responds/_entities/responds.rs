use crate::models::academic::survey::master::answers::_entities::answers as AcademicSurveyMasterAnswer;
use crate::models::academic::survey::master::bundles::_entities::bundles as AcademicSurveyMasterBundle;
use crate::models::academic::survey::master::questions::_entities::questions as AcademicSurveyMasterQuestion;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "academic_survey_transaction", table_name = "responds")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(column_type = "Text", nullable)]
    pub name: Option<String>,
    pub conduct_id: Uuid,
    pub bundle_id: Uuid,
    pub question_id: Uuid,
    pub answer_id: Option<Uuid>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    AcademicSurveyMasterAnswer,
    AcademicSurveyMasterBundle,
    AcademicSurveyMasterQuestion,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::AcademicSurveyMasterAnswer => {
                Entity::belongs_to(AcademicSurveyMasterAnswer::Entity)
                    .from(Column::AnswerId)
                    .to(AcademicSurveyMasterAnswer::Column::Id)
                    .into()
            }
            Self::AcademicSurveyMasterBundle => {
                Entity::belongs_to(AcademicSurveyMasterBundle::Entity)
                    .from(Column::BundleId)
                    .to(AcademicSurveyMasterBundle::Column::Id)
                    .into()
            }
            Self::AcademicSurveyMasterQuestion => {
                Entity::belongs_to(AcademicSurveyMasterQuestion::Entity)
                    .from(Column::QuestionId)
                    .to(AcademicSurveyMasterQuestion::Column::Id)
                    .into()
            }
        }
    }
}

impl Related<AcademicSurveyMasterAnswer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AcademicSurveyMasterAnswer.def()
    }
}

impl Related<AcademicSurveyMasterBundle::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AcademicSurveyMasterBundle.def()
    }
}

impl Related<AcademicSurveyMasterQuestion::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AcademicSurveyMasterQuestion.def()
    }
}

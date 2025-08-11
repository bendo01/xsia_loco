use crate::models::academic::survey::master::questions::_entities::questions as AcademicSurveyMasterQuestion;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "academic_survey_master", table_name = "answers")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub code: i32,
    pub alphabet_code: Option<String>,
    pub name: String,
    pub question_id: Uuid,
    #[sea_orm(column_type = "Double")]
    pub point: f64,
    #[sea_orm(column_type = "Text", nullable)]
    pub suggestion: Option<String>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    AcademicSurveyMasterQuestion,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::AcademicSurveyMasterQuestion => {
                Entity::belongs_to(AcademicSurveyMasterQuestion::Entity)
                    .from(Column::QuestionId)
                    .to(AcademicSurveyMasterQuestion::Column::Id)
                    .into()
            }
        }
    }
}

impl Related<AcademicSurveyMasterQuestion::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AcademicSurveyMasterQuestion.def()
    }
}

use crate::models::academic::survey::master::questions::_entities::questions as AcademicSurveyMasterQuestion;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(
    schema_name = "academic_survey_reference",
    table_name = "question_varieties"
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub code: i32,
    pub alphabet_code: String,
    pub name: String,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    AcademicSurveyMasterQuestions,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::AcademicSurveyMasterQuestions => {
                Entity::has_many(AcademicSurveyMasterQuestion::Entity)
                    .from(Column::Id)
                    .to(AcademicSurveyMasterQuestion::Column::QuestionVarietyId)
                    .into()
            }
        }
    }
}

impl Related<AcademicSurveyMasterQuestion::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AcademicSurveyMasterQuestions.def()
    }
}

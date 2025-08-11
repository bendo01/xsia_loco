use crate::models::academic::survey::master::answers::_entities::answers as AcademicSurveyMasterAnswer;
use crate::models::academic::survey::reference::question_varieties::_entities::question_varieties as AcademicSurveyReferenceQuestionVariety;
use crate::models::institution::master::institutions::_entities::institutions as InstitutionMasterInstitution;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "academic_survey_master", table_name = "questions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub code: i32,
    pub alphabet_code: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    pub institution_id: Uuid,
    pub question_variety_id: Option<Uuid>,
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
    InstitutionMasterInstitution,
    AcademicSurveyReferenceQuestionVariety,
    AcademicSurveyMasterAnswers,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::InstitutionMasterInstitution => {
                Entity::belongs_to(InstitutionMasterInstitution::Entity)
                    .from(Column::InstitutionId)
                    .to(InstitutionMasterInstitution::Column::Id)
                    .into()
            }
            Self::AcademicSurveyReferenceQuestionVariety => {
                Entity::belongs_to(AcademicSurveyReferenceQuestionVariety::Entity)
                    .from(Column::QuestionVarietyId)
                    .to(AcademicSurveyReferenceQuestionVariety::Column::Id)
                    .into()
            }
            Self::AcademicSurveyMasterAnswers => {
                Entity::has_many(AcademicSurveyMasterAnswer::Entity)
                    .from(Column::Id)
                    .to(AcademicSurveyMasterAnswer::Column::QuestionId)
                    .into()
            }
        }
    }
}

impl Related<InstitutionMasterInstitution::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::InstitutionMasterInstitution.def()
    }
}

impl Related<AcademicSurveyReferenceQuestionVariety::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AcademicSurveyReferenceQuestionVariety.def()
    }
}

impl Related<AcademicSurveyMasterAnswer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AcademicSurveyMasterAnswers.def()
    }
}

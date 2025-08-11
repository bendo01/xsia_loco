use crate::models::academic::survey::master::answers::_entities::answers as AcademicSurveyMasterAnswer;
use crate::models::academic::survey::master::questions::_entities::questions as AcademicSurveyMasterQuestion;
use crate::models::academic::survey::reference::question_varieties::_entities::question_varieties as AcademicSurveyReferenceQuestionVariety;
use crate::models::institution::master::institutions::_entities::institutions as InstitutionMasterInstitution;
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(clippy::too_many_lines)]
pub struct QuestionDataObject {
    pub question: AcademicSurveyMasterQuestion::Model,
    pub institution: Option<InstitutionMasterInstitution::Model>,
    pub question_variety: Option<AcademicSurveyReferenceQuestionVariety::Model>,
    pub answers: Option<Vec<AcademicSurveyMasterAnswer::Model>>,
}

impl QuestionDataObject {
    #[allow(clippy::missing_errors_doc)]
    #[allow(clippy::too_many_lines)]
    pub async fn get_by_id(
        ctx: &AppContext,
        id: Uuid,
        with_has_many_relationships: bool,
    ) -> Result<Option<Self>> {
        let question = AcademicSurveyMasterQuestion::Entity::find_by_id(id)
            .filter(AcademicSurveyMasterQuestion::Column::DeletedAt.is_null())
            .one(&ctx.db)
            .await?;
        if let Some(question) = question {
            let institution = question
                .find_related(InstitutionMasterInstitution::Entity)
                .filter(InstitutionMasterInstitution::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            let question_variety = question
                .find_related(AcademicSurveyReferenceQuestionVariety::Entity)
                .filter(AcademicSurveyReferenceQuestionVariety::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            let answers = if with_has_many_relationships {
                let answers_list = question
                    .find_related(AcademicSurveyMasterAnswer::Entity)
                    .filter(AcademicSurveyMasterAnswer::Column::DeletedAt.is_null())
                    .all(&ctx.db)
                    .await?;
                if answers_list.is_empty() {
                    None
                } else {
                    Some(answers_list)
                }
            } else {
                None
            };

            Ok(Some(Self {
                question,
                institution,
                question_variety,
                answers,
            }))
        } else {
            Ok(None)
        }
    }
}

use crate::models::academic::candidate::master::candidates::_entities::candidates as AcademicCandidateMasterCandidate;
use crate::models::academic::course::master::curriculums::_entities::curriculums as AcademicCourseMasterCurriculum;
use crate::models::academic::prior_learning_recognition::transaction::recognitions::_entities::recognitions as AcademicPriorLearningRecognitionTransactionRecognition;
use crate::models::institution::master::units::_entities::units as InstitutionMasterUnit;

use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(clippy::too_many_lines)]
pub struct DataObject {
    pub candidate: AcademicCandidateMasterCandidate::Model,
    pub recognition: Option<AcademicPriorLearningRecognitionTransactionRecognition::Model>,
    pub curriculum: Option<AcademicCourseMasterCurriculum::Model>,
    pub unit: Option<InstitutionMasterUnit::Model>,
}

impl DataObject {
    #[allow(clippy::missing_errors_doc)]
    #[allow(clippy::too_many_lines)]
    pub async fn get_by_id(
        ctx: &AppContext,
        id: Uuid,
        _with_has_many_relationships: bool,
    ) -> Result<Option<Self>> {
        let candidate = AcademicCandidateMasterCandidate::Entity::find_by_id(id)
            .filter(AcademicCandidateMasterCandidate::Column::DeletedAt.is_null())
            .one(&ctx.db)
            .await?;

        if let Some(candidate) = candidate {
            let recognition_opt =
                AcademicPriorLearningRecognitionTransactionRecognition::Entity::find()
                    .filter(
                        AcademicPriorLearningRecognitionTransactionRecognition::Column::DeletedAt
                            .is_null(),
                    )
                    .filter(
                        AcademicPriorLearningRecognitionTransactionRecognition::Column::CandidateId
                            .eq(candidate.id),
                    )
                    .one(&ctx.db)
                    .await?;
            if let Some(recognition) = recognition_opt {
                let curriculum = recognition
                    .find_related(AcademicCourseMasterCurriculum::Entity)
                    .filter(AcademicCourseMasterCurriculum::Column::DeletedAt.is_null())
                    .one(&ctx.db)
                    .await?;
                let unit = recognition
                    .find_related(InstitutionMasterUnit::Entity)
                    .filter(InstitutionMasterUnit::Column::DeletedAt.is_null())
                    .one(&ctx.db)
                    .await?;
                Ok(Some(Self {
                    candidate,
                    recognition: Some(recognition),
                    curriculum,
                    unit,
                }))
            } else {
                Ok(Some(Self {
                    candidate,
                    recognition: None,
                    curriculum: None,
                    unit: None,
                }))
            }
        } else {
            Ok(None)
        }
    }
}

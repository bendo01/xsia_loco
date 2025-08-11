use crate::models::academic::candidate::master::candidates::_entities::candidates as AcademicCandidateMasterCandidate;
use crate::models::academic::candidate::reference::phases::_entities::phases as AcademicCandidateReferencePhase;
use crate::models::academic::candidate::reference::registration_categories::_entities::registration_categories as AcademicCandidateReferenceRegistrationCategory;
use crate::models::academic::candidate::transaction::unit_choices::_entities::unit_choices as AcademicCandidateTransactionUnitChoice;
use crate::models::academic::student::reference::registrations::_entities::registrations as AcademicStudentReferenceRegistration;
use crate::models::institution::master::units::_entities::units as InstitutionMasterUnit;
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(clippy::too_many_lines)]
pub struct CandidateUnitChoiceDataObject {
    pub unit_choice: AcademicCandidateTransactionUnitChoice::Model,
    pub candidate: Option<AcademicCandidateMasterCandidate::Model>,
    pub phase: Option<AcademicCandidateReferencePhase::Model>,
    pub unit: Option<InstitutionMasterUnit::Model>,
    pub student_registration: Option<AcademicStudentReferenceRegistration::Model>,
    pub registration_category: Option<AcademicCandidateReferenceRegistrationCategory::Model>,
}

impl CandidateUnitChoiceDataObject {
    #[allow(clippy::missing_errors_doc)]
    #[allow(clippy::too_many_lines)]
    pub async fn get_by_id(ctx: &AppContext, id: Uuid) -> Result<Option<Self>> {
        let unit_choice = AcademicCandidateTransactionUnitChoice::Entity::find_by_id(id)
            .filter(AcademicCandidateTransactionUnitChoice::Column::DeletedAt.is_null())
            .one(&ctx.db)
            .await?;
        if let Some(unit_choice) = unit_choice {
            let candidate = unit_choice
                .find_related(AcademicCandidateMasterCandidate::Entity)
                .filter(AcademicCandidateMasterCandidate::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            let phase = unit_choice
                .find_related(AcademicCandidateReferencePhase::Entity)
                .filter(AcademicCandidateReferencePhase::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            let unit = unit_choice
                .find_related(InstitutionMasterUnit::Entity)
                .filter(InstitutionMasterUnit::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            let student_registration = unit_choice
                .find_related(AcademicStudentReferenceRegistration::Entity)
                .filter(AcademicStudentReferenceRegistration::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            let registration_category = unit_choice
                .find_related(AcademicCandidateReferenceRegistrationCategory::Entity)
                .filter(AcademicCandidateReferenceRegistrationCategory::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            Ok(Some(Self {
                unit_choice,
                candidate,
                phase,
                unit,
                student_registration,
                registration_category,
            }))
        } else {
            Ok(None)
        }
    }
}

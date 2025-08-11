use crate::models::academic::candidate::master::candidates::_entities::candidates as AcademicCanditateMasterCandidate;
use crate::models::academic::candidate::reference::registration_types::_entities::registration_types as AcademicCandidateReferenceRegistrationType;
use crate::models::academic::general::reference::academic_years::_entities::academic_years as AcademicGeneralReferenceAcademicYear;
use crate::models::location::regencies::_entities::regencies as LocationRegencies;
// use crate::models::person::master::individuals::_entities::individuals as PersonMasterIndividual;
use crate::models::person::master::individuals::data_objects::DataObject as PersonMasterIndividualDataObject;

use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(clippy::too_many_lines)]
pub struct DataObject {
    pub candidate: AcademicCanditateMasterCandidate::Model,
    pub registration_type: Option<AcademicCandidateReferenceRegistrationType::Model>,
    pub academic_year: Option<AcademicGeneralReferenceAcademicYear::Model>,
    pub regency: Option<LocationRegencies::Model>,
    pub individual: Option<PersonMasterIndividualDataObject>,
}

impl DataObject {
    #[allow(clippy::missing_errors_doc)]
    #[allow(clippy::too_many_lines)]
    pub async fn get_by_id(
        ctx: &AppContext,
        id: Uuid,
        _with_has_many_relationships: bool,
    ) -> Result<Option<Self>> {
        let candidate = AcademicCanditateMasterCandidate::Entity::find_by_id(id)
            .filter(AcademicCanditateMasterCandidate::Column::DeletedAt.is_null())
            .one(&ctx.db)
            .await?;
        if let Some(candidate) = candidate {
            let registration_type = AcademicCandidateReferenceRegistrationType::Entity::find_by_id(
                candidate.registration_type_id,
            )
            .filter(AcademicCandidateReferenceRegistrationType::Column::DeletedAt.is_null())
            .one(&ctx.db)
            .await?;
            let academic_year = AcademicGeneralReferenceAcademicYear::Entity::find_by_id(
                candidate.academic_year_id,
            )
            .filter(AcademicGeneralReferenceAcademicYear::Column::DeletedAt.is_null())
            .one(&ctx.db)
            .await?;
            let regency = LocationRegencies::Entity::find_by_id(candidate.school_regency_id)
                .filter(LocationRegencies::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;
            let individual =
                PersonMasterIndividualDataObject::get_by_id(&ctx, candidate.individual_id, true)
                    .await?;

            Ok(Some(Self {
                candidate,
                registration_type,
                academic_year,
                regency,
                individual,
            }))
        } else {
            Ok(None)
        }
    }
}

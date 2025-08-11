use crate::models::academic::survey::master::bundles::_entities::bundles as AcademicSurveyMasterBundle;
use crate::models::institution::master::institutions::_entities::institutions as InstitutionMasterInstitution;
use crate::models::institution::master::staffes::_entities::staffes as InstitutionMasterStaff;
use crate::models::institution::master::units::_entities::units as InstitutionMasterUnit;
use crate::models::institution::reference::unit_types::_entities::unit_types as InstitutionReferenceUnitType;
use crate::models::literate::educations::_entities::educations as LiterateEducation;
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataObject {
    #[serde(flatten)]
    pub unit: InstitutionMasterUnit::Model,
    pub unit_type: Option<InstitutionReferenceUnitType::Model>,
    pub institution: Option<InstitutionMasterInstitution::Model>,
    pub education: Option<LiterateEducation::Model>,
    pub staffes: Option<Vec<InstitutionMasterStaff::Model>>,
    pub bundles: Option<Vec<AcademicSurveyMasterBundle::Model>>,
}

impl DataObject {
    #[allow(clippy::missing_errors_doc)]
    pub async fn get_by_id(
        ctx: &AppContext,
        id: Uuid,
        with_has_many_relationships: bool,
    ) -> Result<Option<Self>> {
        let unit = InstitutionMasterUnit::Entity::find_by_id(id)
            .filter(InstitutionMasterUnit::Column::DeletedAt.is_null())
            .one(&ctx.db)
            .await?;
        if let Some(unit) = unit {
            let unit_type = unit
                .find_related(InstitutionReferenceUnitType::Entity)
                .filter(InstitutionReferenceUnitType::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;
            let education = unit
                .find_related(LiterateEducation::Entity)
                .filter(LiterateEducation::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;
            let institution = unit
                .find_related(InstitutionMasterInstitution::Entity)
                .filter(InstitutionMasterInstitution::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;
            // Conditionally load staffes based on the flag
            let staffes = if with_has_many_relationships {
                let staffes = unit
                    .find_related(InstitutionMasterStaff::Entity)
                    .filter(InstitutionMasterStaff::Column::DeletedAt.is_null())
                    .all(&ctx.db)
                    .await?;
                if staffes.is_empty() {
                    None
                } else {
                    Some(staffes)
                }
            } else {
                None
            };
            let bundles = if with_has_many_relationships {
                let bundles = unit
                    .find_related(AcademicSurveyMasterBundle::Entity)
                    .filter(AcademicSurveyMasterBundle::Column::DeletedAt.is_null())
                    .all(&ctx.db)
                    .await?;
                if bundles.is_empty() {
                    None
                } else {
                    Some(bundles)
                }
            } else {
                None
            };
            Ok(Some(Self {
                unit,
                unit_type,
                institution,
                education,
                staffes,
                bundles,
            }))
        } else {
            Ok(None)
        }
    }
}

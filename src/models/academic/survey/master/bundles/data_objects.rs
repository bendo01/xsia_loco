use crate::models::academic::general::reference::academic_years::_entities::academic_years as AcademicGeneralReferenceAcademicYear;
use crate::models::academic::survey::master::bundles::_entities::bundles as AcademicSurveyMasterBundle;
use crate::models::academic::survey::reference::bundle_categories::_entities::bundle_categories as AcademicSurveyReferenceBundleCategory;
use crate::models::academic::survey::transaction::conducts::_entities::conducts as AcademicSurveyTransactionConduct;
use crate::models::institution::master::institutions::_entities::institutions as InstitutionMasterInstitution;
use crate::models::institution::master::units::_entities::units as InstitutionMasterUnit;
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(clippy::too_many_lines)]
pub struct BundleDataObject {
    pub bundle: AcademicSurveyMasterBundle::Model,
    pub institution: Option<InstitutionMasterInstitution::Model>,
    pub bundle_category: Option<AcademicSurveyReferenceBundleCategory::Model>,
    pub unit: Option<InstitutionMasterUnit::Model>,
    pub academic_year: Option<AcademicGeneralReferenceAcademicYear::Model>,
    pub conducts: Option<Vec<AcademicSurveyTransactionConduct::Model>>,
}

impl BundleDataObject {
    #[allow(clippy::missing_errors_doc)]
    #[allow(clippy::too_many_lines)]
    pub async fn get_by_id(
        ctx: &AppContext,
        id: Uuid,
        with_has_many_relationships: bool,
    ) -> Result<Option<Self>> {
        let bundle = AcademicSurveyMasterBundle::Entity::find_by_id(id)
            .filter(AcademicSurveyMasterBundle::Column::DeletedAt.is_null())
            .one(&ctx.db)
            .await?;
        if let Some(bundle) = bundle {
            let institution = bundle
                .find_related(InstitutionMasterInstitution::Entity)
                .filter(InstitutionMasterInstitution::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            let bundle_category = bundle
                .find_related(AcademicSurveyReferenceBundleCategory::Entity)
                .filter(AcademicSurveyReferenceBundleCategory::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            let unit = bundle
                .find_related(InstitutionMasterUnit::Entity)
                .filter(InstitutionMasterUnit::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            let academic_year = bundle
                .find_related(AcademicGeneralReferenceAcademicYear::Entity)
                .filter(AcademicGeneralReferenceAcademicYear::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            let conducts = if with_has_many_relationships {
                let conducts_list = bundle
                    .find_related(AcademicSurveyTransactionConduct::Entity)
                    .filter(AcademicSurveyTransactionConduct::Column::DeletedAt.is_null())
                    .all(&ctx.db)
                    .await?;
                if conducts_list.is_empty() {
                    None
                } else {
                    Some(conducts_list)
                }
            } else {
                None
            };

            Ok(Some(Self {
                bundle,
                institution,
                bundle_category,
                unit,
                academic_year,
                conducts,
            }))
        } else {
            Ok(None)
        }
    }
}

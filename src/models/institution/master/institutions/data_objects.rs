use crate::models::academic::general::reference::academic_years::_entities::academic_years as AcademicGeneralReferenceAcademicYear;
use crate::models::institution::master::employees::_entities::employees as InstitutionMasterEmployee;
use crate::models::institution::master::institutions::_entities::institutions as InstitutionMasterInstitution;
use crate::models::institution::master::units::_entities::units as InstitutionMasterUnit;
use crate::models::institution::reference::categories::_entities::categories as InstitutionReferenceCategory;
use crate::models::institution::reference::varieties::_entities::varieties as InstitutionReferenceVariety;
use crate::models::location::countries::_entities::countries as LocationCountry;
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(clippy::too_many_lines)]
pub struct InstitutionDataObject {
    pub institution: InstitutionMasterInstitution::Model,
    pub academic_year: Option<AcademicGeneralReferenceAcademicYear::Model>,
    pub category: Option<InstitutionReferenceCategory::Model>,
    pub variety: Option<InstitutionReferenceVariety::Model>,
    pub country: Option<LocationCountry::Model>,
    pub units: Option<Vec<InstitutionMasterUnit::Model>>,
    pub employees: Option<Vec<InstitutionMasterEmployee::Model>>,
}

impl InstitutionDataObject {
    #[allow(clippy::missing_errors_doc)]
    #[allow(clippy::too_many_lines)]
    pub async fn get_by_id(ctx: &AppContext, id: Uuid) -> Result<Option<Self>> {
        let institution = InstitutionMasterInstitution::Entity::find_by_id(id)
            .filter(InstitutionMasterInstitution::Column::DeletedAt.is_null())
            .one(&ctx.db)
            .await?;
        if let Some(institution) = institution {
            let academic_year = institution
                .find_related(AcademicGeneralReferenceAcademicYear::Entity)
                .filter(AcademicGeneralReferenceAcademicYear::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;
            let category = institution
                .find_related(InstitutionReferenceCategory::Entity)
                .filter(InstitutionReferenceCategory::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;
            let variety = institution
                .find_related(InstitutionReferenceVariety::Entity)
                .filter(InstitutionReferenceVariety::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;
            let country = institution
                .find_related(LocationCountry::Entity)
                .filter(LocationCountry::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;
            let units = institution
                .find_related(InstitutionMasterUnit::Entity)
                .filter(InstitutionMasterUnit::Column::DeletedAt.is_null())
                .all(&ctx.db)
                .await?;
            let employees = institution
                .find_related(InstitutionMasterEmployee::Entity)
                .filter(InstitutionMasterEmployee::Column::DeletedAt.is_null())
                .all(&ctx.db)
                .await?;
            Ok(Some(Self {
                institution,
                academic_year,
                category,
                variety,
                country,
                units: if units.is_empty() { None } else { Some(units) },
                employees: if employees.is_empty() {
                    None
                } else {
                    Some(employees)
                },
            }))
        } else {
            Ok(None)
        }
    }
}

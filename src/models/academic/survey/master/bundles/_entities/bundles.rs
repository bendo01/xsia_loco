use crate::models::academic::general::reference::academic_years::_entities::academic_years as AcademicGeneralReferenceAcademicYear;
use crate::models::academic::survey::reference::bundle_categories::_entities::bundle_categories as AcademicSurveyReferenceBundleCategory;
use crate::models::academic::survey::transaction::conducts::_entities::conducts as AcademicSurveyTransactionConduct;
use crate::models::institution::master::institutions::_entities::institutions as InstitutionMasterInstitution;
use crate::models::institution::master::units::_entities::units as InstitutionMasterUnit;

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "academic_survey_master", table_name = "bundles")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub code: i32,
    pub alphabet_code: String,
    pub name: String,
    pub institution_id: Uuid,
    pub bundle_category_id: Uuid,
    pub unit_id: Option<Uuid>,
    pub academic_year_id: Option<Uuid>,
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
    AcademicSurveyTransactionConducts,
    InstitutionMasterInstitution,
    AcademicSurveyReferenceBundleCategory,
    InstitutionMasterUnit,
    AcademicGeneralReferenceAcademicYear,
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
            Self::AcademicSurveyReferenceBundleCategory => {
                Entity::belongs_to(AcademicSurveyReferenceBundleCategory::Entity)
                    .from(Column::BundleCategoryId)
                    .to(AcademicSurveyReferenceBundleCategory::Column::Id)
                    .into()
            }
            Self::InstitutionMasterUnit => Entity::belongs_to(InstitutionMasterUnit::Entity)
                .from(Column::UnitId)
                .to(InstitutionMasterUnit::Column::Id)
                .into(),
            Self::AcademicGeneralReferenceAcademicYear => {
                Entity::belongs_to(AcademicGeneralReferenceAcademicYear::Entity)
                    .from(Column::AcademicYearId)
                    .to(AcademicGeneralReferenceAcademicYear::Column::Id)
                    .into()
            }
            Self::AcademicSurveyTransactionConducts => {
                Entity::has_many(AcademicSurveyTransactionConduct::Entity)
                    .from(Column::Id)
                    .to(AcademicSurveyTransactionConduct::Column::BundleId)
                    .into()
            }
        }
    }
}

impl Related<AcademicSurveyTransactionConduct::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AcademicSurveyTransactionConducts.def()
    }
}

impl Related<InstitutionMasterInstitution::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::InstitutionMasterInstitution.def()
    }
}

impl Related<AcademicGeneralReferenceAcademicYear::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AcademicGeneralReferenceAcademicYear.def()
    }
}

impl Related<AcademicSurveyReferenceBundleCategory::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AcademicSurveyReferenceBundleCategory.def()
    }
}

impl Related<InstitutionMasterUnit::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::InstitutionMasterUnit.def()
    }
}

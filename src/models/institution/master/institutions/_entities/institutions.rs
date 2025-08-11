use crate::models::academic::general::reference::academic_years::_entities::academic_years as AcademicGeneralReferenceAcademicYear;
use crate::models::institution::master::employees::_entities::employees as InstitutionMasterEmployee;
use crate::models::institution::master::units::_entities::units as InstitutionMasterUnit;
use crate::models::institution::reference::categories::_entities::categories as InstitutionReferenceCategory;
use crate::models::institution::reference::varieties::_entities::varieties as InstitutionReferenceVariety;
use crate::models::location::countries::_entities::countries as LocationCountry;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "institution_master", table_name = "institutions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub alphabet_code: Option<String>,
    pub is_active: bool,
    pub variety_id: Uuid,
    pub category_id: Uuid,
    pub country_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub feeder_id: Option<Uuid>,
    pub academic_year_id: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Category,
    Variety,
    Country,
    AcademicYear,
    Units,
    Employees,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Category => Entity::belongs_to(InstitutionReferenceCategory::Entity)
                .from(Column::CategoryId)
                .to(InstitutionReferenceCategory::Column::Id)
                .into(),
            Self::Variety => Entity::belongs_to(InstitutionReferenceVariety::Entity)
                .from(Column::VarietyId)
                .to(InstitutionReferenceVariety::Column::Id)
                .into(),
            Self::Country => Entity::belongs_to(LocationCountry::Entity)
                .from(Column::CountryId)
                .to(LocationCountry::Column::Id)
                .into(),
            Self::AcademicYear => Entity::belongs_to(AcademicGeneralReferenceAcademicYear::Entity)
                .from(Column::AcademicYearId)
                .to(AcademicGeneralReferenceAcademicYear::Column::Id)
                .into(),
            Self::Units => Entity::has_many(InstitutionMasterUnit::Entity)
                .from(Column::Id)
                .to(InstitutionMasterUnit::Column::InstitutionId)
                .into(),
            Self::Employees => Entity::has_many(InstitutionMasterEmployee::Entity)
                .from(Column::Id)
                .to(InstitutionMasterEmployee::Column::InstitutionId)
                .into(),
        }
    }
}

impl Related<InstitutionReferenceCategory::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Category.def()
    }
}

impl Related<InstitutionReferenceVariety::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Variety.def()
    }
}

impl Related<LocationCountry::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Country.def()
    }
}

impl Related<AcademicGeneralReferenceAcademicYear::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AcademicYear.def()
    }
}

impl Related<InstitutionMasterUnit::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Units.def()
    }
}

impl Related<InstitutionMasterEmployee::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Employees.def()
    }
}

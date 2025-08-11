use crate::models::academic::general::reference::academic_years::_entities::academic_years as AcademicGeneralReferenceAcademicYear;
use crate::models::academic::survey::transaction::conducts::_entities::conducts as AcademicSurveyTransactionConduct;
use crate::models::institution::master::units::_entities::units as InstitutionMasterUnit;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(
    schema_name = "academic_campaign_transaction",
    table_name = "activities"
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub unit_id: Uuid,
    pub academic_year_id: Uuid,
    pub week_quantity: i32,
    pub student_target: i32,
    pub candidate_number: i32,
    pub candidate_pass: i32,
    pub became_student: i32,
    pub transfer_student: i32,
    pub total_class_member: i32,
    pub start_date: Option<Date>,
    pub end_date: Option<Date>,
    pub start_transaction: Option<Date>,
    pub end_transaction: Option<Date>,
    pub is_active: bool,
    pub feeder_id: Option<Uuid>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    AcademicGeneralReferenceAcademicYear,
    AcademicSurveyTransactionConducts,
    InstitutionMasterUnit,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::AcademicGeneralReferenceAcademicYear => {
                Entity::belongs_to(AcademicGeneralReferenceAcademicYear::Entity)
                    .from(Column::AcademicYearId)
                    .to(AcademicGeneralReferenceAcademicYear::Column::Id)
                    .into()
            }

            Self::InstitutionMasterUnit => Entity::belongs_to(InstitutionMasterUnit::Entity)
                .from(Column::UnitId)
                .to(InstitutionMasterUnit::Column::Id)
                .into(),
            Self::AcademicSurveyTransactionConducts => {
                Entity::has_many(AcademicSurveyTransactionConduct::Entity)
                    .from(Column::Id)
                    .to(AcademicSurveyTransactionConduct::Column::ActivityId)
                    .into()
            }
        }
    }
}

impl Related<AcademicGeneralReferenceAcademicYear::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AcademicGeneralReferenceAcademicYear.def()
    }
}

impl Related<InstitutionMasterUnit::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::InstitutionMasterUnit.def()
    }
}

impl Related<AcademicSurveyTransactionConduct::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AcademicSurveyTransactionConducts.def()
    }
}

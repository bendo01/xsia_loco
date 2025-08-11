use crate::models::academic::campaign::transaction::calendar_details::_entities::calendar_details as AcademicCampaignTransactionCalendarDetail;
use crate::models::academic::general::reference::academic_years::_entities::academic_years as AcademicGeneralReferenceAcademicYear;
use crate::models::institution::master::institutions::_entities::institutions as InstitutionMasterInstitution;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(
    schema_name = "academic_campaign_transaction",
    table_name = "calendars"
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub academic_year_id: Uuid,
    pub institution_id: Uuid,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    AcademicYear,
    Institution,
    CalendarDetails,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::AcademicYear => Entity::belongs_to(AcademicGeneralReferenceAcademicYear::Entity)
                .from(Column::AcademicYearId)
                .to(AcademicGeneralReferenceAcademicYear::Column::Id)
                .into(),
            Self::Institution => Entity::belongs_to(InstitutionMasterInstitution::Entity)
                .from(Column::InstitutionId)
                .to(InstitutionMasterInstitution::Column::Id)
                .into(),
            Self::CalendarDetails => {
                Entity::has_many(AcademicCampaignTransactionCalendarDetail::Entity)
                    .from(Column::Id)
                    .to(AcademicCampaignTransactionCalendarDetail::Column::CalendarId)
                    .into()
            }
        }
    }
}

impl Related<AcademicGeneralReferenceAcademicYear::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AcademicYear.def()
    }
}

impl Related<InstitutionMasterInstitution::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Institution.def()
    }
}

impl Related<AcademicCampaignTransactionCalendarDetail::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CalendarDetails.def()
    }
}

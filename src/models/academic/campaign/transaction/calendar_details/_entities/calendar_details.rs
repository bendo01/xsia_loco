use crate::models::academic::campaign::reference::calendar_categories::_entities::calendar_categories as AcademicCampaignReferenceCalendarCategory;
use crate::models::academic::campaign::transaction::calendars::_entities::calendars as AcademicCampaignTransactionCalendar;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(
    schema_name = "academic_campaign_transaction",
    table_name = "calendar_details"
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub calendar_category_id: Uuid,
    pub calendar_id: Uuid,
    pub start_date: Date,
    pub end_date: Date,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    CalendarCategory,
    Calendar,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::CalendarCategory => {
                Entity::belongs_to(AcademicCampaignReferenceCalendarCategory::Entity)
                    .from(Column::CalendarCategoryId)
                    .to(AcademicCampaignReferenceCalendarCategory::Column::Id)
                    .into()
            }
            Self::Calendar => Entity::belongs_to(AcademicCampaignTransactionCalendar::Entity)
                .from(Column::CalendarId)
                .to(AcademicCampaignTransactionCalendar::Column::Id)
                .into(),
        }
    }
}

impl Related<AcademicCampaignReferenceCalendarCategory::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CalendarCategory.def()
    }
}

impl Related<AcademicCampaignTransactionCalendar::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Calendar.def()
    }
}

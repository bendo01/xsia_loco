use crate::models::academic::campaign::transaction::calendar_details::_entities::calendar_details as AcademicCampaignTransactionCalendarDetail;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(
    schema_name = "academic_campaign_reference",
    table_name = "calendar_categories"
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub code: i32,
    pub alphabet_code: String,
    pub name: String,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    CalendarDetails,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::CalendarDetails => {
                Entity::has_many(AcademicCampaignTransactionCalendarDetail::Entity)
                    .from(Column::Id)
                    .to(AcademicCampaignTransactionCalendarDetail::Column::CalendarCategoryId)
                    .into()
            }
        }
    }
}

impl Related<AcademicCampaignTransactionCalendarDetail::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CalendarDetails.def()
    }
}

use crate::models::academic::campaign::transaction::teaches::_entities::teaches as AcademicCampaignTransactionTeaches;
use crate::models::building::master::rooms::_entities::rooms as BuildingMasterRoom;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(
    schema_name = "academic_campaign_transaction",
    table_name = "schedules"
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(column_type = "Text", nullable)]
    pub name: Option<String>,
    pub start_hour: Time,
    pub end_hour: Time,
    pub weekday_id: Uuid,
    pub room_id: Uuid,
    pub teach_id: Uuid,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Room,
    Teach,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Room => Entity::belongs_to(BuildingMasterRoom::Entity)
                .from(Column::RoomId)
                .to(BuildingMasterRoom::Column::Id)
                .into(),
            Self::Teach => Entity::belongs_to(AcademicCampaignTransactionTeaches::Entity)
                .from(Column::TeachId)
                .to(AcademicCampaignTransactionTeaches::Column::Id)
                .into(),
        }
    }
}

impl Related<BuildingMasterRoom::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Room.def()
    }
}

impl Related<AcademicCampaignTransactionTeaches::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Teach.def()
    }
}

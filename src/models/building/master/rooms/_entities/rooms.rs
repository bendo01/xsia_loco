use crate::models::building::master::buildings::_entities::buildings as BuildingMasterBuilding;
use crate::models::building::reference::conditions::_entities::conditions as BuildingReferencesCondition;
use crate::models::building::reference::room_types::_entities::room_types as BuildingReferenceRoomType;
use crate::models::institution::master::units::_entities::units as InstitutionMasterUnit;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "building_master", table_name = "rooms")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub code: Option<i32>,
    pub alphabet_code: Option<String>,
    pub name: String,
    #[sea_orm(column_type = "Float", nullable)]
    pub long: Option<f32>,
    #[sea_orm(column_type = "Float", nullable)]
    pub wide: Option<f32>,
    #[sea_orm(column_type = "Float", nullable)]
    pub high: Option<f32>,
    pub room_type_id: Uuid,
    pub unit_id: Option<Uuid>,
    pub building_id: Uuid,
    pub condition_id: Uuid,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    RoomType,
    Unit,
    Building,
    Condition,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::RoomType => Entity::belongs_to(BuildingReferenceRoomType::Entity)
                .from(Column::RoomTypeId)
                .to(BuildingReferenceRoomType::Column::Id)
                .into(),
            Self::Unit => Entity::belongs_to(InstitutionMasterUnit::Entity)
                .from(Column::UnitId)
                .to(InstitutionMasterUnit::Column::Id)
                .into(),
            Self::Building => Entity::belongs_to(BuildingMasterBuilding::Entity)
                .from(Column::BuildingId)
                .to(BuildingMasterBuilding::Column::Id)
                .into(),
            Self::Condition => Entity::belongs_to(BuildingReferencesCondition::Entity)
                .from(Column::ConditionId)
                .to(BuildingReferencesCondition::Column::Id)
                .into(),
        }
    }
}

impl Related<BuildingReferenceRoomType::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RoomType.def()
    }
}

impl Related<InstitutionMasterUnit::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Unit.def()
    }
}

impl Related<BuildingMasterBuilding::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Building.def()
    }
}

impl Related<BuildingReferencesCondition::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Condition.def()
    }
}

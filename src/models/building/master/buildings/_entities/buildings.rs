use crate::models::building::reference::categories::_entities::categories as BuildingReferenceCategory;
use crate::models::building::reference::conditions::_entities::conditions as BuildingReferenceCondition;
use crate::models::building::reference::varieties::_entities::varieties as BuildingReferenceVariety;
use crate::models::contact::master::residences::_entities::residences as ContactMasterResidence;
use crate::models::institution::master::institutions::_entities::institutions as InstitutionMasterInstitution;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "building_master", table_name = "buildings")]
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
    pub variety_id: Uuid,
    pub category_id: Uuid,
    pub total_floor: Option<i32>,
    pub residence_id: Uuid,
    pub condition_id: Uuid,
    pub institution_id: Option<Uuid>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Variety,
    Category,
    Residence,
    Condition,
    Institution,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Variety => Entity::belongs_to(BuildingReferenceVariety::Entity)
                .from(Column::VarietyId)
                .to(BuildingReferenceVariety::Column::Id)
                .into(),
            Self::Category => Entity::belongs_to(BuildingReferenceCategory::Entity)
                .from(Column::CategoryId)
                .to(BuildingReferenceCategory::Column::Id)
                .into(),
            Self::Residence => Entity::belongs_to(ContactMasterResidence::Entity)
                .from(Column::ResidenceId)
                .to(ContactMasterResidence::Column::Id)
                .into(),
            Self::Condition => Entity::belongs_to(BuildingReferenceCondition::Entity)
                .from(Column::ConditionId)
                .to(BuildingReferenceCondition::Column::Id)
                .into(),
            Self::Institution => Entity::belongs_to(InstitutionMasterInstitution::Entity)
                .from(Column::InstitutionId)
                .to(InstitutionMasterInstitution::Column::Id)
                .into(),
        }
    }
}

impl Related<BuildingReferenceVariety::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Variety.def()
    }
}

impl Related<BuildingReferenceCategory::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Category.def()
    }
}

impl Related<ContactMasterResidence::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Residence.def()
    }
}

impl Related<BuildingReferenceCondition::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Condition.def()
    }
}

impl Related<InstitutionMasterInstitution::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Institution.def()
    }
}

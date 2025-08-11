use crate::models::institution::master::employees::_entities::employees as InstitutionMasterEmployee;
use crate::models::institution::master::units::_entities::units as InstitutionMasterUnit;
use crate::models::institution::reference::position_types::_entities::position_types as InstitutionReferencePositionType;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "institution_master", table_name = "staffes")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub decree_number: Option<String>,
    pub decree_date: Option<Date>,
    pub start_date: Option<Date>,
    pub end_date: Option<Date>,
    pub employee_id: Uuid,
    pub unit_id: Uuid,
    pub position_type_id: Option<Uuid>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Employee,
    Unit,
    PositionType,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Employee => Entity::belongs_to(InstitutionMasterEmployee::Entity)
                .from(Column::EmployeeId)
                .to(InstitutionMasterEmployee::Column::Id)
                .into(),
            Self::Unit => Entity::belongs_to(InstitutionMasterUnit::Entity)
                .from(Column::UnitId)
                .to(InstitutionMasterUnit::Column::Id)
                .into(),
            Self::PositionType => Entity::belongs_to(InstitutionReferencePositionType::Entity)
                .from(Column::PositionTypeId)
                .to(InstitutionReferencePositionType::Column::Id)
                .into(),
        }
    }
}

impl Related<InstitutionMasterEmployee::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Employee.def()
    }
}

impl Related<InstitutionMasterUnit::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Unit.def()
    }
}

impl Related<InstitutionReferencePositionType::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PositionType.def()
    }
}

use crate::models::institution::master::institutions::_entities::institutions as InstitutionMasterInstitution;
use crate::models::institution::master::staffes::_entities::staffes as InstitutionMasterStaff;
use crate::models::person::master::individuals::_entities::individuals as PersonMasterIndividual;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "institution_master", table_name = "employees")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub institution_id: Uuid,
    pub individual_id: Uuid,
    pub decree_number: Option<String>,
    pub decree_date: Option<Date>,
    pub is_active: bool,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Institution,
    Individual,
    Staffes,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Institution => Entity::belongs_to(InstitutionMasterInstitution::Entity)
                .from(Column::InstitutionId)
                .to(InstitutionMasterInstitution::Column::Id)
                .into(),
            Self::Individual => Entity::belongs_to(PersonMasterIndividual::Entity)
                .from(Column::IndividualId)
                .to(PersonMasterIndividual::Column::Id)
                .into(),
            Self::Staffes => Entity::has_many(InstitutionMasterStaff::Entity)
                .from(Column::Id)
                .to(InstitutionMasterStaff::Column::EmployeeId)
                .into(),
        }
    }
}

impl Related<InstitutionMasterInstitution::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Institution.def()
    }
}

impl Related<PersonMasterIndividual::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Individual.def()
    }
}

impl Related<InstitutionMasterStaff::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Staffes.def()
    }
}

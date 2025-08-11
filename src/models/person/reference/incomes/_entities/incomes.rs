use crate::models::person::master::individuals::_entities::individuals as PersonMasterIndividual;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "person_reference", table_name = "incomes")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(default_value = "0")]
    pub code: i32,
    pub alphabet_code: String,
    pub name: String,
    #[sea_orm(default_value = "0")]
    pub minimum: f64,
    #[sea_orm(default_value = "0")]
    pub maximum: f64,
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: Option<DateTime>,
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    #[sea_orm(default_value = "00000000-0000-0000-0000-000000000000")]
    pub created_by: Option<Uuid>,
    #[sea_orm(default_value = "00000000-0000-0000-0000-000000000000")]
    pub updated_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Individuals,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Individuals => Entity::has_many(PersonMasterIndividual::Entity).into(),
        }
    }
}

// `Related` trait has to be implemented by hand
impl Related<PersonMasterIndividual::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Individuals.def()
    }
}

use crate::models::person::master::individuals::_entities::individuals as PersonMasterIndividual;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "person_master", table_name = "images")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub individual_id: Uuid,
    pub filename: String,
    pub dir: String,
    pub mimetype: Option<String>,
    pub size: Option<i64>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Individual,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Individual => Entity::belongs_to(PersonMasterIndividual::Entity)
                .from(Column::IndividualId)
                .to(PersonMasterIndividual::Column::Id)
                .into(),
        }
    }
}

// `Related` trait has to be implemented by hand
impl Related<PersonMasterIndividual::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Individual.def()
    }
}

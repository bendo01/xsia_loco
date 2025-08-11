use crate::models::location::sub_districts::_entities::sub_districts as LocationSubDistrict;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "location", table_name = "villages")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub sub_district_id: Uuid,
    pub slug: Option<String>,
    pub alt_slug: Option<String>,
    pub state_ministry_code: Option<String>,
    pub state_post_department_code: Option<String>,
    pub state_ministry_name: Option<String>,
    pub dikti_name: Option<String>,
    pub dikti_code: Option<String>,
    #[sea_orm(column_type = "Double", nullable)]
    pub latitude: Option<f64>,
    #[sea_orm(column_type = "Double", nullable)]
    pub longitude: Option<f64>,
    pub zoom: Option<i32>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sync_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    SubDistrict,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::SubDistrict => Entity::belongs_to(LocationSubDistrict::Entity)
                .from(Column::SubDistrictId)
                .to(LocationSubDistrict::Column::Id)
                .into(),
        }
    }
}

// `Related` trait has to be implemented by hand
impl Related<LocationSubDistrict::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SubDistrict.def()
    }
}

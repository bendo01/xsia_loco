use crate::models::academic::candidate::master::candidates::_entities::candidates as AcademicCandidateMasterCandidate;
use crate::models::location::provinces::_entities::provinces as LocationProvince;
use crate::models::location::regency_types::_entities::regency_types as LocationRegencyType;
use crate::models::location::sub_districts::_entities::sub_districts as LocationSubDistrict;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "location", table_name = "regencies")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub code: Option<String>,
    pub name: String,
    pub dikti_code: Option<String>,
    pub epsbed_code: Option<String>,
    pub province_id: Uuid,
    pub regency_type_id: Uuid,
    pub slug: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
    pub alt_slug: Option<String>,
    pub state_ministry_code: Option<String>,
    pub state_ministry_full_code: Option<String>,
    pub state_post_department_code: Option<String>,
    pub state_ministry_name: Option<String>,
    pub dikti_name: Option<String>,
    pub validation_code: Option<String>,
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
    Province,
    RegencyType,
    SubDistricts,
    Candidates,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Province => Entity::belongs_to(LocationProvince::Entity)
                .from(Column::ProvinceId)
                .to(LocationProvince::Column::Id)
                .into(),
            Self::RegencyType => Entity::belongs_to(LocationRegencyType::Entity)
                .from(Column::RegencyTypeId)
                .to(LocationRegencyType::Column::Id)
                .into(),
            Self::SubDistricts => Entity::has_many(LocationSubDistrict::Entity)
                .from(Column::Id)
                .into(),
            Self::Candidates => Entity::has_many(AcademicCandidateMasterCandidate::Entity)
                .from(Column::Id)
                .into(),
        }
    }
}

// `Related` trait has to be implemented by hand
impl Related<LocationProvince::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Province.def()
    }
}

// `Related` trait has to be implemented by hand
impl Related<LocationRegencyType::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RegencyType.def()
    }
}

// `Related` trait has to be implemented by hand
impl Related<AcademicCandidateMasterCandidate::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Candidates.def()
    }
}

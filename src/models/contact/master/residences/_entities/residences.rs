use crate::models::contact::reference::residence_types::_entities::residence_types as ContactReferenceResidenceType;
use crate::models::location::provinces::_entities::provinces as LocationProvince;
use crate::models::location::regencies::_entities::regencies as LocationRegency;
use crate::models::location::sub_districts::_entities::sub_districts as LocationSubDistrict;
use crate::models::location::villages::_entities::villages as LocationVillage;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "contact_master", table_name = "residences")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub street: String,
    pub citizens_association: i32,
    pub neighborhood_association: i32,
    pub province_id: Uuid,
    pub regency_id: Uuid,
    pub sub_district_id: Uuid,
    pub village_id: Uuid,
    pub residence_type_id: Uuid,
    pub residenceable_type: String,
    pub residenceable_id: Uuid,
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
    ResidenceType,
    Province,
    Regency,
    SubDistrict,
    Village,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::ResidenceType => Entity::belongs_to(ContactReferenceResidenceType::Entity)
                .from(Column::ResidenceTypeId)
                .to(ContactReferenceResidenceType::Column::Id)
                .into(),
            Self::Province => Entity::belongs_to(LocationProvince::Entity)
                .from(Column::ProvinceId)
                .to(LocationProvince::Column::Id)
                .into(),
            Self::Regency => Entity::belongs_to(LocationRegency::Entity)
                .from(Column::RegencyId)
                .to(LocationRegency::Column::Id)
                .into(),
            Self::SubDistrict => Entity::belongs_to(LocationSubDistrict::Entity)
                .from(Column::SubDistrictId)
                .to(LocationSubDistrict::Column::Id)
                .into(),
            Self::Village => Entity::belongs_to(LocationVillage::Entity)
                .from(Column::VillageId)
                .to(LocationVillage::Column::Id)
                .into(),
        }
    }
}

impl Related<ContactReferenceResidenceType::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ResidenceType.def()
    }
}

impl Related<LocationProvince::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Province.def()
    }
}
impl Related<LocationRegency::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Regency.def()
    }
}
impl Related<LocationSubDistrict::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SubDistrict.def()
    }
}
impl Related<LocationVillage::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Village.def()
    }
}

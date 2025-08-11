use crate::models::contact::master::residences::_entities::residences as ContactMasterResidence;
use crate::models::contact::reference::residence_types::_entities::residence_types as ContactReferenceResidenceType;
use crate::models::location::provinces::_entities::provinces as LocationProvince;
use crate::models::location::regencies::_entities::regencies as LocationRegency;
use crate::models::location::sub_districts::_entities::sub_districts as LocationSubDistrict;
use crate::models::location::villages::_entities::villages as LocationVillage;
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(clippy::too_many_lines)]
pub struct ContactResidenceDataObject {
    pub residence: ContactMasterResidence::Model,
    pub residence_type: Option<ContactReferenceResidenceType::Model>,
    pub province: Option<LocationProvince::Model>,
    pub regency: Option<LocationRegency::Model>,
    pub sub_district: Option<LocationSubDistrict::Model>,
    pub village: Option<LocationVillage::Model>,
}

impl ContactResidenceDataObject {
    #[allow(clippy::missing_errors_doc)]
    #[allow(clippy::too_many_lines)]
    pub async fn get_by_id(ctx: &AppContext, id: Uuid) -> Result<Option<Self>> {
        let residence = ContactMasterResidence::Entity::find_by_id(id)
            .filter(ContactMasterResidence::Column::DeletedAt.is_null())
            .one(&ctx.db)
            .await?;
        if let Some(residence) = residence {
            let residence_type = residence
                .find_related(ContactReferenceResidenceType::Entity)
                .filter(ContactReferenceResidenceType::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            let province = residence
                .find_related(LocationProvince::Entity)
                .filter(LocationProvince::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            let regency = residence
                .find_related(LocationRegency::Entity)
                .filter(LocationRegency::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            let sub_district = residence
                .find_related(LocationSubDistrict::Entity)
                .filter(LocationSubDistrict::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            let village = residence
                .find_related(LocationVillage::Entity)
                .filter(LocationVillage::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            Ok(Some(Self {
                residence,
                residence_type,
                province,
                regency,
                sub_district,
                village,
            }))
        } else {
            Ok(None)
        }
    }
}

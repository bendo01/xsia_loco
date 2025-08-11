use crate::models::contact::master::residences::_entities::residences as ReferenceModel;
// use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
// use std::borrow::Cow;
use uuid::Uuid;
use validator::Validate;
// use validator::ValidationError;

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct ModelValidator {
    #[validate(length(min = 2, message = "Nama Minimal 2 karakter"))]
    pub street: String,
    #[validate(range(min = 0))]
    pub citizens_association: i32,
    #[validate(range(min = 0))]
    pub neighborhood_association: i32,
    pub province_id: Uuid,
    pub regency_id: Uuid,
    pub sub_district_id: Uuid,
    pub village_id: Uuid,
    // pub residence_type_id: Uuid,
    // pub residenceable_type: String,
    // pub residenceable_id: Uuid,
}

impl ModelValidator {
    #[must_use]
    pub fn from_model(model: &ReferenceModel::Model) -> Self {
        Self {
            street: model.street.clone(),
            citizens_association: model.citizens_association,
            neighborhood_association: model.neighborhood_association,
            province_id: model.province_id,
            regency_id: model.regency_id,
            sub_district_id: model.sub_district_id,
            village_id: model.village_id,
            // residence_type_id: model.residence_type_id.clone(),
            // residenceable_type: model.residenceable_type.clone(),
            // residenceable_id: model.residenceable_id.clone(),
        }
    }

    #[must_use]
    pub fn from_active_model(model: &ReferenceModel::ActiveModel) -> Self {
        Self {
            street: model.street.as_ref().to_owned(),
            citizens_association: *model.citizens_association.as_ref(),
            neighborhood_association: *model.neighborhood_association.as_ref(),
            province_id: model.province_id.as_ref().to_owned(),
            regency_id: model.regency_id.as_ref().to_owned(),
            sub_district_id: model.sub_district_id.as_ref().to_owned(),
            village_id: model.village_id.as_ref().to_owned(),
            // residence_type_id: model.residence_type_id.as_ref().to_owned(),
            // residenceable_type: model.residenceable_type.as_ref().to_owned(),
            // residenceable_id: model.residenceable_id.as_ref().to_owned(),
        }
    }
}

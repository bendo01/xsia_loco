use crate::models::location::countries::_entities::countries as LocationCountry;
use crate::models::location::provinces::_entities::provinces as LocationProvince;
use loco_openapi::prelude::*;
use loco_rs::prelude::*;
// use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(clippy::too_many_lines)]
pub struct LocationProvinceDataObject {
    pub province: LocationProvince::Model,
    pub country: Option<LocationCountry::Model>,
}

/// Response DTO for province API endpoints
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct ProvinceResponse {
    pub id: String,
    pub code: Option<String>,
    pub name: Option<String>,
    pub dikti_code: Option<String>,
    pub epsbed_code: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub alt_slug: Option<String>,
    pub state_ministry_code: Option<String>,
    pub state_ministry_full_code: Option<String>,
    pub state_post_department_code: Option<String>,
    pub state_ministry_name: Option<String>,
    pub dikti_name: Option<String>,
    pub validation_code: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub zoom: Option<i32>,
    pub country_id: Option<String>,
    #[schema(value_type = Option<String>, format = DateTime)]
    pub created_at: Option<DateTime>,
    #[schema(value_type = Option<String>, format = DateTime)]
    pub updated_at: Option<DateTime>,
    #[schema(value_type = Option<String>, format = DateTime)]
    pub sync_at: Option<DateTime>,
    #[schema(value_type = Option<String>, format = DateTime)]
    pub deleted_at: Option<DateTime>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}

impl From<LocationProvince::Model> for ProvinceResponse {
    fn from(model: LocationProvince::Model) -> Self {
        Self {
            id: model.id.to_string(),
            code: model.code,
            name: model.name,
            dikti_code: model.dikti_code,
            epsbed_code: model.epsbed_code,
            slug: model.slug,
            description: model.description,
            alt_slug: model.alt_slug,
            state_ministry_code: model.state_ministry_code,
            state_ministry_full_code: model.state_ministry_full_code,
            state_post_department_code: model.state_post_department_code,
            state_ministry_name: model.state_ministry_name,
            dikti_name: model.dikti_name,
            validation_code: model.validation_code,
            latitude: model.latitude,
            longitude: model.longitude,
            zoom: model.zoom,
            country_id: model.country_id.map(|uuid| uuid.to_string()),
            created_at: model.created_at,
            updated_at: model.updated_at,
            sync_at: model.sync_at,
            deleted_at: model.deleted_at,
            created_by: model.created_by.map(|uuid| uuid.to_string()),
            updated_by: model.updated_by.map(|uuid| uuid.to_string()),
        }
    }
}

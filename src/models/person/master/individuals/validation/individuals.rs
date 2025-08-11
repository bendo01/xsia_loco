use crate::models::person::master::individuals::_entities::individuals as ReferenceModel;
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use validator::{Validate, ValidationError};

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct ModelValidator {
    #[validate(length(
        min = 16,
        max = 16,
        message = "Nomor Kartu Tanda Penduduk wajib angka sepanjang 16 digit"
    ))]
    pub code: String,
    #[validate(length(min = 2, message = "Nama Minimal 2 karakter"))]
    pub name: String,
    #[validate(length(min = 2, message = "Tempat Lahir Minimal 2 karakter"))]
    pub birth_place: String,
    pub birth_date: Date,
    pub gender_id: Uuid,
    pub religion_id: Uuid,
    pub identification_type_id: Uuid,
    pub marital_status_id: Uuid,
    pub profession_id: Uuid,
    pub is_special_need: bool,
    pub is_social_protection_card_recipient: bool,
    pub is_deceased: bool,
    pub occupation_id: Option<Uuid>,
    pub education_id: Option<Uuid>,
    pub income_id: Option<Uuid>,
    pub front_title: Option<String>,
    pub last_title: Option<String>,
    pub age_classification_id: Option<Uuid>,
}

impl ModelValidator {
    #[must_use]
    pub fn from_model(model: &ReferenceModel::Model) -> Self {
        Self {
            code: model.code.clone(),
            name: model.name.clone(),
            birth_place: model.birth_place.clone(),
            birth_date: model.birth_date,
            gender_id: model.gender_id,
            religion_id: model.religion_id,
            identification_type_id: model.identification_type_id,
            marital_status_id: model.marital_status_id,
            profession_id: model.profession_id,
            is_special_need: model.is_special_need,
            is_social_protection_card_recipient: model.is_social_protection_card_recipient,
            is_deceased: model.is_deceased,
            occupation_id: model.occupation_id,
            education_id: model.education_id,
            income_id: model.income_id,
            front_title: model.front_title.clone(),
            last_title: model.last_title.clone(),
            age_classification_id: model.age_classification_id,
        }
    }

    #[must_use]
    pub fn from_active_model(model: &ReferenceModel::ActiveModel) -> Self {
        Self {
            code: model.code.as_ref().to_owned(),
            name: model.name.as_ref().to_owned(),
            birth_place: model.birth_place.as_ref().to_owned(),
            birth_date: model.birth_date.as_ref().to_owned(),
            gender_id: model.gender_id.as_ref().to_owned(),
            religion_id: model.religion_id.as_ref().to_owned(),
            identification_type_id: model.identification_type_id.as_ref().to_owned(),
            marital_status_id: model.marital_status_id.as_ref().to_owned(),
            profession_id: model.profession_id.as_ref().to_owned(),
            is_special_need: model.is_special_need.as_ref().to_owned(),
            is_social_protection_card_recipient: model
                .is_social_protection_card_recipient
                .as_ref()
                .to_owned(),
            is_deceased: model.is_deceased.as_ref().to_owned(),
            occupation_id: model.occupation_id.as_ref().to_owned(),
            education_id: model.education_id.as_ref().to_owned(),
            income_id: model.income_id.as_ref().to_owned(),
            front_title: model.front_title.as_ref().to_owned(),
            last_title: model.last_title.as_ref().to_owned(),
            age_classification_id: model.age_classification_id.as_ref().to_owned(),
        }
    }
}

pub trait ModelValidation {
    fn validate_unique_code(
        &self,
        db: &DatabaseConnection,
        exclude_id: Option<Uuid>,
    ) -> impl std::future::Future<Output = Result<(), ValidationError>> + Send;
}

impl ModelValidation for ModelValidator {
    async fn validate_unique_code(
        &self,
        db: &DatabaseConnection,
        exclude_id: Option<Uuid>,
    ) -> Result<(), ValidationError> {
        let mut query = ReferenceModel::Entity::find()
            .filter(ReferenceModel::Column::DeletedAt.is_null())
            .filter(ReferenceModel::Column::Code.eq(&self.code));

        if let Some(id) = exclude_id {
            query = query.filter(ReferenceModel::Column::Id.ne(id));
        }
        let existing = query.one(db).await;

        match existing {
            Ok(Some(_)) => {
                let mut error = ValidationError::new("kode harus unik");
                error.message = Some(Cow::Borrowed("kode sudah ada"));
                Err(error)
            }
            Ok(None) => Ok(()),
            Err(err) => {
                let mut error = ValidationError::new("error pengaksesan database");
                error.message = Some(Cow::Owned(err.to_string()));
                Err(error)
            }
        }
    }
}

use crate::models::academic::course::reference::varieties::_entities::varieties as ReferenceModel;
use async_trait::async_trait;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct ModelValidator {
    #[validate(range(min = 1, message = "Kode wajib angka bilangan bulat"))]
    pub code: i32,
    #[validate(length(min = 1, message = "Kode Huruf tidak boleh kosong"))]
    pub alphabet_code: String,
    #[validate(length(min = 2, message = "Nama Minimal 2 karakter"))]
    pub name: String,
    pub curriculum_type_id: Option<Uuid>,
    pub start_effective_date: Option<Date>,
    pub end_effective_date: Option<Date>,
}

impl ModelValidator {
    #[must_use]
    pub fn from_model(model: &ReferenceModel::Model) -> Self {
        Self {
            code: model.code,
            alphabet_code: model.alphabet_code.clone(),
            name: model.name.clone(),
            curriculum_type_id: model.curriculum_type_id,
            start_effective_date: model.start_effective_date,
            end_effective_date: model.end_effective_date,
        }
    }

    #[must_use]
    pub fn from_active_model(model: &ReferenceModel::ActiveModel) -> Self {
        Self {
            code: *model.code.as_ref(),
            alphabet_code: model.alphabet_code.as_ref().to_owned(),
            name: model.name.as_ref().to_owned(),
            curriculum_type_id: model.curriculum_type_id.as_ref().to_owned(),
            start_effective_date: model.start_effective_date.as_ref().to_owned(),
            end_effective_date: model.end_effective_date.as_ref().to_owned(),
        }
    }
}

#[async_trait]
pub trait ModelValidation {
    async fn validate_unique_code(&self, db: &DatabaseConnection) -> Result<(), ModelError>;
    async fn validate_unique_alphabet_code(
        &self,
        db: &DatabaseConnection,
    ) -> Result<(), ModelError>;
}

#[async_trait]
impl ModelValidation for ModelValidator {
    async fn validate_unique_code(&self, db: &DatabaseConnection) -> Result<(), ModelError> {
        let existing_category = ReferenceModel::Entity::find()
            .filter(
                model::query::condition()
                    .eq(ReferenceModel::Column::Code, self.code)
                    .build(),
            )
            .one(db)
            .await?;

        if existing_category.is_some() {
            return Err(ModelError::msg("code sudah ada"));
        }

        Ok(())
    }

    async fn validate_unique_alphabet_code(
        &self,
        db: &DatabaseConnection,
    ) -> Result<(), ModelError> {
        let existing_category = ReferenceModel::Entity::find()
            .filter(
                model::query::condition()
                    .eq(ReferenceModel::Column::AlphabetCode, &self.alphabet_code)
                    .build(),
            )
            .one(db)
            .await?;

        if existing_category.is_some() {
            return Err(ModelError::msg("kode huruf sudah ada"));
        }

        Ok(())
    }
}

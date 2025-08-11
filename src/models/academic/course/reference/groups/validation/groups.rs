use crate::models::academic::course::reference::groups::_entities::groups as ReferenceModel;
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
    #[validate(length(min = 1, message = "Singkatan Minimal 1 karakter"))]
    pub abbreviation: String,
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
            abbreviation: model.abbreviation.clone(),
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
            abbreviation: model.abbreviation.as_ref().to_owned(),
            start_effective_date: model.start_effective_date.as_ref().to_owned(),
            end_effective_date: model.end_effective_date.as_ref().to_owned(),
        }
    }
}

#[async_trait]
pub trait ModelValidation {
    async fn validate_unique_code(
        &self,
        db: &DatabaseConnection,
        exclude_id: Option<Uuid>,
    ) -> Result<(), ModelError>;
    async fn validate_unique_alphabet_code(
        &self,
        db: &DatabaseConnection,
        exclude_id: Option<Uuid>,
    ) -> Result<(), ModelError>;
    async fn validate_unique_name(
        &self,
        db: &DatabaseConnection,
        exclude_id: Option<Uuid>,
    ) -> Result<(), ModelError>;
}

#[async_trait]
impl ModelValidation for ModelValidator {
    async fn validate_unique_code(
        &self,
        db: &DatabaseConnection,
        exclude_id: Option<Uuid>,
    ) -> Result<(), ModelError> {
        let mut query = ReferenceModel::Entity::find()
            .filter(ReferenceModel::Column::DeletedAt.is_null())
            .filter(
                model::query::condition()
                    .eq(ReferenceModel::Column::Code, self.code)
                    .build(),
            );

        if let Some(id) = exclude_id {
            query = query.filter(ReferenceModel::Column::Id.ne(id));
        }

        let existing = query.one(db).await?;

        if existing.is_some() {
            return Err(ModelError::msg("code sudah ada"));
        }

        Ok(())
    }

    async fn validate_unique_alphabet_code(
        &self,
        db: &DatabaseConnection,
        exclude_id: Option<Uuid>,
    ) -> Result<(), ModelError> {
        let mut query = ReferenceModel::Entity::find()
            .filter(ReferenceModel::Column::DeletedAt.is_null())
            .filter(
                model::query::condition()
                    .eq(ReferenceModel::Column::AlphabetCode, &self.alphabet_code)
                    .build(),
            );

        if let Some(id) = exclude_id {
            query = query.filter(ReferenceModel::Column::Id.ne(id));
        }

        let existing = query.one(db).await?;

        if existing.is_some() {
            return Err(ModelError::msg("kode huruf sudah ada"));
        }

        Ok(())
    }

    async fn validate_unique_name(
        &self,
        db: &DatabaseConnection,
        exclude_id: Option<Uuid>,
    ) -> Result<(), ModelError> {
        let mut query = ReferenceModel::Entity::find()
            .filter(ReferenceModel::Column::DeletedAt.is_null())
            .filter(
                model::query::condition()
                    .eq(ReferenceModel::Column::Name, &self.name)
                    .build(),
            );
        if let Some(id) = exclude_id {
            query = query.filter(ReferenceModel::Column::Id.ne(id));
        }
        let existing_record = query.one(db).await?;

        if existing_record.is_some() {
            return Err(ModelError::msg("nama sudah ada"));
        }

        Ok(())
    }
}

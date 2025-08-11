use crate::models::academic::survey::reference::question_varieties::_entities::question_varieties as ReferenceModel;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use validator::Validate;
use validator::ValidationError;

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct ModelValidator {
    #[validate(range(min = 1, message = "Kode wajib angka bilangan bulat"))]
    pub code: i32,

    #[validate(length(min = 1, message = "Kode Huruf tidak boleh kosong"))]
    pub alphabet_code: String,

    #[validate(length(min = 2, message = "Nama Minimal 2 karakter"))]
    pub name: String,
}

impl ModelValidator {
    #[must_use]
    pub fn from_model(model: &ReferenceModel::Model) -> Self {
        Self {
            code: model.code,
            alphabet_code: model.alphabet_code.clone(),
            name: model.name.clone(),
        }
    }

    #[must_use]
    pub fn from_active_model(model: &ReferenceModel::ActiveModel) -> Self {
        Self {
            code: *model.code.as_ref(),
            alphabet_code: model.alphabet_code.as_ref().to_owned(),
            name: model.name.as_ref().to_owned(),
        }
    }
}

pub trait ModelValidation {
    fn validate_unique_code(
        &self,
        db: &DatabaseConnection,
        exclude_id: Option<Uuid>,
    ) -> impl std::future::Future<Output = Result<(), ValidationError>> + Send;
    fn validate_unique_alphabet_code(
        &self,
        db: &DatabaseConnection,
        exclude_id: Option<Uuid>,
    ) -> impl std::future::Future<Output = Result<(), ValidationError>> + Send;
    fn validate_unique_name(
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
            .filter(
                model::query::condition()
                    .eq(ReferenceModel::Column::Code, self.code)
                    .build(),
            );

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

    async fn validate_unique_alphabet_code(
        &self,
        db: &DatabaseConnection,
        exclude_id: Option<Uuid>,
    ) -> Result<(), ValidationError> {
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

        let existing = query.one(db).await;

        match existing {
            Ok(Some(_)) => {
                let mut error = ValidationError::new("kode huruf harus unik");
                error.message = Some(Cow::Borrowed("kode huruf sudah ada"));
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

    async fn validate_unique_name(
        &self,
        db: &DatabaseConnection,
        exclude_id: Option<Uuid>,
    ) -> Result<(), ValidationError> {
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
        let existing = query.one(db).await;

        match existing {
            Ok(Some(_)) => {
                let mut error = ValidationError::new("nama harus unik");
                error.message = Some(Cow::Borrowed("nama sudah ada"));
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

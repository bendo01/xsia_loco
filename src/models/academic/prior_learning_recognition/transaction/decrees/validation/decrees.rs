use crate::models::academic::prior_learning_recognition::reference::evaluator_types::_entities::evaluator_types as ReferenceModel;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use validator::Validate;
use validator::ValidationError;

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct ModelValidator {
    pub decree_number: Uuid,
    pub decree_date: Date,
    pub evaluation_id: Uuid,
}

impl ModelValidator {
    #[must_use]
    pub fn from_model(model: &ReferenceModel::Model) -> Self {
        Self {
            decree_number: model.decree_number.clone(),
            decree_date: model.decree_date.clone(),
            evaluation_id: model.evaluation_id.clone(),
        }
    }

    #[must_use]
    pub fn from_active_model(model: &ReferenceModel::ActiveModel) -> Self {
        Self {
            decree_number: model.decree_number.as_ref().to_owned(),
            decree_date: model.decree_date.as_ref().to_owned(),
            evaluation_id: model.evaluation_id.as_ref().to_owned(),
        }
    }
}

pub trait ModelValidation {
    fn validate_unique_decree_number(
        &self,
        db: &DatabaseConnection,
        exclude_id: Option<Uuid>,
    ) -> impl std::future::Future<Output = Result<(), ValidationError>> + Send;
}

impl ModelValidation for ModelValidator {
    async fn validate_unique_decree_number(
        &self,
        db: &DatabaseConnection,
        exclude_id: Option<Uuid>,
    ) -> Result<(), ValidationError> {
        let mut query = ReferenceModel::Entity::find()
            .filter(ReferenceModel::Column::DeletedAt.is_null())
            .filter(
                model::query::condition()
                    .eq(ReferenceModel::Column::DecreeNumber, self.decree_number)
                    .build(),
            );

        if let Some(id) = exclude_id {
            query = query.filter(ReferenceModel::Column::Id.ne(id));
        }

        let existing = query.one(db).await;

        match existing {
            Ok(Some(_)) => {
                let mut error = ValidationError::new("Nomor Surat harus unik");
                error.message = Some(Cow::Borrowed("Nomor Surat sudah ada"));
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

use crate::models::academic::prior_learning_recognition::reference::evaluation_details::_entities::evaluation_details as ReferenceModel;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use validator::Validate;
use validator::ValidationError;

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct ModelValidator {
    pub individual_id: Uuid,
    pub evaluator_type_id: Uuid,
    pub recognition_id: Uuid,
}

impl ModelValidator {
    #[must_use]
    pub fn from_model(model: &ReferenceModel::Model) -> Self {
        Self {
            individual_id: model.individual_id.clone(),
            evaluator_type_id: model.evaluator_type_id.clone(),
            recognition_id: model.recognition_id.clone(),
        }
    }

    #[must_use]
    pub fn from_active_model(model: &ReferenceModel::ActiveModel) -> Self {
        Self {
            individual_id: model.individual_id.as_ref().to_owned(),
            evaluator_type_id: model.evaluator_type_id.as_ref().to_owned(),
            recognition_id: model.recognition_id.as_ref().to_owned(),
        }
    }
}

pub trait ModelValidation {
    fn validate_unique_data(
        &self,
        db: &DatabaseConnection,
        exclude_id: Option<Uuid>,
    ) -> impl std::future::Future<Output = Result<(), ValidationError>> + Send;
}

impl ModelValidation for ModelValidator {
    async fn validate_unique_data(
        &self,
        db: &DatabaseConnection,
        exclude_id: Option<Uuid>,
    ) -> Result<(), ValidationError> {
        let mut query = ReferenceModel::Entity::find()
            .filter(ReferenceModel::Column::DeletedAt.is_null())
            .filter(
                model::query::condition()
                    .eq(ReferenceModel::Column::IndividualId, self.individual_id)
                    .build(),
            )
            .filter(
                model::query::condition()
                    .eq(ReferenceModel::Column::EvaluatorTypeId, self.evaluator_type_id)
                    .build(),
            )
            .filter(
                model::query::condition()
                    .eq(ReferenceModel::Column::RecognitionId, self.recognition_id)
                    .build(),
            );

        if let Some(id) = exclude_id {
            query = query.filter(ReferenceModel::Column::Id.ne(id));
        }

        let existing = query.one(db).await;

        match existing {
            Ok(Some(_)) => {
                let mut error = ValidationError::new("Data harus unik");
                error.message = Some(Cow::Borrowed("Data sudah ada"));
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
use crate::models::academic::prior_learning_recognition::transaction::recognitions::_entities::recognitions as ReferenceModel;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
// use uuid::uuid;
use validator::Validate;
use validator::ValidationError;

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct ModelValidator {
    // #[validate(length(min = 1, message = "Kode Huruf tidak boleh kosong"))]
    // pub name: String,
    pub candidate_id: Uuid,
    pub unit_id: Uuid,
    // pub curriculum_id: Uuid,
}

impl ModelValidator {
    #[must_use]
    pub fn from_model(model: &ReferenceModel::Model) -> Self {
        Self {
            // name: model.name.clone(),
            candidate_id: model.candidate_id.clone(),
            unit_id: model.unit_id.clone(),
            // curriculum_id: model.curriculum_id.clone(),
        }
    }

    #[must_use]
    pub fn from_active_model(model: &ReferenceModel::ActiveModel) -> Self {
        Self {
            // name: model.name.as_ref().to_owned(),
            candidate_id: model.candidate_id.as_ref().to_owned(),
            unit_id: model.unit_id.as_ref().to_owned(),
            // curriculum_id: model.curriculum_id.as_ref().to_owned(),
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
        // let candidate_id = uuid!(&self.candidate_id);
        // let unit_id = uuid!(&self.unit_id);
        let mut query = ReferenceModel::Entity::find()
            .filter(ReferenceModel::Column::DeletedAt.is_null())
            .filter(
                model::query::condition()
                    .eq(ReferenceModel::Column::CandidateId, self.candidate_id)
                    .build(),
            )
            .filter(
                model::query::condition()
                    .eq(ReferenceModel::Column::UnitId, self.unit_id)
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

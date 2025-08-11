use crate::models::auth::roles::_entities::roles as ReferenceModel;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use validator::Validate;
use validator::ValidationError;

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct ModelValidator {
    #[validate(length(min = 2, message = "Nama Minimal 2 karakter"))]
    pub name: String,
    pub user_id: Uuid,
    pub position_type_id: Uuid,
    pub roleable_id: Uuid,
    pub roleable_type: String,
}

impl ModelValidator {
    #[must_use]
    pub fn from_model(model: &ReferenceModel::Model) -> Self {
        Self {
            name: model.name.clone(),
            user_id: model.user_id,
            position_type_id: model.position_type_id,
            roleable_id: model.roleable_id,
            roleable_type: model.roleable_type.clone(),
        }
    }

    #[must_use]
    pub fn from_active_model(model: &ReferenceModel::ActiveModel) -> Self {
        Self {
            name: model.name.as_ref().to_owned(),
            user_id: model.user_id.as_ref().to_owned(),
            position_type_id: model.position_type_id.as_ref().to_owned(),
            roleable_id: model.roleable_id.as_ref().to_owned(),
            roleable_type: model.roleable_type.as_ref().to_owned(),
        }
    }
}

pub trait ModelValidation {
    fn validate_unique_name(
        &self,
        db: &DatabaseConnection,
        exclude_id: Option<Uuid>,
    ) -> impl std::future::Future<Output = Result<(), ValidationError>> + Send;
}

impl ModelValidation for ModelValidator {
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

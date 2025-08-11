use async_trait::async_trait;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::auth::users::_entities::users as AuthUser;

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct UserValidator {
    #[validate(length(min = 2, message = "Name must be at least 2 characters long"))]
    pub name: String,

    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: Option<String>,
}

impl UserValidator {
    #[must_use]
    pub fn from_model(model: &AuthUser::Model) -> Self {
        Self {
            name: model.name.clone(),
            email: model.email.clone(),
            password: None, // Password is not validated from existing model
        }
    }

    #[must_use]
    pub fn from_active_model(model: &AuthUser::ActiveModel) -> Self {
        Self {
            name: model.name.as_ref().to_owned(),
            email: model.email.as_ref().to_owned(),
            password: None, // Password not needed for validation from active model
        }
    }
}

#[async_trait]
pub trait UserValidation {
    async fn validate_unique_email(&self, db: &DatabaseConnection) -> Result<(), ModelError>;
}

#[async_trait]
impl UserValidation for UserValidator {
    async fn validate_unique_email(&self, db: &DatabaseConnection) -> Result<(), ModelError> {
        // Check if email already exists in database
        let existing_user = AuthUser::Entity::find()
            .filter(
                model::query::condition()
                    .eq(AuthUser::Column::Email, &self.email)
                    .build(),
            )
            .one(db)
            .await?;

        if existing_user.is_some() {
            return Err(ModelError::msg("Email already exists"));
        }

        Ok(())
    }
}

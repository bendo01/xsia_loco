use async_trait::async_trait;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::location::continents::_entities::continents::{self, ActiveModel, Model};

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct ContinentValidator {
    pub code: Option<i32>,

    #[validate(length(min = 1, message = "Alphabet code cannot be empty"))]
    pub alphabet_code: String,

    #[validate(length(min = 2, message = "Name must be at least 2 characters long"))]
    pub name: String,

    #[validate(length(min = 2, message = "Slug must be at least 2 characters long"))]
    pub slug: Option<String>,
}

impl ContinentValidator {
    #[must_use]
    pub fn from_model(model: &Model) -> Self {
        Self {
            code: model.code,
            alphabet_code: model.alphabet_code.clone(),
            name: model.name.clone(),
            slug: model.slug.clone(),
        }
    }

    #[must_use]
    pub fn from_active_model(model: &ActiveModel) -> Self {
        Self {
            code: model.code.as_ref().to_owned(),
            alphabet_code: model.alphabet_code.as_ref().to_owned(),
            name: model.name.as_ref().to_owned(),
            slug: model.slug.as_ref().to_owned(),
        }
    }
}

#[async_trait]
pub trait ContinentValidation {
    async fn validate_unique_alphabet_code(
        &self,
        db: &DatabaseConnection,
    ) -> Result<(), ModelError>;
    async fn validate_unique_name(&self, db: &DatabaseConnection) -> Result<(), ModelError>;
}

#[async_trait]
impl ContinentValidation for ContinentValidator {
    async fn validate_unique_alphabet_code(
        &self,
        db: &DatabaseConnection,
    ) -> Result<(), ModelError> {
        let existing_continent = continents::Entity::find()
            .filter(
                model::query::condition()
                    .eq(continents::Column::AlphabetCode, &self.alphabet_code)
                    .build(),
            )
            .one(db)
            .await?;

        if existing_continent.is_some() {
            return Err(ModelError::msg("Continent alphabet code already exists"));
        }

        Ok(())
    }

    async fn validate_unique_name(&self, db: &DatabaseConnection) -> Result<(), ModelError> {
        let existing_continent = continents::Entity::find()
            .filter(
                model::query::condition()
                    .eq(continents::Column::Name, &self.name)
                    .build(),
            )
            .one(db)
            .await?;

        if existing_continent.is_some() {
            return Err(ModelError::msg("Continent name already exists"));
        }

        Ok(())
    }
}

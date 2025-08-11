use async_trait::async_trait;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use crate::models::person::reference::incomes::_entities::incomes::{self, ActiveModel, Model};

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct IncomeValidator {
    #[validate(range(min = 0, message = "Code cannot be negative"))]
    pub code: i32,

    #[validate(length(min = 1, message = "Alphabet code cannot be empty"))]
    pub alphabet_code: String,

    #[validate(length(min = 2, message = "Name must be at least 2 characters long"))]
    pub name: String,

    #[validate(range(min = 0.0, message = "Minimum income cannot be negative"))]
    pub minimum: f64,

    #[validate(range(min = 0.0, message = "Maximum income cannot be negative"))]
    pub maximum: f64,
}

impl IncomeValidator {
    #[must_use]
    pub fn from_model(model: &Model) -> Self {
        Self {
            code: model.code,
            alphabet_code: model.alphabet_code.clone(),
            name: model.name.clone(),
            minimum: model.minimum,
            maximum: model.maximum,
        }
    }

    #[must_use]
    pub fn from_active_model(model: &ActiveModel) -> Self {
        Self {
            code: *model.code.as_ref(),
            alphabet_code: model.alphabet_code.as_ref().to_owned(),
            name: model.name.as_ref().to_owned(),
            minimum: *model.minimum.as_ref(),
            maximum: *model.maximum.as_ref(),
        }
    }

    /// Validates that the minimum income is not greater than the maximum income.
    ///
    /// # Errors
    ///
    /// Returns a `ValidationError` with code `"minimum_exceeds_maximum"` if the minimum
    /// income is greater than the maximum income.
    pub fn validate_income_range(&self) -> Result<(), ValidationError> {
        if self.minimum > self.maximum {
            return Err(ValidationError::new("minimum_exceeds_maximum"));
        }
        Ok(())
    }
}

#[async_trait]
pub trait IncomeValidation {
    async fn validate_unique_code(&self, db: &DatabaseConnection) -> Result<(), ModelError>;
    async fn validate_income_consistency(&self) -> Result<(), ModelError>;
}

#[async_trait]
impl IncomeValidation for IncomeValidator {
    async fn validate_unique_code(&self, db: &DatabaseConnection) -> Result<(), ModelError> {
        let existing_income = incomes::Entity::find()
            .filter(
                model::query::condition()
                    .eq(incomes::Column::Code, self.code)
                    .build(),
            )
            .one(db)
            .await?;

        if existing_income.is_some() {
            return Err(ModelError::msg("Income code already exists"));
        }

        Ok(())
    }

    async fn validate_income_consistency(&self) -> Result<(), ModelError> {
        if self.minimum > self.maximum {
            return Err(ModelError::msg(
                "Minimum income cannot be greater than maximum income",
            ));
        }

        Ok(())
    }
}

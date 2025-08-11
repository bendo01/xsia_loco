use crate::models::person::reference::age_classifications::_entities::age_classifications::{
    Column, Entity,
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use std::borrow::Cow;
use uuid::Uuid;
use validator::ValidationError;

// Custom validator function to check uniqueness of the 'code' column
#[allow(clippy::missing_errors_doc)]
pub async fn validate_unique_code(
    code: &i32,
    db: &DatabaseConnection,
    exclude_id: Option<Uuid>,
) -> Result<(), ValidationError> {
    let mut query = Entity::find()
        .filter(Column::DeletedAt.is_null())
        .filter(Column::Code.eq(*code));
    if let Some(id) = exclude_id {
        query = query.filter(Column::Id.ne(id));
    }
    let existing_record = query.one(db).await;

    match existing_record {
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

#[allow(clippy::missing_errors_doc)]
pub async fn validate_unique_alphabet_code(
    alphabet_code: &str,
    db: &DatabaseConnection,
    exclude_id: Option<Uuid>,
) -> Result<(), ValidationError> {
    let mut query = Entity::find()
        .filter(Column::DeletedAt.is_null())
        .filter(Column::AlphabetCode.eq(alphabet_code));
    if let Some(id) = exclude_id {
        query = query.filter(Column::Id.ne(id));
    }
    let existing_record = query.one(db).await;

    match existing_record {
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

// Custom validator function to check uniqueness of the 'code' column
#[allow(clippy::missing_errors_doc)]
pub async fn validate_unique_name(
    name: &str,
    db: &DatabaseConnection,
    exclude_id: Option<Uuid>,
) -> Result<(), ValidationError> {
    let mut query = Entity::find()
        .filter(Column::DeletedAt.is_null())
        .filter(Column::Name.eq(name));
    if let Some(id) = exclude_id {
        query = query.filter(Column::Id.ne(id));
    }
    let existing_record = query.one(db).await;

    match existing_record {
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

#[allow(clippy::missing_errors_doc)]
pub fn validate_minimum(minimum: &i32) -> Result<(), ValidationError> {
    if *minimum < 0 {
        let mut error = ValidationError::new("minimum tidak boleh negatif");
        error.message = Some(Cow::Borrowed(
            "minimum harus lebih besar atau sama dengan 0",
        ));
        return Err(error);
    }
    Ok(())
}

#[allow(clippy::missing_errors_doc)]
pub fn validate_min_max(minimum: &i32, maximum: &Option<i32>) -> Result<(), ValidationError> {
    if let Some(max) = maximum {
        if max < minimum {
            let mut error =
                ValidationError::new("maximum harus lebih besar atau sama dengan minimum");
            error.message = Some(Cow::Borrowed(
                "maximum tidak boleh lebih kecil dari minimum",
            ));
            return Err(error);
        }
    }
    Ok(())
}

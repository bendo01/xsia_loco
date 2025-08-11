// use crate::common::settings::Settings;
use crate::models::academic::student::master::students::_entities::students as AcademicStudentMasterStudent;
use crate::models::auth::users::_entities::users as AuthUser;
use crate::models::person::master::individuals::_entities::individuals as PersonMasterIndividual;
use loco_rs::prelude::*;
// use sea_orm::{DatabaseConnection, DbErr, EntityTrait, QueryOrder};
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
// use uuid::Uuid;
use validator::{Validate, ValidationError};

#[derive(Clone, Debug, Validate, Deserialize, Serialize)]
pub struct ClaimUserValidator {
    #[validate(length(min = 8, message = "Kata Sandi Minimal 8 Karakter"))]
    pub password: String,
    #[validate(email)]
    pub email: String,
    pub student_code: String,
    #[validate(length(
        min = 16,
        max = 16,
        message = "Nomor Induk Kependudukan wajib 16 karakter"
    ))]
    pub individual_code: String,
}

pub trait ClaimUserValidation {
    fn validate_unique_email(
        &self,
        db: &DatabaseConnection,
    ) -> impl std::future::Future<Output = Result<(), ValidationError>> + Send;
    fn validate_student_exists(
        &self,
        db: &DatabaseConnection,
    ) -> impl std::future::Future<Output = Result<(), ValidationError>> + Send;
    fn validate_individual_exists(
        &self,
        db: &DatabaseConnection,
    ) -> impl std::future::Future<Output = Result<(), ValidationError>> + Send;

    fn validate_individual_related_to_student(
        &self,
        db: &DatabaseConnection,
    ) -> impl std::future::Future<Output = Result<(), ValidationError>> + Send;

    fn validate_individual_related_to_user(
        &self,
        db: &DatabaseConnection,
    ) -> impl std::future::Future<Output = Result<(), ValidationError>> + Send;
}

impl ClaimUserValidation for ClaimUserValidator {
    async fn validate_unique_email(&self, db: &DatabaseConnection) -> Result<(), ValidationError> {
        // Check if email already exists in database
        let existing_user = AuthUser::Entity::find()
            .filter(AuthUser::Column::DeletedAt.is_null())
            .filter(AuthUser::Column::Email.eq(&self.email))
            .one(db)
            .await;

        match existing_user {
            Ok(Some(_)) => {
                let mut error = ValidationError::new("email harus unik");
                error.message = Some(Cow::Borrowed("email sudah ada"));
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

    async fn validate_student_exists(
        &self,
        db: &DatabaseConnection,
    ) -> Result<(), ValidationError> {
        let existing_student = AcademicStudentMasterStudent::Entity::find()
            .filter(AcademicStudentMasterStudent::Column::DeletedAt.is_null())
            .filter(AcademicStudentMasterStudent::Column::Code.eq(&self.student_code))
            .one(db)
            .await;

        match existing_student {
            Ok(Some(_)) => Ok(()),
            Ok(None) => {
                let mut error = ValidationError::new("mahasiswa tidak ditemukan");
                error.message = Some(Cow::Borrowed("mahasiswa tidak ditemukan"));
                Err(error)
            }
            Err(err) => {
                let mut error = ValidationError::new("error pengaksesan database");
                error.message = Some(Cow::Owned(err.to_string()));
                Err(error)
            }
        }
    }

    async fn validate_individual_exists(
        &self,
        db: &DatabaseConnection,
    ) -> Result<(), ValidationError> {
        let existing_individual = PersonMasterIndividual::Entity::find()
            .filter(PersonMasterIndividual::Column::DeletedAt.is_null())
            .filter(PersonMasterIndividual::Column::Code.eq(&self.individual_code))
            .one(db)
            .await;

        match existing_individual {
            Ok(Some(_)) => Ok(()),
            Ok(None) => {
                let mut error = ValidationError::new("individual tidak ditemukan");
                error.message = Some(Cow::Borrowed("individual tidak ditemukan"));
                Err(error)
            }
            Err(err) => {
                let mut error = ValidationError::new("error pengaksesan database");
                error.message = Some(Cow::Owned(err.to_string()));
                Err(error)
            }
        }
    }

    async fn validate_individual_related_to_student(
        &self,
        db: &DatabaseConnection,
    ) -> Result<(), ValidationError> {
        let existing_individual = PersonMasterIndividual::Entity::find()
            .filter(PersonMasterIndividual::Column::DeletedAt.is_null())
            .filter(PersonMasterIndividual::Column::Code.eq(&self.individual_code))
            .one(db)
            .await;

        match existing_individual {
            Ok(Some(individual)) => {
                let existing_student = AcademicStudentMasterStudent::Entity::find()
                    .filter(AcademicStudentMasterStudent::Column::DeletedAt.is_null())
                    .filter(AcademicStudentMasterStudent::Column::Code.eq(&self.student_code))
                    .one(db)
                    .await;

                match existing_student {
                    Ok(Some(student)) => {
                        if individual.id == student.individual_id {
                            Ok(())
                        } else {
                            let mut error =
                                ValidationError::new("individual tidak sesuai dengan mahasiswa");
                            error.message =
                                Some(Cow::Borrowed("individual tidak sesuai dengan mahasiswa"));
                            Err(error)
                        }
                    }
                    Ok(None) => {
                        let mut error = ValidationError::new("mahasiswa tidak ditemukan");
                        error.message = Some(Cow::Borrowed("mahasiswa tidak ditemukan"));
                        Err(error)
                    }
                    Err(err) => {
                        let mut error = ValidationError::new("error pengaksesan database");
                        error.message = Some(Cow::Owned(err.to_string()));
                        Err(error)
                    }
                }
            }
            Ok(None) => {
                let mut error = ValidationError::new("individual tidak ditemukan");
                error.message = Some(Cow::Borrowed("individual tidak ditemukan"));
                Err(error)
            }
            Err(err) => {
                let mut error = ValidationError::new("error pengaksesan database");
                error.message = Some(Cow::Owned(err.to_string()));
                Err(error)
            }
        }
    }

    async fn validate_individual_related_to_user(
        &self,
        db: &DatabaseConnection,
    ) -> Result<(), ValidationError> {
        let existing_individual = PersonMasterIndividual::Entity::find()
            .filter(PersonMasterIndividual::Column::DeletedAt.is_null())
            .filter(PersonMasterIndividual::Column::Code.eq(&self.individual_code))
            .one(db)
            .await;

        match existing_individual {
            Ok(Some(individual)) => {
                let existing_user = AuthUser::Entity::find()
                    .filter(AuthUser::Column::DeletedAt.is_null())
                    .filter(AuthUser::Column::IndividualId.eq(individual.id))
                    .one(db)
                    .await;

                match existing_user {
                    Ok(Some(user)) => {
                        if individual.id == user.individual_id {
                            let mut error = ValidationError::new("individual sudah mempunyai akun");
                            error.message = Some(Cow::Borrowed("individual sudah mempunyai akun"));
                            Err(error)
                        } else {
                            Ok(())
                        }
                    }
                    Ok(None) => Ok(()),
                    Err(err) => {
                        let mut error = ValidationError::new("error pengaksesan database akun");
                        error.message = Some(Cow::Owned(err.to_string()));
                        Err(error)
                    }
                }
            }
            Ok(None) => {
                let mut error = ValidationError::new("individual tidak ditemukan");
                error.message = Some(Cow::Borrowed("individual tidak ditemukan"));
                Err(error)
            }
            Err(err) => {
                let mut error = ValidationError::new("error pengaksesan database individual");
                error.message = Some(Cow::Owned(err.to_string()));
                Err(error)
            }
        }
    }
}

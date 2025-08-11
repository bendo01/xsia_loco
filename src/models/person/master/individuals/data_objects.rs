use crate::models::academic::lecturer::master::lecturers::_entities::lecturers as AcademicLecturerMasterLecturer;
use crate::models::academic::student::master::students::_entities::students as AcademicStudentMasterStudent;
use crate::models::institution::master::employees::_entities::employees as InstitutionMasterEmployee;
use crate::models::person::master::biodatas::_entities::biodatas as PersonMasterBiodata;
use crate::models::person::master::family_card_members::_entities::family_card_members as PersonMasterFamilyCardMember;
use crate::models::person::master::images::_entities::images as PersonMasterImage;
use crate::models::person::master::individuals::_entities::individuals as PersonMasterIndividual;
use crate::models::person::reference::age_classifications::_entities::age_classifications as PersonReferenceAgeClassification;
use crate::models::person::reference::genders::_entities::genders as PersonReferenceGender;
use crate::models::person::reference::identification_types::_entities::identification_types as PersonReferenceIdentificationType;
use crate::models::person::reference::incomes::_entities::incomes as PersonReferenceIncome;
use crate::models::person::reference::marital_statuses::_entities::marital_statuses as PersonReferenceMaritalStatus;
use crate::models::person::reference::occupations::_entities::occupations as PersonReferenceOccupation;
use crate::models::person::reference::professions::_entities::professions as PersonReferenceProfession;
use crate::models::person::reference::religions::_entities::religions as PersonReferenceReligion;
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(clippy::too_many_lines)]
pub struct DataObject {
    pub individual: PersonMasterIndividual::Model,
    pub gender: Option<PersonReferenceGender::Model>,
    pub religion: Option<PersonReferenceReligion::Model>,
    pub identification_type: Option<PersonReferenceIdentificationType::Model>,
    pub income: Option<PersonReferenceIncome::Model>,
    pub marital_status: Option<PersonReferenceMaritalStatus::Model>,
    pub occupation: Option<PersonReferenceOccupation::Model>,
    pub profession: Option<PersonReferenceProfession::Model>,
    pub age_classification: Option<PersonReferenceAgeClassification::Model>,
    pub biodata: Option<PersonMasterBiodata::Model>,
    pub picture: Option<PersonMasterImage::Model>,
    pub lecturer: Option<AcademicLecturerMasterLecturer::Model>,
    pub students: Option<Vec<AcademicStudentMasterStudent::Model>>,
    pub employees: Option<Vec<InstitutionMasterEmployee::Model>>,
    pub family_card_members: Option<Vec<PersonMasterFamilyCardMember::Model>>,
}

impl DataObject {
    #[allow(clippy::missing_errors_doc)]
    #[allow(clippy::too_many_lines)]
    pub async fn get_by_id(
        ctx: &AppContext,
        id: Uuid,
        with_has_many_relationships: bool,
    ) -> Result<Option<Self>> {
        let individual = PersonMasterIndividual::Entity::find_by_id(id)
            .filter(PersonMasterIndividual::Column::DeletedAt.is_null())
            .one(&ctx.db)
            .await?;
        if let Some(individual) = individual {
            let age_classification = individual
                .find_related(PersonReferenceAgeClassification::Entity)
                .filter(PersonReferenceAgeClassification::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;
            let gender = individual
                .find_related(PersonReferenceGender::Entity)
                .filter(PersonReferenceGender::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;
            let religion = individual
                .find_related(PersonReferenceReligion::Entity)
                .filter(PersonReferenceReligion::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;
            let identification_type = individual
                .find_related(PersonReferenceIdentificationType::Entity)
                .filter(PersonReferenceIdentificationType::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;
            let income = individual
                .find_related(PersonReferenceIncome::Entity)
                .filter(PersonReferenceIncome::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;
            let marital_status = individual
                .find_related(PersonReferenceMaritalStatus::Entity)
                .filter(PersonReferenceMaritalStatus::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;
            let occupation = individual
                .find_related(PersonReferenceOccupation::Entity)
                .filter(PersonMasterIndividual::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;
            let profession = individual
                .find_related(PersonReferenceProfession::Entity)
                .filter(PersonReferenceProfession::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;

            let biodata = individual
                .find_related(PersonMasterBiodata::Entity)
                .filter(PersonMasterBiodata::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;
            let picture = individual
                .find_related(PersonMasterImage::Entity)
                .filter(PersonMasterImage::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;
            let lecturer = individual
                .find_related(AcademicLecturerMasterLecturer::Entity)
                .filter(AcademicLecturerMasterLecturer::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await?;
            // Conditionally load has-many relationships
            let students = if with_has_many_relationships {
                let list = individual
                    .find_related(AcademicStudentMasterStudent::Entity)
                    .filter(AcademicStudentMasterStudent::Column::DeletedAt.is_null())
                    .all(&ctx.db)
                    .await?;
                if list.is_empty() { None } else { Some(list) }
            } else {
                None
            };

            let employees = if with_has_many_relationships {
                let list = individual
                    .find_related(InstitutionMasterEmployee::Entity)
                    .filter(InstitutionMasterEmployee::Column::DeletedAt.is_null())
                    .all(&ctx.db)
                    .await?;
                if list.is_empty() { None } else { Some(list) }
            } else {
                None
            };

            let family_card_members = if with_has_many_relationships {
                let list = individual
                    .find_related(PersonMasterFamilyCardMember::Entity)
                    .filter(PersonMasterFamilyCardMember::Column::DeletedAt.is_null())
                    .all(&ctx.db)
                    .await?;
                if list.is_empty() { None } else { Some(list) }
            } else {
                None
            };

            Ok(Some(Self {
                individual,
                gender,
                religion,
                identification_type,
                income,
                marital_status,
                occupation,
                profession,
                age_classification,
                biodata,
                picture,
                lecturer,
                students,
                employees,
                family_card_members,
            }))
        } else {
            Ok(None)
        }
    }
}

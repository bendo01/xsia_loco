use crate::models::academic::candidate::master::candidates::_entities::candidates as AcademicCandidateMasterCandidate;
use crate::models::academic::candidate::reference::registration_types::_entities::registration_types as AcademicCandidateReferenceRegistrationType;
use crate::models::academic::prior_learning_recognition::transaction::recognitions::_entities::recognitions as AcademicPriorLearningRecognitionTransactionRecognition;
use crate::models::academic::student::reference::registrations::_entities::registrations as AcademicStudentReferenceRegistration;
use crate::models::auth::users::_entities::users as AuthUser;
use crate::models::contact::master::residences::_entities::residences as ContactMasterResidence;
use crate::models::person::master::family_card_members::_entities::family_card_members as PersonMasterFamilyMember;
use crate::models::person::master::family_cards::_entities::family_cards as PersonMasterFamilyCard;
use crate::models::person::master::individuals::_entities::individuals as PersonMasterIndividual;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::{Uuid, uuid};

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Deserialize, Serialize)]
pub struct CandidateValidate {
    pub is_profile_exist: bool,
    pub is_family_card_exist: bool,
    pub is_mother_exist: bool,
    pub is_father_exist: bool,
    pub is_address_exist: bool,
    pub is_guardian_exist: bool,
    pub registration_id: Uuid,
    pub candidate_id: Uuid,
    pub recognition_id: Uuid,
}

impl CandidateValidate {
    /// Checks and validates different aspects of a candidate's profile
    ///
    /// # Arguments
    /// * `id` - The UUID of the candidate to validate
    /// * `ctx` - The application context containing the database connection
    ///
    /// # Returns
    /// A `CandidateValidate` struct indicating which components of the candidate's profile exist
    ///
    /// # Errors
    /// This function may return an error in the following cases:
    /// * Database connection errors
    /// * Query execution errors
    /// * Invalid data retrieval errors
    pub async fn check(ctx: &AppContext, id: Uuid) -> Result<Self, Error> {
        let mut validate = Self {
            is_profile_exist: false,
            is_family_card_exist: false,
            is_mother_exist: false,
            is_father_exist: false,
            is_address_exist: false,
            is_guardian_exist: false,
            registration_id: uuid!("00000000-0000-0000-0000-000000000000"),
            candidate_id: uuid!("00000000-0000-0000-0000-000000000000"),
            recognition_id: uuid!("00000000-0000-0000-0000-000000000000"),
        };

        // Fetch candidate
        let candidate_opt = AcademicCandidateMasterCandidate::Entity::find()
            .filter(AcademicCandidateMasterCandidate::Column::DeletedAt.is_null())
            .filter(AcademicCandidateMasterCandidate::Column::Id.eq(id))
            .one(&ctx.db)
            .await?;

        if let Some(current_candidate) = candidate_opt {
            // println!("{:#?}", current_candidate);
            // Fecth candidate registration type
            validate.candidate_id = current_candidate.id;
            let candidate_registration_type_opt =
                AcademicCandidateReferenceRegistrationType::Entity::find()
                    .filter(AcademicCandidateReferenceRegistrationType::Column::DeletedAt.is_null())
                    .filter(
                        AcademicCandidateReferenceRegistrationType::Column::Id
                            .eq(current_candidate.registration_type_id),
                    )
                    .one(&ctx.db)
                    .await?;

            if let Some(candidate_registration_type) = candidate_registration_type_opt {
                // println!("{:#?}", candidate_registration_type);
                // find student registration type
                let student_registration_opt = AcademicStudentReferenceRegistration::Entity::find()
                    .filter(AcademicStudentReferenceRegistration::Column::DeletedAt.is_null())
                    .filter(
                        AcademicStudentReferenceRegistration::Column::Id
                            .eq(candidate_registration_type.student_registration_id),
                    )
                    .one(&ctx.db)
                    .await?;

                if let Some(student_registration) = student_registration_opt {
                    // println!("{:#?}", student_registration);
                    validate.registration_id = student_registration.id;
                }
            }

            // fetch recognition
            let recognition_opt =
                AcademicPriorLearningRecognitionTransactionRecognition::Entity::find()
                    .filter(
                        AcademicPriorLearningRecognitionTransactionRecognition::Column::DeletedAt
                            .is_null(),
                    )
                    .filter(
                        AcademicPriorLearningRecognitionTransactionRecognition::Column::CandidateId
                            .eq(current_candidate.id),
                    )
                    .one(&ctx.db)
                    .await?;
            if let Some(recognition) = recognition_opt {
                validate.recognition_id = recognition.id;
            }

            // Fetch user
            let user_opt = AuthUser::Entity::find()
                .filter(AuthUser::Column::DeletedAt.is_null())
                .filter(AuthUser::Column::Id.eq(current_candidate.user_id))
                .one(&ctx.db)
                .await?;

            if let Some(user) = user_opt {
                if user.individual_id != Uuid::nil() {
                    // Fetch individual
                    let individual_opt = PersonMasterIndividual::Entity::find()
                        .filter(PersonMasterIndividual::Column::DeletedAt.is_null())
                        .filter(PersonMasterIndividual::Column::Id.eq(user.individual_id))
                        .one(&ctx.db)
                        .await?;

                    if let Some(individual) = individual_opt {
                        validate.is_profile_exist = true;

                        // fetch residence
                        let residence_opt = ContactMasterResidence::Entity::find()
                            .filter(ContactMasterResidence::Column::DeletedAt.is_null())
                            .filter(
                                ContactMasterResidence::Column::ResidenceableId
                                    .eq(individual.clone().id),
                            )
                            .filter(
                                ContactMasterResidence::Column::ResidenceableType
                                    .eq("App\\Models\\Person\\Master\\Individual".to_string()),
                            )
                            .one(&ctx.db)
                            .await?;
                        if let Some(_residence) = residence_opt {
                            validate.is_address_exist = true;
                        }

                        // Fetch family card
                        let family_card_opt = PersonMasterFamilyCard::Entity::find()
                            .filter(PersonMasterFamilyCard::Column::DeletedAt.is_null())
                            .filter(
                                PersonMasterFamilyCard::Column::IndividualId
                                    .eq(individual.clone().id),
                            )
                            .one(&ctx.db)
                            .await?;

                        if let Some(family_card) = family_card_opt {
                            validate.is_family_card_exist = true;

                            // Fetch all family card members in one query
                            let family_card_members = PersonMasterFamilyMember::Entity::find()
                                .filter(PersonMasterFamilyMember::Column::DeletedAt.is_null())
                                .filter(
                                    PersonMasterFamilyMember::Column::FamilyCardId
                                        .eq(family_card.id),
                                )
                                .filter(
                                    PersonMasterFamilyMember::Column::IndividualId
                                        .eq(individual.id),
                                )
                                .all(&ctx.db)
                                .await?;

                            // Define UUID constants for mother and father
                            let mother_uuid = uuid!("c13b50e8-0bcb-4a02-a7af-fc34fd9afeb1");
                            let father_uuid = uuid!("84eca2b0-0b6e-4b6f-9e3f-5f259ededbb8");

                            // Check for mother, father, and guardian in memory
                            validate.is_mother_exist = family_card_members
                                .iter()
                                .any(|m| m.relative_type_id == mother_uuid);
                            validate.is_father_exist = family_card_members
                                .iter()
                                .any(|m| m.relative_type_id == father_uuid);
                            validate.is_guardian_exist = family_card_members.iter().any(|m| {
                                m.relative_type_id != mother_uuid
                                    && m.relative_type_id != father_uuid
                            });
                        }
                    }
                }
            }
        }

        Ok(validate)
    }
}

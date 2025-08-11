use crate::models::academic::campaign::transaction::activities::_entities::activities as AcademicCampaignTransactionActivity;
use crate::models::academic::general::reference::academic_years::_entities::academic_years as AcademicGeneralReferenceAcademicYear;
use crate::models::academic::student::campaign::activities::_entities::activities as AcademicStudentCampaignActivity;
use crate::models::academic::student::master::students::_entities::students as AcademicStudentMasterStudent;
use crate::models::auth::users::_entities::users as AuthUser;
use crate::models::contact::master::phones::_entities::phones as ContactMasterPhone;
use crate::models::institution::master::units::data_objects::DataObject as InstitutionMasterUnitDataObject;
use crate::models::payment::midtrans::customer_details::_entities::customer_details as PaymentMidtransCustomerDetail;
use crate::models::payment::midtrans::transaction_details::_entities::transaction_details as PaymentMidtransTransactionDetail;
use crate::models::person::master::individuals::_entities::individuals as PersonMasterIndividual;

use chrono::Local;
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder, Set};
// use serde::{Deserialize, Serialize};
// use std::str::FromStr;
// use uuid::{Uuid, uuid};
use uuid::Uuid;

pub struct CheckStudentPaymentMidtransTransaction {
    pub individual: PersonMasterIndividual::Model,
    pub phone: Option<ContactMasterPhone::Model>,
    pub user: Option<AuthUser::Model>,
    pub student: AcademicStudentMasterStudent::Model,
    pub student_activity: AcademicStudentCampaignActivity::Model,
    pub account_id: Uuid,
}

pub struct DataGenerator;
impl DataGenerator {
    async fn upsert(
        app_context: &AppContext,
        params: CheckStudentPaymentMidtransTransaction,
    ) -> Result<Vec<PaymentMidtransTransactionDetail::Model>, Error> {
        let mut returned: Vec<PaymentMidtransTransactionDetail::Model> = Vec::new();
        let polymorph: String = String::from("App\\Models\\Academic\\Student\\Campaign\\Activity");
        let existing_transaction_detail = PaymentMidtransTransactionDetail::Entity::find()
            .filter(PaymentMidtransTransactionDetail::Column::DeletedAt.is_null())
            .filter(
                PaymentMidtransTransactionDetail::Column::TransactionDetailableId
                    .eq(params.student_activity.id.clone()),
            )
            .filter(
                PaymentMidtransTransactionDetail::Column::TransactionDetailableType
                    .eq(polymorph.clone()),
            )
            .all(&app_context.db)
            .await
            .map_err(|e| {
                Error::Message(format!(
                    "Database error while checking for existing activity: {e}"
                ))
            })?;
        if !existing_transaction_detail.is_empty() {
            // Payment exists, return it
            Ok(existing_transaction_detail)
        } else {
            // Payment does not exist, create it
            let installment: f64 = params.student_activity.finance_fee.clone() / 2.0;
            // PaymentMidtransTransactionDetail
            let mut index = 1;

            while index <= 2 {
                let now = Local::now().naive_local();
                let mut uuidv7_string = uuid7::uuid7().to_string();
                let mut pk_id = Uuid::parse_str(&uuidv7_string).expect("Invalid UUID format");

                let new_transaction_detail = PaymentMidtransTransactionDetail::ActiveModel {
                    id: Set(pk_id),
                    account_id: Set(params.account_id.clone()),
                    order_id: Set(format!(
                        "Pembayaran {} {}",
                        index, params.student_activity.name
                    )),
                    gross_amount: Set(installment.clone()),
                    is_paid: Set(false),
                    transaction_detailable_id: Set(params.student_activity.id.clone()),
                    transaction_detailable_type: Set(polymorph.clone()),
                    created_at: Set(Some(now)),
                    updated_at: Set(Some(now)),
                    sync_at: Set(None),
                    deleted_at: Set(None),
                    created_by: Set(None),
                    updated_by: Set(None),
                };

                let current_transaction_detail =
                    PaymentMidtransTransactionDetail::Entity::insert(new_transaction_detail)
                        .exec(&app_context.db)
                        .await
                        .map_err(|e| {
                            Error::Message(format!("Failed to insert new payment: {e}"))
                        })?;

                // Fetch the newly created activity
                let transaction_detail = PaymentMidtransTransactionDetail::Entity::find_by_id(
                    current_transaction_detail.last_insert_id,
                )
                .one(&app_context.db)
                .await
                .map_err(|e| Error::Message(format!("Failed to fetch newly created payment: {e}")))?
                .ok_or_else(|| Error::Message("Newly created payment not found".to_string()))?;

                // generate customer details
                uuidv7_string = uuid7::uuid7().to_string();
                pk_id = Uuid::parse_str(&uuidv7_string).expect("Invalid UUID format");
                let new_customer_detail = PaymentMidtransCustomerDetail::ActiveModel {
                    id: Set(pk_id),
                    first_name: Set(params.individual.clone().name),
                    last_name: Set(None),
                    email: Set(Some(params.user.clone().unwrap().email)),
                    phone: Set(Some(params.phone.clone().unwrap().phone_number)),
                    transaction_detail_id: Set(transaction_detail.clone().id),
                    created_at: Set(Some(now)),
                    updated_at: Set(Some(now)),
                    sync_at: Set(None),
                    deleted_at: Set(None),
                    created_by: Set(None),
                    updated_by: Set(None),
                };

                let _current_customer_detail =
                    PaymentMidtransCustomerDetail::Entity::insert(new_customer_detail)
                        .exec(&app_context.db)
                        .await
                        .map_err(|e| {
                            Error::Message(format!("Failed to insert new customer_detail: {e}"))
                        })?;

                // Fetch the newly created activity
                // let _customer_detail = PaymentMidtransCustomerDetail::Entity::find_by_id(
                //     current_customer_detail.last_insert_id,
                // )
                // .one(&app_context.db)
                // .await
                // .map_err(|e| {
                //     Error::Message(format!(
                //         "Failed to fetch newly created customer_detail: {e}"
                //     ))
                // })?
                // .ok_or_else(|| {
                //     Error::Message("Newly created customer_detail not found".to_string())
                // })?;

                returned.push(transaction_detail.clone());
                index += 1;
            }
            Ok(returned)
        }
    }
}

pub struct GenerateStudentPaymentMidtransTransaction;
#[async_trait]
impl Task for GenerateStudentPaymentMidtransTransaction {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "GenerateStudentPaymentMidtransTransaction".to_string(),
            detail: "Generate Student Payment Midtrans Transaction".to_string(),
        }
    }

    #[allow(clippy::too_many_lines)]
    async fn run(&self, app_context: &AppContext, vars: &task::Vars) -> Result<(), Error> {
        let mut unit_id: Uuid = Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap();
        let mut academic_year_id: Uuid =
            Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap();
        let mut student_academic_year_id: Uuid =
            Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap();
        let mut account_id: Uuid = Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap();
        let unit_id_string = vars.cli_arg("unit_id");
        let academic_year_id_string = vars.cli_arg("academic_year_id");
        let student_academic_year_id_string = vars.cli_arg("student_academic_year_id");
        let account_id_string = vars.cli_arg("account_id");

        if unit_id_string.is_ok() {
            unit_id = Uuid::parse_str(unit_id_string.unwrap().as_str()).unwrap();
        }

        if academic_year_id_string.is_ok() {
            academic_year_id = Uuid::parse_str(academic_year_id_string.unwrap().as_str()).unwrap();
        }

        if student_academic_year_id_string.is_ok() {
            student_academic_year_id =
                Uuid::parse_str(student_academic_year_id_string.unwrap().as_str()).unwrap();
        }

        if account_id_string.is_ok() {
            account_id = Uuid::parse_str(account_id_string.unwrap().as_str()).unwrap();
        }

        // find academic_year
        let academic_year_result = AcademicGeneralReferenceAcademicYear::Entity::find()
            .filter(AcademicGeneralReferenceAcademicYear::Column::DeletedAt.is_null())
            .filter(AcademicGeneralReferenceAcademicYear::Column::Id.eq(academic_year_id))
            .one(&app_context.db)
            .await;

        let academic_year_opt = match academic_year_result {
            Ok(ref opt) => opt,
            Err(db_err) => {
                return Err(Error::Message(format!(
                    "Database error while querying activity: {db_err}"
                )));
            }
        };

        let Some(academic_year) = academic_year_opt else {
            return Err(Error::Message(format!(
                "academic_year not found with ID: {academic_year_id}"
            )));
        };

        // find student academic year
        let student_academic_year_result = AcademicGeneralReferenceAcademicYear::Entity::find()
            .filter(AcademicGeneralReferenceAcademicYear::Column::DeletedAt.is_null())
            .filter(AcademicGeneralReferenceAcademicYear::Column::Id.eq(student_academic_year_id))
            .one(&app_context.db)
            .await;

        let student_academic_year_opt = match student_academic_year_result {
            Ok(ref opt) => opt,
            Err(db_err) => {
                return Err(Error::Message(format!(
                    "Database error while querying activity: {db_err}"
                )));
            }
        };

        let Some(student_academic_year) = student_academic_year_opt else {
            return Err(Error::Message(format!(
                "student_academic_year not found with ID: {academic_year_id}"
            )));
        };

        // find unit
        let unit_result =
            InstitutionMasterUnitDataObject::get_by_id(app_context, unit_id, false).await?;
        let Some(ref unit_data) = unit_result else {
            return Err(Error::Message(format!("unit not found with ID: {unit_id}")));
        };

        // find unit activity
        let unit_activity_result = AcademicCampaignTransactionActivity::Entity::find()
            .filter(AcademicCampaignTransactionActivity::Column::DeletedAt.is_null())
            .filter(AcademicCampaignTransactionActivity::Column::UnitId.eq(unit_id))
            .filter(
                AcademicCampaignTransactionActivity::Column::AcademicYearId.eq(academic_year.id),
            )
            .one(&app_context.db)
            .await;
        let unit_activity_opt = match unit_activity_result {
            Ok(ref opt) => opt,
            Err(db_err) => {
                return Err(Error::Message(format!(
                    "Database error while querying activity: {db_err}"
                )));
            }
        };

        let Some(unit_activity) = unit_activity_opt else {
            return Err(Error::Message(format!(
                "unit_activity not found with ID: {academic_year_id}"
            )));
        };

        let student_result = AcademicStudentMasterStudent::Entity::find()
            .filter(AcademicStudentMasterStudent::Column::DeletedAt.is_null())
            .filter(AcademicStudentMasterStudent::Column::UnitId.eq(unit_data.unit.id))
            .filter(
                AcademicStudentMasterStudent::Column::AcademicYearId.eq(student_academic_year.id),
            )
            .order_by_asc(AcademicStudentMasterStudent::Column::Code)
            .all(&app_context.db)
            .await;
        let students = match student_result {
            Ok(students) => students,
            Err(db_err) => {
                return Err(Error::Message(format!(
                    "Database error while querying students: {db_err}"
                )));
            }
        };

        if students.is_empty() {
            return Err(Error::Message(format!("students not found")));
        }

        for student in students {
            // find individual
            let individual_result = PersonMasterIndividual::Entity::find()
                .filter(PersonMasterIndividual::Column::DeletedAt.is_null())
                .filter(PersonMasterIndividual::Column::Id.eq(student.clone().individual_id))
                .one(&app_context.db)
                .await;
            let individual_opt = match individual_result {
                Ok(ref opt) => opt,
                Err(db_err) => {
                    return Err(Error::Message(format!(
                        "Database error while querying individual: {db_err}"
                    )));
                }
            };

            let Some(individual) = individual_opt else {
                return Err(Error::Message(format!("individual not found")));
            };
            // find user
            let user_result = AuthUser::Entity::find()
                .filter(AuthUser::Column::DeletedAt.is_null())
                .filter(AuthUser::Column::IndividualId.eq(student.clone().individual_id))
                .one(&app_context.db)
                .await;
            let user_opt = match user_result {
                Ok(ref opt) => opt,
                Err(db_err) => {
                    return Err(Error::Message(format!(
                        "Database error while querying user: {db_err}"
                    )));
                }
            };

            // let Some(user) = user_opt else {
            //     return Err(Error::Message(format!("user not found")));
            // };

            let user = if let Some(u) = user_opt {
                u
            } else {
                println!("user not found for student code {}", student.clone().code);
                continue; // or return Err(...) depending on your logic needs
            };
            // find phone
            let phone_result = ContactMasterPhone::Entity::find()
                .filter(ContactMasterPhone::Column::DeletedAt.is_null())
                .filter(ContactMasterPhone::Column::PhoneableId.eq(student.clone().individual_id))
                .filter(
                    ContactMasterPhone::Column::PhoneableType
                        .eq(String::from("App\\Models\\Person\\Master\\Individual")),
                )
                // .build(DbBackend::Postgres);
                .one(&app_context.db)
                .await;
            // println!("{}", phone_result.to_string());
            let phone_opt = match phone_result {
                Ok(ref opt) => opt,
                Err(db_err) => {
                    return Err(Error::Message(format!(
                        "Database error while querying contact: {db_err}"
                    )));
                }
            };

            // let Some(phone) = phone_opt else {
            //     // return Err(Error::Message(format!("phone not found")));
            //     println!("Phone not found for student code {}", student.clone().code);
            // };
            let phone = if let Some(p) = phone_opt {
                p
            } else {
                println!("Phone not found for student code {}", student.clone().code);
                continue; // or return Err(...) depending on your logic needs
            };
            //find student activity
            let student_activity_result = AcademicStudentCampaignActivity::Entity::find()
                .filter(AcademicStudentCampaignActivity::Column::DeletedAt.is_null())
                .filter(AcademicStudentCampaignActivity::Column::StudentId.eq(student.clone().id))
                .filter(
                    AcademicStudentCampaignActivity::Column::UnitActivityId
                        .eq(unit_activity.clone().id),
                )
                .one(&app_context.db)
                .await;
            let student_activity_opt = match student_activity_result {
                Ok(ref opt) => opt,
                Err(db_err) => {
                    return Err(Error::Message(format!(
                        "Database error while querying student activity: {db_err}"
                    )));
                }
            };

            let Some(student_activity) = student_activity_opt else {
                return Err(Error::Message(format!(
                    "student_activity not found with ID: {academic_year_id}"
                )));
            };
            // create params CheckStudentPaymentMidtransTransaction
            let params = CheckStudentPaymentMidtransTransaction {
                user: Some(user.clone()),
                individual: individual.clone(),
                phone: Some(phone.clone()),
                student: student.clone(),
                student_activity: student_activity.clone(),
                account_id: account_id.clone(),
            };

            // execute params
            if let Err(err) = DataGenerator::upsert(app_context, params).await {
                eprintln!("Error processing unit {}: {}", student.code, err);
            }
        }

        Ok(())
    }
}

#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::common::settings::Settings;
use crate::middleware::is_authenticated::AuthenticatedLayer;
use crate::models::academic::student::campaign::activities::_entities::activities as AcademicStudentCampaignActivity;
use crate::models::academic::student::master::students::_entities::students as AcademicStudentMasterStudent;
use crate::models::auth::users::_entities::users;
use crate::models::institution::master::units::_entities::units as InstitutionMasterUnit;
use crate::models::payment::midtrans::transaction_details::_entities::transaction_details as PaymentMidtransTransactionDetails;
use crate::services::payment::midtrans::token::Token as ServicePaymentMidtransToken;
use axum::debug_handler;
use axum::extract::Extension;
use chrono::Local;
use loco_rs::prelude::*;
use uuid::Uuid;

#[debug_handler]
pub async fn index(
    State(ctx): State<AppContext>,
    Extension(user): Extension<users::Model>,
) -> Result<Response> {
    if let Some(settings) = &ctx.config.settings {
        let mut student_ids: Vec<Uuid> = Vec::new();
        let mut unit_ids: Vec<Uuid> = Vec::new();
        let mut student_activity_ids: Vec<Uuid> = Vec::new();

        let settings = Settings::from_json(settings)?;
        // Parse the institution_id string to UUID directly
        let institution_id = match Uuid::parse_str(&settings.current_institution_id) {
            Ok(id) => id,
            Err(e) => {
                return Err(Error::Message(format!(
                    "Failed to parse institution UUID: {e}"
                )));
            }
        };
        let unit_type_id = match Uuid::parse_str("019759fd-36e8-4f43-80ed-4f687a48145d") {
            Ok(id) => id,
            Err(e) => {
                return Err(Error::Message(format!(
                    "Failed to parse institution UUID: {e}"
                )));
            }
        };

        // find all units
        let units_result = InstitutionMasterUnit::Entity::find()
            .filter(InstitutionMasterUnit::Column::DeletedAt.is_null())
            .filter(InstitutionMasterUnit::Column::InstitutionId.eq(institution_id))
            .filter(InstitutionMasterUnit::Column::UnitTypeId.eq(unit_type_id))
            .all(&ctx.db)
            .await;
        let units = match units_result {
            Ok(units) => units,
            Err(db_err) => {
                return Err(Error::Message(format!(
                    "Database error while querying units: {db_err}"
                )));
            }
        };

        if units.is_empty() {
            // Return empty successful response when no units found
            return format::json(serde_json::json!({
                "data": [],
                "message": "No units found for the institution"
            }));
        }

        for unit in units {
            unit_ids.push(unit.id);
        }

        // find student
        let students_result = AcademicStudentMasterStudent::Entity::find()
            .filter(AcademicStudentMasterStudent::Column::DeletedAt.is_null())
            .filter(AcademicStudentMasterStudent::Column::IndividualId.eq(user.individual_id))
            .filter(AcademicStudentMasterStudent::Column::UnitId.is_in(unit_ids.clone()))
            .all(&ctx.db)
            .await;
        let students = match students_result {
            Ok(students) => students,
            Err(db_err) => {
                return Err(Error::Message(format!(
                    "Database error while querying units: {db_err}"
                )));
            }
        };

        if students.is_empty() {
            // Return empty successful response when no students found
            return format::json(serde_json::json!({
                "data": [],
                "message": "No students found for the user"
            }));
        }

        for student in students {
            student_ids.push(student.id);
        }

        // find all activities
        let student_activities_result = AcademicStudentCampaignActivity::Entity::find()
            .filter(AcademicStudentCampaignActivity::Column::DeletedAt.is_null())
            .filter(AcademicStudentCampaignActivity::Column::StudentId.is_in(student_ids.clone()))
            .all(&ctx.db)
            .await;
        let student_activities = match student_activities_result {
            Ok(student_activities) => student_activities,
            Err(db_err) => {
                return Err(Error::Message(format!(
                    "Database error while querying units: {db_err}"
                )));
            }
        };

        if student_activities.is_empty() {
            // Return empty successful response when no activities found
            return format::json(serde_json::json!({
                "data": [],
                "message": "No activities found for the students"
            }));
        }

        for student_activity in student_activities {
            student_activity_ids.push(student_activity.id);
        }

        // find all payments
        let payment_result = PaymentMidtransTransactionDetails::Entity::find()
            .filter(PaymentMidtransTransactionDetails::Column::DeletedAt.is_null())
            .filter(
                PaymentMidtransTransactionDetails::Column::TransactionDetailableId
                    .is_in(student_activity_ids.clone()),
            )
            .filter(
                PaymentMidtransTransactionDetails::Column::TransactionDetailableType
                    .eq("App\\Models\\Academic\\Student\\Campaign\\Activity".to_string()),
            )
            .all(&ctx.db)
            .await;

        match payment_result {
            Ok(payments) => {
                if payments.is_empty() {
                    // Return empty data array in JSON format
                    return format::json(serde_json::json!({
                        "data": [],
                        "message": "No payments found"
                    }));
                }

                // Return the payments data in JSON format
                format::json(serde_json::json!({
                    "data": payments,
                    "message": "Payments retrieved successfully"
                }))
            }
            Err(db_err) => {
                return Err(Error::Message(format!(
                    "Database error while querying payment_result: {db_err}"
                )));
            }
        }
    } else {
        // Return error if settings are not available
        Err(Error::Message("Settings not available".to_string()))
    }
}

pub async fn pay(
    State(ctx): State<AppContext>,
    Extension(_user): Extension<users::Model>,
    Path(transaction_detail_id): Path<Uuid>,
) -> Result<Response> {
    let transaction_detail_result = PaymentMidtransTransactionDetails::Entity::find()
        .filter(PaymentMidtransTransactionDetails::Column::DeletedAt.is_null())
        .filter(PaymentMidtransTransactionDetails::Column::Id.eq(transaction_detail_id))
        .one(&ctx.db)
        .await;
    let transaction_detail_opt = match transaction_detail_result {
        Ok(ref opt) => opt,
        Err(db_err) => {
            return Err(Error::Message(format!(
                "Database error while querying activity: {db_err}"
            )));
        }
    };
    let Some(transaction_detail) = transaction_detail_opt else {
        return Err(Error::Message(format!("transaction detail not found")));
    };

    // Fix 1: Clone the transaction_detail model to pass an owned value
    // Fix 2: Await the token result since get() is an async function
    let token_result = ServicePaymentMidtransToken::get(ctx, transaction_detail.clone()).await;

    match token_result {
        Ok(token) => {
            // Now we can safely debug print the token
            // println!("Token: {:#?}", token);
            // Return the token as a JSON response
            format::json(serde_json::json!({
                "data": token,
                "message": "Payment token generated successfully"
            }))
        }
        Err(err) => {
            // Return the error
            Err(Error::Message(format!(
                "Failed to get payment token: {}",
                err
            )))
        }
    }
}

#[debug_handler]
pub async fn successful_payment(
    State(ctx): State<AppContext>,
    Extension(_user): Extension<users::Model>,
    Path(transaction_detail_id): Path<Uuid>,
) -> Result<Response> {
    let transaction_detail_result = PaymentMidtransTransactionDetails::Entity::find()
        .filter(PaymentMidtransTransactionDetails::Column::DeletedAt.is_null())
        .filter(PaymentMidtransTransactionDetails::Column::Id.eq(transaction_detail_id))
        .one(&ctx.db)
        .await;
    let transaction_detail_opt = match transaction_detail_result {
        Ok(ref opt) => opt,
        Err(db_err) => {
            return Err(Error::Message(format!(
                "Database error while querying activity: {db_err}"
            )));
        }
    };
    let Some(transaction_detail) = transaction_detail_opt else {
        return Err(Error::Message(format!("transaction detail not found")));
    };

    let now = Local::now().naive_local();
    let mut data = transaction_detail.clone().into_active_model();
    data.updated_at = Set(Some(now));
    data.is_paid = Set(true);

    let updated_item = data.update(&ctx.db).await?;
    // println!("Successful payment");
    format::json(updated_item)
}

// #[debug_handler]
// pub async fn error_payment(
//     State(ctx): State<AppContext>,
//     Extension(_user): Extension<users::Model>,
//     Path(transaction_detail_id): Path<Uuid>,
// ) -> Result<Response> {
//     println!("Error payment");
//     format::empty()
// }

pub fn routes(ctx: &AppContext) -> Routes {
    Routes::new()
        .prefix("api/payment/midtrans/student")
        .add("/", get(index).layer(AuthenticatedLayer::new(ctx.clone())))
        .add(
            "/pay/{transaction_detail_id}",
            get(pay).layer(AuthenticatedLayer::new(ctx.clone())),
        )
        .add(
            "/successful_payment/{transaction_detail_id}",
            get(successful_payment).layer(AuthenticatedLayer::new(ctx.clone())),
        )
    // .add("/error_payment/{transaction_detail_id}", get(error_payment))
}

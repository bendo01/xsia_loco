use crate::common::settings::Settings;
use crate::models::payment::midtrans::accounts::_entities::accounts as PaymentMidtransAccount;
use crate::models::payment::midtrans::customer_details::_entities::customer_details as PaymentMidtransCustomerDetail;
use crate::models::payment::midtrans::transaction_details::_entities::transaction_details as PaymentMidtransTransactionDetail;
use base64::{Engine as _, engine::general_purpose::STANDARD};
use loco_rs::prelude::*; // This imports most common Loco types including Error
use reqwest::Client;
use serde::{Deserialize, Serialize};
// use serde_json;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreditCard {
    pub secure: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionDetails {
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(rename = "gross_amount")]
    pub gross_amount: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomerDetails {
    #[serde(rename = "first_name")]
    pub first_name: String,
    #[serde(rename = "last_name")]
    pub last_name: Option<String>,
    #[serde(rename = "email")]
    pub email: Option<String>,
    #[serde(rename = "phone")]
    pub phone: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenParams {
    #[serde(rename = "transaction_details")]
    pub transaction_details: TransactionDetails,
    #[serde(rename = "credit_card")]
    pub credit_card: CreditCard,
    #[serde(rename = "customer_details")]
    pub customer_details: Option<CustomerDetails>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    #[serde(rename = "token")]
    pub token: Option<String>,
    #[serde(rename = "redirect_url")]
    pub redirect_url: Option<String>,
}

impl Token {
    pub async fn get(
        ctx: AppContext,
        transaction_detail: PaymentMidtransTransactionDetail::Model,
    ) -> Result<Token, Error> {
        if let Some(settings) = &ctx.config.settings {
            let settings = Settings::from_json(settings)?;
            // Removed unused 'returned' variable

            // get account
            let account_result = PaymentMidtransAccount::Entity::find()
                .filter(PaymentMidtransAccount::Column::DeletedAt.is_null())
                .filter(
                    PaymentMidtransAccount::Column::Id.eq(transaction_detail.clone().account_id),
                )
                .one(&ctx.db)
                .await;
            let account_opt = match account_result {
                Ok(ref opt) => opt,
                Err(db_err) => {
                    return Err(Error::Message(format!(
                        "Database error while querying activity: {db_err}"
                    )));
                }
            };
            let Some(account) = account_opt else {
                return Err(Error::Message(format!("account not found")));
            };
            // get customer details
            let customer_detail_result = PaymentMidtransCustomerDetail::Entity::find()
                .filter(PaymentMidtransCustomerDetail::Column::DeletedAt.is_null())
                .filter(
                    PaymentMidtransCustomerDetail::Column::TransactionDetailId
                        .eq(transaction_detail.clone().id),
                )
                .one(&ctx.db)
                .await;
            let customer_detail_opt = match customer_detail_result {
                Ok(ref opt) => opt,
                Err(db_err) => {
                    return Err(Error::Message(format!(
                        "Database error while querying activity: {db_err}"
                    )));
                }
            };

            // Log if no customer details are found
            if customer_detail_opt.is_none() {
                println!(
                    "customer_detail not found for student code {}",
                    transaction_detail.clone().id
                );
            }

            let url = if settings.is_production_midtrans_payment {
                account.clone().production_url
            } else {
                account.clone().sandbox_url
            };

            let rounded_gross_amount = transaction_detail.clone().gross_amount.round() as i64;

            // Implement the token retrieval logic here
            let mut token_params = TokenParams {
                credit_card: CreditCard { secure: true },
                transaction_details: TransactionDetails {
                    order_id: transaction_detail.clone().id.to_string(),
                    gross_amount: rounded_gross_amount,
                },
                customer_details: None,
            };

            // Only add customer details if we have them
            if let Some(detail) = &customer_detail_opt {
                token_params.customer_details = Some(CustomerDetails {
                    first_name: detail.clone().first_name,
                    last_name: detail.clone().last_name,
                    email: detail.clone().email,
                    phone: detail.clone().phone,
                });
            }

            let encoded = STANDARD.encode(format!("{}:", account.clone().server_key));
            // let cloned = token_params.clone();
            // println!("token_params: {:#?}", token_params.clone());
            // println!("encoded: {:#?}", encoded.clone());
            // let json = serde_json::to_string(&cloned).unwrap();
            // println!("JSON output: {}", json);

            let http_client: Client = Client::new();

            // Fix 2: Correct request building with reqwest
            let http_result = http_client
                .post(&url)
                .header("Content-Type", "application/json")
                .header("Accept", "application/json")
                .header("Authorization", format!("Basic {}", encoded))
                .timeout(tokio::time::Duration::from_secs(10))
                .json(&token_params) // Use the struct directly
                .send()
                .await;

            // Handle the HTTP result without unwrapping
            let response = match http_result {
                Ok(res) => match res.json::<Token>().await {
                    Ok(token_response) => token_response,
                    Err(_) => return Err(Error::string("Failed to parse response JSON")),
                },
                Err(_) => return Err(Error::string("Failed to send request")),
            };

            // Check if data exists
            match response.token {
                Some(_) => {
                    // println!("Token response: {:#?}", response.clone());
                    Ok(response)
                } // Return the entire Token struct, not just the token string
                None => Err(Error::string("Token data not found in response")),
            }
        } else {
            Err(Error::Message(
                "fail to register user because setting not loaded".to_string(),
            ))
        }
    }
}

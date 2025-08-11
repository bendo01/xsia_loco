use crate::common::settings::Settings;
use crate::models::feeder::referensi::agama::_entities::agama as FeederReferensiAgama;
use crate::services::feeder_dikti::requester::token::Token;
use chrono::Local;
use loco_rs::prelude::*;
use reqwest::Client;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Data {
    pub id_agama: i32,
    pub nama_agama: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RequestData {
    pub act: String,
    pub token: String,
    pub filter: Option<String>,
    pub order: Option<String>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReturnData {
    pub error_code: i32,
    pub error_desc: Option<String>,
    pub jumlah: i32,
    pub data: Option<Vec<Data>>,
}

impl RequestData {
    /// Fetches religion data from the Feeder Dikti API
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - Failed to get a token from the token service
    /// - Settings are not properly loaded
    /// - HTTP request to Feeder API fails
    /// - Response parsing fails
    pub async fn get(ctx: &AppContext) -> Result<ReturnData, Error> {
        let token = match Token::get(ctx.clone()).await {
            Ok(token) => token,
            Err(err) => {
                return Err(Error::Message(format!("Fail To Request: {err}")));
            }
        };

        // Now complete the implementation with the token
        if let Some(settings) = &ctx.config.settings {
            let settings = Settings::from_json(settings)?;
            let feeder_url = settings.feeder_url;

            let request_data = Self {
                act: "Getreference".to_string(),
                token,
                filter: None,
                order: None,
                limit: None,
                offset: None,
            };

            let http_client: Client = Client::new();
            let response = match http_client
                .post(feeder_url)
                .timeout(tokio::time::Duration::from_secs(10))
                .json(&request_data)
                .send()
                .await
            {
                Ok(res) => match res.json::<ReturnData>().await {
                    Ok(data) => data,
                    Err(err) => {
                        return Err(Error::Message(format!(
                            "Failed to parse response JSON: {err}",
                        )));
                    }
                },
                Err(err) => return Err(Error::Message(format!("Failed to send request: {err}"))),
            };
            // println!("Response: {response:#?}");
            Ok(response)
        } else {
            Err(Error::Message("Settings not loaded".to_string()))
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetAgama;

impl GetAgama {
    /// Inserts or updates religion data in the database
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - Database query fails
    /// - Update or insert operations fail
    ///
    /// # Panics
    ///
    /// This function will panic if:
    /// - The UUID generated is in an invalid format
    pub async fn upsert(ctx: &AppContext, params: Data) -> Result<(), Error> {
        // Check if the record exists
        let data_result = FeederReferensiAgama::Entity::find()
            .filter(FeederReferensiAgama::Column::DeletedAt.is_null())
            .filter(FeederReferensiAgama::Column::IdAgama.eq(params.id_agama))
            .filter(
                FeederReferensiAgama::Column::NamaAgama.eq(params
                    .nama_agama
                    .clone()
                    .trim()
                    .to_string()),
            )
            .one(&ctx.db)
            .await;

        // Then handle the Result
        let data_opt = match data_result {
            Ok(opt) => opt,
            Err(db_err) => {
                return Err(Error::Message(format!(
                    "Database error while querying reference: {db_err}"
                )));
            }
        };

        // If the record exists, update it; otherwise, insert a new one
        if let Some(existing_reference) = data_opt {
            let mut reference: FeederReferensiAgama::ActiveModel = existing_reference.into();
            reference.nama_agama = Set(params.nama_agama.clone().trim().to_string());
            reference.updated_at = Set(Local::now().naive_local());
            reference.sync_at = Set(Some(Local::now().naive_local()));
            match reference.update(&ctx.db).await {
                Ok(_updated_model) => (), // Use _ to ignore the returned model
                Err(err) => {
                    return Err(Error::Message(format!("Failed to update reference: {err}")));
                }
            }
        } else {
            let uuidv7_string = uuid7::uuid7().to_string();
            let pk_id = Uuid::parse_str(&uuidv7_string).expect("Invalid UUID format");
            let new_reference = FeederReferensiAgama::ActiveModel {
                id: Set(pk_id),
                id_agama: Set(params.id_agama),
                nama_agama: Set(params.nama_agama.clone().trim().to_string()),
                created_at: Set(Local::now().naive_local()),
                updated_at: Set(Local::now().naive_local()),
                sync_at: Set(Some(Local::now().naive_local())),
                // Set other required fields
                ..Default::default()
            };
            match FeederReferensiAgama::Entity::insert(new_reference)
                .exec(&ctx.db)
                .await
            {
                Ok(_insert_result) => (), // Use _ to ignore the insert result
                Err(err) => {
                    return Err(Error::Message(format!(
                        "Failed to insert new reference: {err}"
                    )));
                }
            }
        }

        Ok(())
    }
}

#[async_trait]
impl Task for GetAgama {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "GetAgama".to_string(),
            detail: "get reference data from feeder".to_string(),
        }
    }

    async fn run(&self, ctx: &AppContext, _vars: &task::Vars) -> Result<(), Error> {
        let req_option = RequestData::get(ctx).await;
        // Handle the datas option properly
        if let Ok(req) = req_option {
            if let Some(data_vec) = req.data {
                // Iterate through each item in the vector
                for data in data_vec {
                    // println!("Data: {data:#?}");
                    // Handle upsert without ? operator
                    match Self::upsert(ctx, data).await {
                        Ok(()) => {
                            // Upsert succeeded, continue to next item
                        }
                        Err(err) => {
                            // Log the error but continue processing
                            eprintln!("Error upserting data: {err}");
                            // Alternatively, if you want to stop on first error:
                            // return Err(err);
                        }
                    }
                }
            }
        } else if let Err(err) = req_option {
            return Err(err);
        }

        Ok(())
    }
}

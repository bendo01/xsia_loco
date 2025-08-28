use crate::models::feeder::referensi::jalur_masuk::_entities::jalur_masuk as FeederReferensiModel;
use crate::tasks::feeder_dikti::downstream::request_data::RequestData;
use chrono::Local;
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Data {
    pub id_jalur_masuk: String,
    pub nama_jalur_masuk: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetJalurMasuk;

impl GetJalurMasuk {
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
        let data_result = FeederReferensiModel::Entity::find()
            .filter(FeederReferensiModel::Column::DeletedAt.is_null())
            .filter(FeederReferensiModel::Column::IdJalurMasuk.eq(params.id_jalur_masuk.clone()))
            .filter(
                FeederReferensiModel::Column::NamaJalurMasuk.eq(params
                    .nama_jalur_masuk
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
            let mut reference: FeederReferensiModel::ActiveModel = existing_reference.into();
            reference.nama_jalur_masuk = Set(params.nama_jalur_masuk.clone().trim().to_string());
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
            let new_reference = FeederReferensiModel::ActiveModel {
                id: Set(pk_id),
                id_jalur_masuk: Set(params.id_jalur_masuk),
                nama_jalur_masuk: Set(params.nama_jalur_masuk.clone().trim().to_string()),
                created_at: Set(Local::now().naive_local()),
                updated_at: Set(Local::now().naive_local()),
                sync_at: Set(Some(Local::now().naive_local())),
                // Set other required fields
                ..Default::default()
            };
            match FeederReferensiModel::Entity::insert(new_reference)
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
impl Task for GetJalurMasuk {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "GetJalurMasuk".to_string(),
            detail: "get reference data from feeder".to_string(),
        }
    }

    async fn run(&self, ctx: &AppContext, _vars: &task::Vars) -> Result<(), Error> {
        let req_option = RequestData::get::<Data>(ctx, "GetJalurMasuk".to_string()).await;
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

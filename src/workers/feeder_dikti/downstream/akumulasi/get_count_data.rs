use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

use chrono::Local;
use colored::Colorize;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use uuid::Uuid; // Removed unused `uuid` macro import

use crate::common::settings::Settings;
use crate::models::feeder::akumulasi::jumlah_data::_entities::jumlah_data as FeederAkumulasiJumlahData;
use crate::tasks::feeder_dikti::downstream::request_data_akumulasi::{
    InputRequestData, RequestDataAkumulasi,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetCountData;

impl GetCountData {
    pub async fn upsert(
        ctx: &AppContext,
        table_column_name: String,
        total_feeder: i32,
    ) -> Result<(), Error> {
        // Initialize with default UUID, will be updated from settings
        let institution_id: Uuid = if let Some(current_settings) = &ctx.config.settings {
            println!("Settings loaded");
            let settings = Settings::from_json(current_settings)?;
            match Uuid::parse_str(settings.current_institution_id.as_str()) {
                Ok(parsed_id) => {
                    println!("Successfully parsed institution id");
                    parsed_id
                }
                Err(_) => {
                    println!("Failed to parse institution id");
                    return Err(Error::Message("Invalid institution ID format".to_string()));
                }
            }
        } else {
            return Err(Error::Message("Setting not loaded".to_string()));
        };

        let data_result = FeederAkumulasiJumlahData::Entity::find()
            .filter(FeederAkumulasiJumlahData::Column::DeletedAt.is_null())
            .filter(FeederAkumulasiJumlahData::Column::InstitutionId.eq(institution_id))
            .filter(FeederAkumulasiJumlahData::Column::Name.eq(table_column_name.clone()))
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
            let mut reference: FeederAkumulasiJumlahData::ActiveModel = existing_reference.into();
            reference.total_feeder = Set(total_feeder);
            reference.updated_at = Set(Local::now().naive_local());
            reference.sync_at = Set(Some(Local::now().naive_local()));
            match reference.update(&ctx.db).await {
                Ok(_updated_model) => {
                    println!("{}", "Data updated successfully".green());
                    Ok(())
                }
                Err(err) => {
                    return Err(Error::Message(format!("Failed to update reference: {err}")));
                }
            }
        } else {
            let uuidv7_string = uuid7::uuid7().to_string();
            let pk_id = Uuid::parse_str(&uuidv7_string).expect("Invalid UUID format");
            let new_reference = FeederAkumulasiJumlahData::ActiveModel {
                id: Set(pk_id),
                name: Set(table_column_name.clone()),
                institution_id: Set(institution_id),
                total_feeder: Set(total_feeder), // Don't forget to set this field
                created_at: Set(Local::now().naive_local()),
                updated_at: Set(Local::now().naive_local()),
                sync_at: Set(Some(Local::now().naive_local())),
                ..Default::default()
            };
            match FeederAkumulasiJumlahData::Entity::insert(new_reference)
                .exec(&ctx.db)
                .await
            {
                Ok(_insert_result) => {
                    println!("{}", "Data inserted successfully".green());
                    Ok(())
                }
                Err(err) => {
                    return Err(Error::Message(format!(
                        "Failed to insert new reference: {err}"
                    )));
                }
            }
        }
    }
}

pub struct GetCountDataWorker {
    pub ctx: AppContext,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct GetCountDataWorkerWorkerArgs {
    pub table_column_name: String,
    pub action_name: String,
}

#[async_trait]
impl BackgroundWorker<GetCountDataWorkerWorkerArgs> for GetCountDataWorker {
    /// Creates a new instance of the Worker with the given application context.
    ///
    /// This function is called when registering the worker with the queue system.
    ///
    /// # Parameters
    /// * `ctx` - The application context containing shared resources
    fn build(ctx: &AppContext) -> Self {
        Self { ctx: ctx.clone() }
    }

    /// Returns the class name of the worker.
    ///
    /// This name is used when enqueueing jobs and identifying the worker in logs.
    /// The implementation returns the struct name as a string.
    fn class_name() -> String {
        "AkumulasiGetCountData".to_string()
    }

    /// Returns tags associated with this worker.
    ///
    /// Tags can be used to filter which workers run during startup.
    /// The default implementation returns an empty vector (no tags).
    fn tags() -> Vec<String> {
        Vec::new()
    }

    /// Performs the actual work when a job is processed.
    ///
    /// This is the main function that contains the worker's logic.
    /// It gets executed when a job is dequeued from the job queue.
    ///
    /// # Returns
    /// * `Result<()>` - Ok if the job completed successfully, Err otherwise
    async fn perform(&self, args: GetCountDataWorkerWorkerArgs) -> Result<()> {
        println!("=================AkumulasiGetCountData=======================");
        // TODO: Some actual work goes here...
        println!("Table column name: {}", args.table_column_name.clone());
        println!("Action name: {}", args.action_name.clone());
        let req_option = RequestDataAkumulasi::get(
            &self.ctx,
            InputRequestData {
                act: args.action_name.clone(),
                filter: None,
                order: None,
                limit: None,
                offset: None,
            },
        )
        .await;
        if let Ok(req) = req_option {
            // println!("Data Akumulasi: {:#?}", req.clone().data);
            // convert to i32 if needed

            let total_feeder = match req.data.to_i32() {
                Ok(parsed_value) => parsed_value,
                Err(parse_err) => {
                    eprintln!(
                        "Error parsing data to i32: {parse_err}. Data: '{}'",
                        req.data
                    );
                    return Err(Error::Message(format!(
                        "Failed to parse data '{}' to i32: {parse_err}",
                        req.data
                    )));
                }
            };

            match GetCountData::upsert(&self.ctx, args.table_column_name.clone(), total_feeder)
                .await
            {
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
        } else {
            println!("Failed To Get Data {:#?}", req_option);
        }
        Ok(())
    }
}

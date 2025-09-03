use chrono::Local;
use colored::Colorize;
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use uuid::Uuid; // Removed unused `uuid` macro import

use crate::common::settings::Settings;
use crate::models::feeder::akumulasi::jumlah_data::_entities::jumlah_data as FeederAkumulasiJumlahData;
use crate::tasks::feeder_dikti::downstream::request_data_akumulasi::RequestDataAkumulasi;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetCountMahasiswa;

impl GetCountMahasiswa {
    pub async fn upsert(ctx: &AppContext, total_feeder: i32) -> Result<(), Error> {
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
            .filter(
                FeederAkumulasiJumlahData::Column::Name.eq("FA0003GetCountMahasiswa".to_string()),
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
                name: Set("FA0003GetCountMahasiswa".to_string()),
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

// Rest of the code remains the same...

#[async_trait]
impl Task for GetCountMahasiswa {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "GetCountMahasiswa".to_string(),
            detail: "get periode estimate count data mahasiswa from feeder".to_string(),
        }
    }

    async fn run(&self, ctx: &AppContext, _vars: &task::Vars) -> Result<(), Error> {
        // if let Some(current_settings) = &ctx.config.settings {
        // println!("Settings {:#?}", current_settings);
        // let settings = Settings::from_json(current_settings)?;
        //if let Ok(institution_id) = Uuid::parse_str(settings.current_institution_id.as_str()) {
        // println!("Institution ID: {:#?}", institution_id);
        // get record FM0008GetListMahasiswa with id institution id
        // let req_option = RequestDataAkumulasi::get(ctx, "GetCountMahasiswa".to_string()).await;
        // if let Ok(req) = req_option {
        // println!("Data Akumulasi: {:#?}", req);
        // }
        // }

        // } else {
        // println!("Settings Not loaded");
        // }

        let req_option = RequestDataAkumulasi::get(ctx, "GetCountMahasiswa".to_string()).await;
        if let Ok(req) = req_option {
            println!("Data Akumulasi: {:#?}", req.clone().data);
            match Self::upsert(ctx, req.clone().data).await {
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
        Ok(())
    }
}

use chrono::Local;
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::common::settings::Settings;
use crate::tasks::feeder_dikti::downstream::request_data_akumulasi::RequestDataAkumulasi;
use crate::models::feeder::akumulasi::jumlah_data::_entities::jumlah_data as FeederAkumulasiJumlahData;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetCountMahasiswa;

impl GetCountMahasiswa {
  
}

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
            println!("Data Akumulasi: {:#?}", req.data);
        }
        Ok(())
    }
}

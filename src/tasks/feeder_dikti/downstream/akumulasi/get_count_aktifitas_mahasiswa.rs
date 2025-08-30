// use crate::models::feeder::akumulasi::jumlah_data::_entities::jumlah_data as FeederReferensiModel;
// use crate::task::feeder_dikti::downstream::request_data_akumulasi::RequestData;
// use chrono::Local;
// use loco_rs::prelude::*;
// use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
// use serde::{Deserialize, Serialize};
// use uuid::Uuid;

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct GetCountAktifitasMahasiswa;

// impl GetCountAktifitasMahasiswa {
  
// }

// #[async_trait]
// impl Task for GetCountAktifitasMahasiswa {
//     fn task(&self) -> TaskInfo {
//         TaskInfo {
//             name: "GetCountAktifitasMahasiswa".to_string(),
//             detail: "get reference estimate count data from feeder".to_string(),
//         }
//     }

//     async fn run(&self, ctx: &AppContext, _vars: &task::Vars) -> Result<(), Error> {
//       Ok(())
//     }
// }

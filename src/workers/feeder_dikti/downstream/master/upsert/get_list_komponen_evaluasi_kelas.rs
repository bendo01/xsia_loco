use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

use crate::tasks::feeder_dikti::downstream::request_only_data::{InputRequestData, RequestData};
use crate::workers::feeder_dikti::downstream::master::download::get_list_komponen_evaluasi_kelas::{ModelData, ModelInput};

pub struct Worker {
    pub ctx: AppContext,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct WorkerArgs {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[async_trait]
impl BackgroundWorker<WorkerArgs> for Worker {
    fn build(ctx: &AppContext) -> Self {
        Self { ctx: ctx.clone() }
    }

    fn class_name() -> String {
        "GetListKomponenEvaluasiKelasUpsert".to_string()
    }

    fn tags() -> Vec<String> {
        Vec::new()
    }

    async fn perform(&self, args: WorkerArgs) -> Result<()> {
        let limit = args.limit;
        let offset = args.offset;

        let req = RequestData::get::<ModelInput>(&self.ctx, InputRequestData {
            act: "GetListKomponenEvaluasiKelas".to_string(),
            filter: None,
            order: None,
            limit,
            offset,
        })
        .await;

        match req {
            Ok(response) => {
                match response.data {
                    Some(items) if !items.is_empty() => {
                        for item in items {
                            if let Err(e) = ModelData::upsert(&self.ctx, item).await {
                                eprintln!("Failed to upsert item: {}", e);
                            }
                        }
                    }
                    Some(_) | None => {
                        // nothing to do
                    }
                }
            }
            Err(err) => {
                return Err(Error::Message(format!("Failed requesting feeder: {err}")));
            }
        }

        Ok(())
    }
}

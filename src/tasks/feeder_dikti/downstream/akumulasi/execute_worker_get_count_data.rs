use loco_rs::prelude::*;
use crate::workers::feeder_dikti::downstream::akumulasi::get_count_data::{GetCountDataWorker, GetCountDataWorkerWorkerArgs};

pub struct ExecuteWorkerGetCountData;
#[async_trait]
impl Task for ExecuteWorkerGetCountData {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "ExecuteWorkerGetCountData".to_string(),
            detail: "Task generator".to_string(),
        }
    }
    async fn run(&self, app_context: &AppContext, _vars: &task::Vars) -> Result<()> {
        println!("Task ExecuteWorkerGetCountData generated");
        let _ = GetCountDataWorker::perform_later(&app_context, GetCountDataWorkerWorkerArgs{
            table_column_name: "FA0003GetCountMahasiswa".to_string(),
            action_name: "GetCountMahasiswa".to_string(),
        }).await;
        Ok(())
    }
}

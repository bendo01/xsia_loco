use crate::workers::feeder_dikti::downstream::master::download::get_list_komponen_evaluasi_kelas::{
    Worker, WorkerArgs,
};
use loco_rs::prelude::*;
// use tokio::time::{Duration, sleep};
pub struct ExecuteWorkerGetListKomponenEvaluasiKelas;
#[async_trait]
impl Task for ExecuteWorkerGetListKomponenEvaluasiKelas {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "ExecuteWorkerGetListKomponenEvaluasiKelas".to_string(),
            detail: "Task generator".to_string(),
        }
    }
    async fn run(&self, app_context: &AppContext, _vars: &task::Vars) -> Result<()> {
        println!("Task ExecuteWorkerGetListKomponenEvaluasiKelas generated");
        // Enqueue the worker
        Worker::perform_later(app_context, WorkerArgs {}).await?;
        println!("Task ExecuteWorkerGetListKomponenEvaluasiKelas completed - all workers enqueued");
        Ok(())
    }
}

use crate::workers::feeder_dikti::downstream::master::get_list_komponen_evaluasi_kelas::{
    Worker, WorkerArgs,
};
use loco_rs::prelude::*;
// use tokio::time::{Duration, sleep};
pub struct ExecuteWorkerGetListMahasiswa;
#[async_trait]
impl Task for ExecuteWorkerGetListMahasiswa {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "ExecuteWorkerGetListMahasiswa".to_string(),
            detail: "Task generator".to_string(),
        }
    }
    async fn run(&self, app_context: &AppContext, _vars: &task::Vars) -> Result<()> {
        println!("Task ExecuteWorkerGetListMahasiswa generated");
        // Enqueue the worker
        Worker::perform_later(app_context, WorkerArgs {}).await?;
        println!("Task ExecuteWorkerGetListMahasiswa completed - all workers enqueued");
        Ok(())
    }
}

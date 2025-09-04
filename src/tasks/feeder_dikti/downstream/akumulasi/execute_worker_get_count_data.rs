use loco_rs::prelude::*;

pub struct ExecuteWorkerGetCountData;
#[async_trait]
impl Task for ExecuteWorkerGetCountData {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "execute_worker_get_count_data".to_string(),
            detail: "Task generator".to_string(),
        }
    }
    async fn run(&self, _app_context: &AppContext, _vars: &task::Vars) -> Result<()> {
        println!("Task ExecuteWorkerGetCountData generated");
        Ok(())
    }
}

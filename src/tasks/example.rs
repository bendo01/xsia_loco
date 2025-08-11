use loco_rs::prelude::*;

pub struct Example;
#[async_trait]
impl Task for Example {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "example".to_string(),
            detail: "Task generator".to_string(),
        }
    }
    async fn run(&self, _app_context: &AppContext, _vars: &task::Vars) -> Result<()> {
        println!("Task Example generated");
        Ok(())
    }
}

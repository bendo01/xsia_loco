use loco_rs::{hash, prelude::*};

pub struct GenerateHashPassword;
#[async_trait]
impl Task for GenerateHashPassword {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "GenerateHashPassword".to_string(),
            detail: "generate hash password".to_string(),
        }
    }
    async fn run(&self, _app_context: &AppContext, vars: &task::Vars) -> Result<()> {
        let input_password = vars.cli_arg("input_password");

        if let Ok(password) = input_password {
            let hashed_password = hash::hash_password(password.as_str());
            println!("Show Generated Hash Password: {:#?}", hashed_password);
        } else {
            println!("Please provide a password to hash");
        }
        Ok(())
    }
}

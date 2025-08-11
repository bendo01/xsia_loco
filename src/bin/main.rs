use loco_rs::cli;
use migration::Migrator;
use xsia_loco::app::App;

#[tokio::main]
async fn main() -> loco_rs::Result<()> {
    dotenvy::dotenv().expect("Failed to load .env file");
    cli::main::<App, Migrator>().await
}

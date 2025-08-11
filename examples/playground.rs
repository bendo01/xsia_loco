// use xsia_loco::services::feeder_dikti::requester::token::Token;
#[allow(unused_imports)]
use loco_rs::{cli::playground, hash, prelude::*};
//use sea_orm::{DbBackend, QueryOrder, QuerySelect, entity::*, query::*};
// use sea_orm::{ColumnTrait, EntityTrait, FromQueryResult, QueryFilter, QueryOrder, QuerySelect};
// use serde::Serialize;
// use uuid::{Uuid, uuid};
// use uuid::Uuid;
use xsia_loco::app::App;
// use xsia_loco::common::settings::Settings;
// use xsia_loco::models::academic::student::master::students::_entities::students as AcademicStudentMasterStudent;
// use xsia_loco::models::auth::roles::_entities::roles as AuthRole;
// use xsia_loco::models::auth::users::_entities::users as AuthUser;
// use xsia_loco::models::institution::master::units::_entities::units as InstitutionMasterUnit;
// use xsia_loco::models::person::master::individuals::_entities::individuals as PersonMasterIndividual;
//
use base64::{Engine as _, engine::general_purpose::STANDARD};

#[tokio::main]
async fn main() -> loco_rs::Result<()> {
    let ctx = playground::<App>().await?;

    // let active_model: articles::ActiveModel = ActiveModel {
    //     title: Set(Some("how to build apps in 3 steps".to_string())),
    //     content: Set(Some("use Loco: https://loco.rs".to_string())),
    //     ..Default::default()
    // };
    // active_model.insert(&ctx.db).await.unwrap();

    // let res = articles::Entity::find().all(&ctx.db).await.unwrap();
    // println!("{:?}", res);
    // println!("welcome to playground. edit me at `examples/playground.rs`");
    // let token_request = Token::get().await;
    // let token = match token_request {
    //     Ok(token) => token,
    //     Err(err) => {
    //         eprintln!("Error: {}", err);
    //         return Err(err.into());
    //     }
    // };
    // println!("{:?}", token);
    if let Some(settings) = &ctx.config.settings {
        println!("Settings {:#?}", settings);
    } else {
        println!("Settings Not loaded");
    }

    // encode base64
    let server_key = "SB-Mid-server-qgKZNaPCznrDBHTuaFsMNBgL";
    let auth_string = format!("{}:", server_key);
    let encoded = STANDARD.encode(auth_string);
    println!("{}", encoded);
    Ok(())
}

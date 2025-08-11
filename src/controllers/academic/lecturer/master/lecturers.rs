#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use crate::middleware::is_authorized::AuthorizedLayer;
// use crate::models::academic::lecturer::master::lecturers::_entities::lecturers as AcademicLecturerMasterLecturer;
use crate::models::academic::lecturer::master::lecturers::data_objects::LecturerDataObject;
// use axum::{Json, debug_handler, http::StatusCode};
use axum::debug_handler;
// use chrono::Utc;
use loco_rs::prelude::*;

// async fn load_item(ctx: &AppContext, id: Uuid) -> Result<AcademicLecturerMasterLecturer::Model> {
//     let item = AcademicLecturerMasterLecturer::Entity::find_by_id(id)
//         .one(&ctx.db)
//         .await?;
//     item.ok_or_else(|| Error::NotFound)
// }

#[debug_handler]
pub async fn index(State(_ctx): State<AppContext>) -> Result<Response> {
    format::empty()
}

#[debug_handler]
pub async fn index_unit(
    State(_ctx): State<AppContext>,
    Path(_unit_id): Path<Uuid>,
) -> Result<Response> {
    format::empty()
}

#[debug_handler]
pub async fn index_institution(
    State(_ctx): State<AppContext>,
    Path(_institution_id): Path<Uuid>,
) -> Result<Response> {
    format::empty()
}

#[debug_handler]
pub async fn show(State(ctx): State<AppContext>, Path(id): Path<Uuid>) -> Result<Response> {
    let lecturer = LecturerDataObject::get_by_id(&ctx, id).await?;
    #[allow(clippy::option_if_let_else)]
    match lecturer {
        Some(data) => format::json(data),
        None => Err(Error::NotFound),
    }
}

// #[debug_handler]
// pub async fn store(State(_ctx): State<AppContext>) -> Result<Response> {
//     format::empty()
// }

// #[debug_handler]
// pub async fn destroy(State(_ctx): State<AppContext>) -> Result<Response> {
//     format::empty()
// }

// #[debug_handler]
// pub async fn update(State(_ctx): State<AppContext>) -> Result<Response> {
//     format::empty()
// }

#[debug_handler]
pub async fn list(State(_ctx): State<AppContext>) -> Result<Response> {
    format::empty()
}

pub fn routes(ctx: &AppContext) -> Routes {
    Routes::new()
        .prefix("api/academic/lecturer/master/lecturers")
        .add(
            "/list",
            get(list).layer(AuthorizedLayer::new(
                ctx.clone(),
                "academic.lecturer.master.lecturers.list",
            )),
        )
        // .add(
        //     "/store",
        //     post(store).layer(AuthorizedLayer::new(
        //         ctx.clone(),
        //         "academic.lecturer.master.lecturers.index",
        //     )),
        // )
        .add(
            "/{id}",
            get(show).layer(AuthorizedLayer::new(
                ctx.clone(),
                "academic.lecturer.master.lecturers.show",
            )),
        )
        // .add(
        //     "/{id}",
        //     delete(destroy).layer(AuthorizedLayer::new(
        //         ctx.clone(),
        //         "academic.lecturer.master.lecturers.delete",
        //     )),
        // )
        // .add(
        //     "/{id}",
        //     put(update).layer(AuthorizedLayer::new(
        //         ctx.clone(),
        //         "academic.lecturer.master.lecturers.update",
        //     )),
        // )
        .add(
            "/",
            post(index).layer(AuthorizedLayer::new(
                ctx.clone(),
                "academic.lecturer.master.lecturers.index",
            )),
        )
}

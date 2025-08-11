#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::middleware::is_authenticated::AuthenticatedLayer;
use crate::models::academic::student::master::students::student_validate::StudentValidate;
use axum::debug_handler;
use loco_rs::prelude::*;

#[debug_handler]
pub async fn student_validation(
    Path(id): Path<Uuid>,
    State(ctx): State<AppContext>,
) -> Result<Response, loco_rs::Error> {
    let candidate_result = StudentValidate::check(&ctx, id).await;
    match candidate_result {
        Ok(candidate_check) => format::json(candidate_check),
        Err(err) => {
            let error_message = format!("Error checking student: {err}");
            Err(Error::Message(error_message))
        }
    }
}

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
pub async fn store(State(_ctx): State<AppContext>) -> Result<Response> {
    format::empty()
}

#[debug_handler]
pub async fn show(State(_ctx): State<AppContext>) -> Result<Response> {
    format::empty()
}

#[debug_handler]
pub async fn destroy(State(_ctx): State<AppContext>) -> Result<Response> {
    format::empty()
}

#[debug_handler]
pub async fn update(State(_ctx): State<AppContext>) -> Result<Response> {
    format::empty()
}

#[debug_handler]
pub async fn list(State(_ctx): State<AppContext>) -> Result<Response> {
    format::empty()
}

pub fn routes(ctx: &AppContext) -> Routes {
    Routes::new()
        .prefix("api/academic/student/master/students")
        .add(
            "/student_validation/{id}",
            get(student_validation).layer(AuthenticatedLayer::new(ctx.clone())),
        )
        .add("/{id}", get(show))
        .add("/{id}", delete(destroy))
        .add("/{id}", post(update))
        .add("/list", get(list))
        .add("/", get(index))
        .add("/", post(store))
}

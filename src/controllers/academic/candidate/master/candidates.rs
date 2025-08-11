#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use crate::middleware::is_authenticated::AuthenticatedLayer;
use crate::middleware::is_authorized::AuthorizedLayer;
use crate::models::academic::candidate::master::candidates::_entities::candidates as ReferenceModel;
use crate::models::academic::candidate::master::candidates::candidate_validate::CandidateValidate;
use crate::models::academic::candidate::master::candidates::data_objects::DataObject as ReferenceDataObject;
use crate::vendor::paginate::pagination::{PaginateInput, PaginateResult};
// use crate::vendor::validation::common::format_validation_errors;
use axum::{Json, debug_handler, extract::Path};
use loco_rs::prelude::*;
use sea_orm::sea_query::Expr; // Import Expr to build expressions
// use sea_orm::{ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};
use rust_xlsxwriter::Workbook;
use sea_orm::{Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};
use sea_query::extension::postgres::PgExpr;
use serde::{Deserialize, Serialize};
// use serde_json::json;

#[derive(Deserialize, Serialize)]
pub struct ModelPagination {
    pagination: PaginateResult,
    data: Vec<Option<ReferenceDataObject>>,
}

#[debug_handler]
pub async fn candidate_validation(
    Path(id): Path<Uuid>,
    State(ctx): State<AppContext>,
) -> Result<Response, loco_rs::Error> {
    // let candidate_result = CandidateValidate::check(id, loco_rs::prelude::State(ctx)).await;
    let candidate_result = CandidateValidate::check(&ctx, id).await;
    match candidate_result {
        Ok(candidate_check) => format::json(candidate_check),
        Err(err) => {
            let error_message = format!("Error checking candidate: {err}");
            Err(Error::Message(error_message))
        }
    }
}

#[debug_handler]
pub async fn candidate_chart(
    Path(_id): Path<Uuid>,
    State(_ctx): State<AppContext>,
) -> Result<Response, loco_rs::Error> {
    todo!();
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
    State(ctx): State<AppContext>,
    Path(institution_id): Path<Uuid>,
    Json(paginate_input): Json<PaginateInput>,
) -> Result<Response> {
    let mut query = ReferenceModel::Entity::find();
    // query = query.filter(
    //     Condition::any()
    //         .add(Expr::col(ReferenceModel::Column::DeletedAt).is_null())
    //         .add(Expr::col(ReferenceModel::Column::InstitutionId).eq(institution_id)),
    // );
    query = query.filter(ReferenceModel::Column::DeletedAt.is_null());
    query = query.filter(ReferenceModel::Column::InstitutionId.eq(institution_id));

    // Apply search filters
    if let Some(search) = &paginate_input.search {
        let search_pattern = format!("%{search}%");
        query = query.filter(
            Condition::any()
                .add(Expr::col(ReferenceModel::Column::Code).ilike(search_pattern.clone()))
                .add(Expr::col(ReferenceModel::Column::Name).ilike(search_pattern.clone())),
        );
    }

    // Apply sorting
    if let Some(sort_by) = &paginate_input.sort_by {
        match (sort_by.as_str(), paginate_input.sort_dir.as_deref()) {
            ("code", Some("asc")) => {
                query = query.order_by_asc(ReferenceModel::Column::Code);
            }
            ("code", Some("desc")) => {
                query = query.order_by_desc(ReferenceModel::Column::Code);
            }
            ("name", Some("asc")) => {
                query = query.order_by_asc(ReferenceModel::Column::Name);
            }
            ("name", Some("desc")) => {
                query = query.order_by_desc(ReferenceModel::Column::Name);
            }
            _ => query = query.order_by_asc(ReferenceModel::Column::Code),
        }
    }

    // Pagination logic
    let page = paginate_input.page;
    let per_page = paginate_input.per_page;

    let paginator = query.paginate(&ctx.db, per_page);
    let total_data = paginator.num_items().await?;
    let total_page = paginator.num_pages().await?;
    // let items = paginator.fetch_page(page - 1).await?;
    let datas = paginator.fetch_page(page - 1).await?;

    let mut items = Vec::new();
    for data in datas {
        let item_object = ReferenceDataObject::get_by_id(&ctx, data.id, false).await?;
        items.push(item_object);
    }

    // Create the pagination metadata
    let result = PaginateResult {
        search: paginate_input.search,
        sort_by: paginate_input.sort_by,
        column: paginate_input.column,
        sort_dir: paginate_input.sort_dir,
        page,
        per_page,
        total_page,
        last_page: total_page,
        total_data,
    };

    // Combine the data and pagination result
    let response = ModelPagination {
        pagination: result,
        data: items,
    };

    // Return the response as JSON
    format::json(response)
}

#[debug_handler]
pub async fn index_institution_excel_export(
    State(ctx): State<AppContext>,
    Path(institution_id): Path<Uuid>,
) -> Result<Response> {
    let mut query = ReferenceModel::Entity::find();
    let mut workbook = Workbook::new();

    // 2. Add a worksheet and write something
    let worksheet = workbook.add_worksheet();
    // worksheet.write_string(0, 0, "Hello from Loco!").unwrap();
    // worksheet.write_number(1, 0, 123.45).unwrap();
    // Start Set Column Header
    worksheet.write_string(0, 0, "Kode").unwrap();
    worksheet.write_string(0, 1, "Nama").unwrap();
    worksheet.write_string(0, 2, "Program Studi").unwrap();
    worksheet.write_string(0, 3, "Kabupaten").unwrap();
    worksheet.write_string(0, 4, "NISN").unwrap();
    worksheet.write_string(0, 5, "KIP").unwrap();
    worksheet.write_string(0, 6, "Pemandu").unwrap();
    // End Set Column Header
    query = query.filter(ReferenceModel::Column::DeletedAt.is_null());
    query = query.filter(ReferenceModel::Column::InstitutionId.eq(institution_id));
    query = query.order_by_asc(ReferenceModel::Column::Code);
    let datas = query.all(&ctx.db).await?;
    let mut items = Vec::new();
    for data in datas {
        let item_object = ReferenceDataObject::get_by_id(&ctx, data.id, false).await?;
        // println!("{:#?}", item_object.clone());
        items.push(item_object);
    }
    for (index, item) in items.iter().enumerate() {
        // println!("{:#?}: {:#?}", index, item);
        // println!("{:#?} {:#?}", index, item.candidate);
        // println!("{:#?}", item);
        if let Some(data_object) = item {
            // println!("Candidate code: {}", data_object.candidate.code);
            worksheet
                .write_string(
                    (index + 1).try_into().unwrap(),
                    0,
                    data_object.candidate.code.clone(),
                )
                .unwrap();
            worksheet
                .write_string(
                    (index + 1).try_into().unwrap(),
                    1,
                    data_object.candidate.name.clone(),
                )
                .unwrap();
            worksheet
                .write_string(
                    (index + 1).try_into().unwrap(),
                    2,
                    data_object.registration_type.clone().unwrap().name.clone(),
                )
                .unwrap();
            worksheet
                .write_string(
                    (index + 1).try_into().unwrap(),
                    3,
                    data_object.regency.clone().unwrap().name.clone(),
                )
                .unwrap();
            if let Some(student_national_number) = &data_object.candidate.student_national_number {
                worksheet
                    .write_string((index + 1).try_into().unwrap(), 4, student_national_number)
                    .unwrap();
            }

            if let Some(state_smart_card_number) = &data_object.candidate.state_smart_card_number {
                worksheet
                    .write_string((index + 1).try_into().unwrap(), 5, state_smart_card_number)
                    .unwrap();
            }

            // To this
            if let Some(guidance_name) = &data_object.candidate.guidence_name {
                worksheet
                    .write_string((index + 1).try_into().unwrap(), 6, guidance_name)
                    .unwrap();
            }
        }
    }

    // 3. Save to buffer (Vec<u8>) instead of writing to disk
    let buffer = workbook
        .save_to_buffer()
        .map_err(|e| Error::Message(format!("Excel error: {e}")))?;

    // 4. Return as downloadable file in HTTP response
    let response = Response::builder()
        .status(200)
        .header(
            "Content-Type",
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        )
        .header(
            "Content-Disposition",
            "attachment; filename=\"daftar_kandidat.xlsx\"",
        )
        .body(buffer.into())?;

    Ok(response)
}

pub fn routes(ctx: &AppContext) -> Routes {
    Routes::new()
        .prefix("api/academic/candidate/master/candidates")
        .add(
            "/candidate_validation/{id}",
            get(candidate_validation).layer(AuthenticatedLayer::new(ctx.clone())),
        )
        // .add(
        //     "/index_institution/{id}",
        //     get(index_institution).layer(AuthenticatedLayer::new(ctx.clone())),
        // )
        .add(
            "/index_institution/{id}",
            post(index_institution).layer(AuthorizedLayer::new(
                ctx.clone(),
                "academic.candidate.master.candidates.index.institution",
            )),
        )
        // .add(
        //     "/index_institution_excel_export/{institution_id}",
        //     get(index_institution_excel_export),
        // )
        .add(
            "/index_institution_excel_export/{institution_id}",
            post(index_institution_excel_export).layer(AuthorizedLayer::new(
                ctx.clone(),
                "academic.candidate.master.candidates.index.institution_excel_export",
            )),
        )
}

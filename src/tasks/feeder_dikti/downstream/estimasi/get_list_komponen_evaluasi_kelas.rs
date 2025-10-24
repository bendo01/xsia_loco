use crate::common::settings::Settings;
use crate::models::feeder::akumulasi::estimasi::_entities::estimasi as FeederAkumulasiEstimasi;
use crate::models::feeder::master::komponen_evaluasi_kelas::feeder_model::ModelInput as FeederModel;
use crate::tasks::feeder_dikti::downstream::request_only_data::{InputRequestData, RequestData};
use chrono::Local;
use futures::future::try_join_all;
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set, TransactionTrait};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::sleep;
use uuid::Uuid;

// Configuration constants
const TASK_NAME: &str = "EstimateKomponenEvaluasiKelas";
const API_ACTION: &str = "GetListKomponenEvaluasiKelas";
const DEFAULT_LIMIT: i32 = 100;
const MAX_CONCURRENT_WORKERS: usize = 10;
const RETRY_DELAY_MS: u64 = 1000;

#[derive(Deserialize, Debug, Serialize)]
pub struct WorkerArgs {
    pub act: String,
    pub filter: Option<String>,
    pub order: Option<String>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Debug)]
pub enum TaskError {
    SettingsNotLoaded,
    InvalidInstitutionId(String),
    DatabaseError(String),
    RequestError(String),
    WorkerEnqueueError(String),
}

impl From<TaskError> for Error {
    fn from(err: TaskError) -> Self {
        match err {
            TaskError::SettingsNotLoaded => Error::Message("Settings not loaded".to_string()),
            TaskError::InvalidInstitutionId(msg) => {
                Error::Message(format!("Invalid institution ID: {}", msg))
            }
            TaskError::DatabaseError(msg) => Error::Message(format!("Database error: {}", msg)),
            TaskError::RequestError(msg) => Error::Message(format!("Request error: {}", msg)),
            TaskError::WorkerEnqueueError(msg) => {
                Error::Message(format!("Worker enqueue error: {}", msg))
            }
        }
    }
}

pub struct EstimateKomponenEvaluasiKelas;

impl EstimateKomponenEvaluasiKelas {
    /// Extract institution ID from app context settings
    fn get_institution_id(app_context: &AppContext) -> Result<Uuid, TaskError> {
        let current_settings = app_context
            .config
            .settings
            .as_ref()
            .ok_or(TaskError::SettingsNotLoaded)?;

        let settings = Settings::from_json(current_settings)
            .map_err(|e| TaskError::InvalidInstitutionId(e.to_string()))?;

        Uuid::parse_str(&settings.current_institution_id)
            .map_err(|e| TaskError::InvalidInstitutionId(e.to_string()))
    }

    /// Find existing progress tracking record
    async fn find_progress_record(
        app_context: &AppContext,
        institution_id: Uuid,
    ) -> Result<Option<FeederAkumulasiEstimasi::Model>, TaskError> {
        FeederAkumulasiEstimasi::Entity::find()
            .filter(FeederAkumulasiEstimasi::Column::DeletedAt.is_null())
            .filter(FeederAkumulasiEstimasi::Column::InstitutionId.eq(institution_id))
            .filter(FeederAkumulasiEstimasi::Column::Name.eq(TASK_NAME))
            .one(&app_context.db)
            .await
            .map_err(|e| TaskError::DatabaseError(e.to_string()))
    }

    /// Reset existing progress record or create new one
    async fn initialize_progress_record(
        app_context: &AppContext,
        institution_id: Uuid,
        existing_record: Option<FeederAkumulasiEstimasi::Model>,
    ) -> Result<i32, TaskError> {
        let txn = app_context
            .db
            .begin()
            .await
            .map_err(|e| TaskError::DatabaseError(e.to_string()))?;

        let limit = match existing_record {
            Some(record) => {
                // Reset existing record
                let limit = record.total_data_per_request;
                let mut active: FeederAkumulasiEstimasi::ActiveModel = record.into_active_model();
                active.last_offset = Set(0);
                active.total_data = Set(0);
                active.updated_at = Set(Local::now().naive_local());

                active
                    .update(&txn)
                    .await
                    .map_err(|e| TaskError::DatabaseError(e.to_string()))?;

                println!("Reset existing {} progress record", TASK_NAME);
                limit
            }
            None => {
                // Create new record
                // let pk_id = Uuid::from(uuid7::uuid7());
                let uuid_v7 = uuid7::uuid7();
                let uuid_string = uuid_v7.to_string();
                let pk_id: Uuid = Uuid::parse_str(&uuid_string).expect("Invalid UUID string"); // Handle parsing errors appropriately

                let new_record = FeederAkumulasiEstimasi::ActiveModel {
                    id: Set(pk_id),
                    institution_id: Set(institution_id),
                    name: Set(TASK_NAME.to_string()),
                    total_data_per_request: Set(DEFAULT_LIMIT),
                    last_offset: Set(0),
                    total_data: Set(0),
                    ..Default::default()
                };

                new_record
                    .insert(&txn)
                    .await
                    .map_err(|e| TaskError::DatabaseError(e.to_string()))?;

                println!(
                    "Created new {} progress record for institution {}",
                    TASK_NAME, institution_id
                );
                DEFAULT_LIMIT
            }
        };

        txn.commit()
            .await
            .map_err(|e| TaskError::DatabaseError(e.to_string()))?;

        Ok(limit)
    }

    /// Update progress tracking record
    async fn update_progress(
        app_context: &AppContext,
        institution_id: Uuid,
        offset: i32,
        limit: i32,
        processed_count: i32,
    ) -> Result<(), TaskError> {
        let txn = app_context
            .db
            .begin()
            .await
            .map_err(|e| TaskError::DatabaseError(e.to_string()))?;

        let record = FeederAkumulasiEstimasi::Entity::find()
            .filter(FeederAkumulasiEstimasi::Column::DeletedAt.is_null())
            .filter(FeederAkumulasiEstimasi::Column::InstitutionId.eq(institution_id))
            .filter(FeederAkumulasiEstimasi::Column::Name.eq(TASK_NAME))
            .one(&txn)
            .await
            .map_err(|e| TaskError::DatabaseError(e.to_string()))?;

        if let Some(record) = record {
            let mut active: FeederAkumulasiEstimasi::ActiveModel = record.into_active_model();
            active.total_data = Set(active.total_data.as_ref() + processed_count);
            active.last_offset = Set(offset + limit);
            active.updated_at = Set(Local::now().naive_local());

            active
                .update(&txn)
                .await
                .map_err(|e| TaskError::DatabaseError(e.to_string()))?;
        }

        txn.commit()
            .await
            .map_err(|e| TaskError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    /// Enqueue workers for a batch of records with concurrency control
    async fn enqueue_workers(
        app_context: &AppContext,
        data: Vec<FeederModel>,
        offset: i32,
    ) -> Result<(), TaskError> {
        let chunks: Vec<_> = data
            .into_iter()
            .enumerate()
            .collect::<Vec<_>>()
            .chunks(MAX_CONCURRENT_WORKERS)
            .map(|chunk| chunk.to_vec())
            .collect();

        for chunk in chunks {
            let futures = chunk.into_iter().map(|(index, obj)| {
                let worker_args = crate::workers::feeder_dikti::downstream::master::upsert::get_list_komponen_evaluasi_kelas::WorkerArgs {
                    feeder_model: obj,
                };

                async move {
                    match crate::workers::feeder_dikti::downstream::master::upsert::get_list_komponen_evaluasi_kelas::Worker::perform_later(app_context, worker_args).await {
                        Ok(_) => {
                            println!("✅ Enqueued upsert worker for item {} (offset {})", index, offset + index as i32);
                            Ok(())
                        }
                        Err(err) => {
                            eprintln!("❌ Failed to enqueue upsert worker for item {} (offset {}): {:?}", index, offset + index as i32, err);
                            Err(TaskError::WorkerEnqueueError(err.to_string()))
                        }
                    }
                }
            });

            // Process chunk concurrently, but fail fast if any worker fails
            try_join_all(futures).await?;

            // Small delay between chunks to prevent overwhelming the system
            sleep(Duration::from_millis(RETRY_DELAY_MS / 10)).await;
        }

        Ok(())
    }

    /// Fetch a page of data from the API
    async fn fetch_page(
        app_context: &AppContext,
        limit: i32,
        offset: i32,
    ) -> Result<Option<Vec<FeederModel>>, TaskError> {
        let response = RequestData::get::<FeederModel>(
            app_context,
            InputRequestData {
                act: API_ACTION.to_string(),
                filter: None,
                order: None,
                limit: Some(limit),
                offset: Some(offset),
            },
        )
        .await
        .map_err(|e| TaskError::RequestError(e.to_string()))?;

        match response.data {
            Some(data) if !data.is_empty() => Ok(Some(data)),
            _ => Ok(None),
        }
    }

    /// Main pagination loop
    async fn process_paginated_data(
        app_context: &AppContext,
        institution_id: Uuid,
        limit: i32,
    ) -> Result<(), TaskError> {
        let mut offset = 0;
        let mut total_processed = 0;

        loop {
            println!("Fetching page at offset {} with limit {}", offset, limit);

            match Self::fetch_page(app_context, limit, offset).await? {
                Some(data) => {
                    let count = data.len() as i32;
                    println!("Received {} records (offset={})", count, offset);

                    // Enqueue workers for this batch
                    Self::enqueue_workers(app_context, data, offset).await?;

                    // Update progress
                    Self::update_progress(app_context, institution_id, offset, limit, count)
                        .await?;

                    total_processed += count;
                    offset += limit;

                    // Continue to next page
                }
                None => {
                    println!("No more data returned from feeder (offset={})", offset);
                    break;
                }
            }
        }

        println!("Completed processing {} total records", total_processed);
        Ok(())
    }
}

#[async_trait]
impl Task for EstimateKomponenEvaluasiKelas {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: TASK_NAME.to_string(),
            detail: "Fetch and process Komponen Evaluasi Kelas data from Feeder Dikti".to_string(),
        }
    }

    async fn run(&self, app_context: &AppContext, _vars: &task::Vars) -> Result<()> {
        println!("Starting {} task", TASK_NAME);

        // Get institution ID
        let institution_id = Self::get_institution_id(app_context)?;

        // Find existing progress record
        let existing_record = Self::find_progress_record(app_context, institution_id).await?;

        // Initialize/reset progress tracking
        let limit =
            Self::initialize_progress_record(app_context, institution_id, existing_record).await?;

        // Process all pages
        Self::process_paginated_data(app_context, institution_id, limit).await?;

        println!("✅ {} task completed successfully", TASK_NAME);
        Ok(())
    }
}

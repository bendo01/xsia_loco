use crate::common::settings::Settings;
use crate::models::feeder::akumulasi::estimasi::_entities::estimasi as FeederAkumulasiEstimasi;
use chrono::Local;
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set, TransactionTrait};

// Configuration constants
const TASK_NAME: &str = "EstimateAktifitasMengajarDosen";
const API_ACTION: &str = "GetAktivitasMengajarDosen";

// API Request Configuration
const DEFAULT_LIMIT: i32 = 1000; // Records per API request page
const DEFAULT_ORDER: &str = "id_registrasi_dosen ASC"; // Sort order for API results
const DEFAULT_FILTER: &str = ""; // Filter criteria (empty = no filter)

// Worker Configuration
// const MAX_CONCURRENT_WORKERS: usize = 10; // Max concurrent worker jobs
// const RETRY_DELAY_MS: u64 = 1000; // Delay between worker chunks (ms)

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

pub struct EstimateAktifitasMengajarDosen;

impl EstimateAktifitasMengajarDosen {
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
                let uuid_v7 = uuid7::uuid7();
                let uuid_string = uuid_v7.to_string();
                let pk_id: Uuid = Uuid::parse_str(&uuid_string).expect("Invalid UUID string");

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

    /// Fetch data and enqueue a worker for a batch of records
    async fn enqueue_worker(
        app_context: &AppContext,
        limit: i32,
        offset: i32,
    ) -> Result<(), TaskError> {
        use crate::models::feeder::master::aktifitas_mengajar_dosen::feeder_model::ModelInput as FeederModel;
        use crate::tasks::feeder_dikti::downstream::feeder_request::{
            InputRequestData, RequestData,
        };

        println!("🔄 Fetching data for offset={}, limit={}", offset, limit);

        // Fetch data from API
        let response = RequestData::get::<FeederModel>(
            app_context,
            InputRequestData {
                act: API_ACTION.to_string(),
                filter: Some(DEFAULT_FILTER.to_string()),
                order: Some(DEFAULT_ORDER.to_string()),
                limit: Some(limit),
                offset: Some(offset),
            },
        )
        .await
        .map_err(|e| TaskError::RequestError(e.to_string()))?;

        // Check for API error
        if let Some(error_desc) = &response.error_desc {
            if !error_desc.is_empty() {
                return Err(TaskError::RequestError(format!(
                    "API error (code: {}): {}",
                    response.error_code, error_desc
                )));
            }
        }

        // Extract records from response
        let records = response.data.unwrap_or_default();

        if records.is_empty() {
            println!("📭 No records found at offset={}", offset);
            return Ok(());
        }

        println!("📦 Fetched {} records at offset={}", records.len(), offset);

        // Enqueue worker with actual data
        let worker_args = crate::workers::feeder_dikti::downstream::master::upsert::get_aktifitas_mengajar_dosen::WorkerArgs {
            records,
        };

        match crate::workers::feeder_dikti::downstream::master::upsert::get_aktifitas_mengajar_dosen::Worker::perform_later(app_context, worker_args).await {
            Ok(_) => {
                println!("✅ Enqueued worker for offset={}", offset);
                Ok(())
            }
            Err(err) => {
                eprintln!("❌ Failed to enqueue worker for offset={}: {:?}", offset, err);
                Err(TaskError::WorkerEnqueueError(err.to_string()))
            }
        }
    }

    /// Check if there's data available at the given offset (lightweight check)
    async fn has_data_at_offset(
        app_context: &AppContext,
        _limit: i32,
        offset: i32,
    ) -> Result<bool, TaskError> {
        use crate::models::feeder::master::aktifitas_mengajar_dosen::feeder_model::ModelInput as FeederModel;
        use crate::tasks::feeder_dikti::downstream::feeder_request::{
            InputRequestData, RequestData,
        };

        println!("🔍 Checking data availability at offset={}", offset);

        let response = RequestData::get::<FeederModel>(
            app_context,
            InputRequestData {
                act: API_ACTION.to_string(),
                filter: Some(DEFAULT_FILTER.to_string()),
                order: Some(DEFAULT_ORDER.to_string()),
                limit: Some(1), // Only fetch 1 record to check
                offset: Some(offset),
            },
        )
        .await
        .map_err(|e| TaskError::RequestError(e.to_string()))?;

        // Check for API error
        if let Some(error_desc) = &response.error_desc {
            if !error_desc.is_empty() {
                println!(
                    "⚠️  API returned error (error_code: {}): {}",
                    response.error_code, error_desc
                );
                return Ok(false);
            }
        }

        // Check if data exists
        let has_data = response.data.map_or(false, |d| !d.is_empty());

        if has_data {
            println!("✅ Data available at offset={}", offset);
        } else {
            println!("📭 No data at offset={}", offset);
        }

        Ok(has_data)
    }

    /// Main pagination loop - enqueue workers for each page
    async fn process_paginated_data(
        app_context: &AppContext,
        institution_id: Uuid,
        limit: i32,
    ) -> Result<(), TaskError> {
        let mut offset = 0;
        let mut total_workers_enqueued = 0;
        let mut page_number = 1;

        loop {
            println!(
                "📄 Page {}: Checking offset={}, limit={}",
                page_number, offset, limit
            );

            // Check if there's data at this offset
            let has_data = Self::has_data_at_offset(app_context, limit, offset).await?;

            if !has_data {
                println!(
                    "✅ Pagination completed at offset={}, total workers enqueued: {}",
                    offset, total_workers_enqueued
                );
                break;
            }

            // Fetch data and enqueue worker to process this batch
            Self::enqueue_worker(app_context, limit, offset).await?;

            // Update progress with limit (actual count will be verified by worker)
            Self::update_progress(app_context, institution_id, offset, limit, limit).await?;

            total_workers_enqueued += 1;
            offset += limit;
            page_number += 1;
        }

        println!(
            "🎉 Completed enqueueing {} workers across {} pages",
            total_workers_enqueued,
            page_number - 1
        );
        Ok(())
    }
}

#[async_trait]
impl Task for EstimateAktifitasMengajarDosen {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: TASK_NAME.to_string(),
            detail: "Fetch and process Aktifitas Mengajar Dosen data from Feeder Dikti".to_string(),
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

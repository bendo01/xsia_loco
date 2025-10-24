use chrono::Local;
use colored::Colorize;
use loco_rs::prelude::*;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter, Set,
    TransactionTrait, TryIntoModel,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::feeder::master::komponen_evaluasi_kelas::_entities::komponen_evaluasi_kelas as FeederMasterKomponenEvaluasiKelas;
use crate::models::feeder::master::komponen_evaluasi_kelas::feeder_model::ModelInput as FeederModel;

// Configuration constants
const WORKER_NAME: &str = "GetListKomponenEvaluasiKelas";

#[derive(Debug)]
pub enum UpsertError {
    MissingRequiredField(String),
    DatabaseError(String),
    ValidationError(String),
}

impl From<UpsertError> for Error {
    fn from(err: UpsertError) -> Self {
        match err {
            UpsertError::MissingRequiredField(field) => {
                Error::Message(format!("Missing required field: {}", field))
            }
            UpsertError::DatabaseError(msg) => Error::Message(format!("Database error: {}", msg)),
            UpsertError::ValidationError(msg) => {
                Error::Message(format!("Validation error: {}", msg))
            }
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputData;

impl InputData {
    /// Validate required fields from FeederModel
    fn validate_input(input: &FeederModel) -> Result<(Uuid, Uuid, Uuid), UpsertError> {
        let id_komponen_evaluasi = input
            .id_komponen_evaluasi
            .ok_or_else(|| UpsertError::MissingRequiredField("id_komponen_evaluasi".to_string()))?;

        let id_kelas_kuliah = input
            .id_kelas_kuliah
            .ok_or_else(|| UpsertError::MissingRequiredField("id_kelas_kuliah".to_string()))?;

        let id_jenis_evaluasi = input
            .id_jenis_evaluasi
            .ok_or_else(|| UpsertError::MissingRequiredField("id_jenis_evaluasi".to_string()))?;

        Ok((id_komponen_evaluasi, id_kelas_kuliah, id_jenis_evaluasi))
    }

    /// Find existing record by composite key
    async fn find_existing_record(
        ctx: &AppContext,
        id_komponen_evaluasi: Uuid,
        id_kelas_kuliah: Uuid,
        id_jenis_evaluasi: Uuid,
    ) -> Result<Option<FeederMasterKomponenEvaluasiKelas::Model>, UpsertError> {
        FeederMasterKomponenEvaluasiKelas::Entity::find()
            .filter(FeederMasterKomponenEvaluasiKelas::Column::DeletedAt.is_null())
            .filter(
                FeederMasterKomponenEvaluasiKelas::Column::IdKomponenEvaluasi
                    .eq(id_komponen_evaluasi),
            )
            .filter(FeederMasterKomponenEvaluasiKelas::Column::IdKelasKuliah.eq(id_kelas_kuliah))
            .filter(
                FeederMasterKomponenEvaluasiKelas::Column::IdJenisEvaluasi.eq(id_jenis_evaluasi),
            )
            .one(&ctx.db)
            .await
            .map_err(|e| UpsertError::DatabaseError(e.to_string()))
    }

    /// Create ActiveModel from FeederModel input
    fn create_active_model(
        input: &FeederModel,
        id_komponen_evaluasi: Uuid,
        id_kelas_kuliah: Uuid,
        id_jenis_evaluasi: Uuid,
        existing_id: Option<Uuid>,
    ) -> FeederMasterKomponenEvaluasiKelas::ActiveModel {
        let now = Local::now().naive_local();
        let today = now.date();

        let mut active_model = FeederMasterKomponenEvaluasiKelas::ActiveModel {
            id_komponen_evaluasi: Set(id_komponen_evaluasi),
            id_kelas_kuliah: Set(id_kelas_kuliah),
            id_jenis_evaluasi: Set(id_jenis_evaluasi),
            nomor_urut: Set(input.nomor_urut.unwrap_or_default()),
            bobot_evaluasi: Set(input
                .bobot_evaluasi
                .map(|v| v.to_string())
                .unwrap_or_default()),
            nama: Set(input.nama.clone()),
            nama_inggris: Set(input.nama_inggris.clone()),
            last_update: Set(input.last_update.unwrap_or(today)),
            tgl_create: Set(input.tgl_create.unwrap_or(today)),
            ..Default::default()
        };

        if let Some(id) = existing_id {
            // Updating existing record
            active_model.id = Set(id);
            active_model.updated_at = Set(Some(now));
            active_model.sync_at = Set(Some(now));
        } else {
            // Creating new record
            let pk_id = Uuid::from(uuid7::uuid7());
            active_model.id = Set(pk_id);
            active_model.created_at = Set(Some(now));
            active_model.updated_at = Set(Some(now));
            active_model.sync_at = Set(Some(now));
        }

        active_model
    }

    /// Perform upsert operation with transaction safety
    pub async fn upsert(ctx: &AppContext, input: FeederModel) -> Result<(), Error> {
        // Validate input
        let (id_komponen_evaluasi, id_kelas_kuliah, id_jenis_evaluasi) =
            Self::validate_input(&input)?;

        // Use transaction for atomic operation
        let txn = ctx
            .db
            .begin()
            .await
            .map_err(|e| UpsertError::DatabaseError(e.to_string()))?;

        // Check if record exists
        let existing_record = Self::find_existing_record(
            ctx,
            id_komponen_evaluasi,
            id_kelas_kuliah,
            id_jenis_evaluasi,
        )
        .await?;

        let (operation, result) = match existing_record {
            Some(existing) => {
                // Update existing record
                let active_model = Self::create_active_model(
                    &input,
                    id_komponen_evaluasi,
                    id_kelas_kuliah,
                    id_jenis_evaluasi,
                    Some(existing.id),
                );

                let update_result = active_model.update(&txn).await;
                ("updated", update_result)
            }
            None => {
                // Insert new record
                let active_model = Self::create_active_model(
                    &input,
                    id_komponen_evaluasi,
                    id_kelas_kuliah,
                    id_jenis_evaluasi,
                    None,
                );

                let insert_result = active_model.insert(&txn).await;
                ("inserted", insert_result)
            }
        };

        // Handle the result
        match result {
            Ok(_) => {
                // Commit transaction
                txn.commit()
                    .await
                    .map_err(|e| UpsertError::DatabaseError(e.to_string()))?;

                println!("{}", format!("✅ Data {} successfully", operation).green());
                Ok(())
            }
            Err(err) => {
                // Transaction will auto-rollback on drop
                Err(
                    UpsertError::DatabaseError(format!("Failed to {} record: {}", operation, err))
                        .into(),
                )
            }
        }
    }

    /// Batch upsert for multiple records (for future use)
    #[allow(dead_code)]
    pub async fn batch_upsert(ctx: &AppContext, inputs: Vec<FeederModel>) -> Result<(), Error> {
        if inputs.is_empty() {
            return Ok(());
        }

        let txn = ctx
            .db
            .begin()
            .await
            .map_err(|e| UpsertError::DatabaseError(e.to_string()))?;

        let mut success_count = 0;
        let mut error_count = 0;

        for (index, input) in inputs.iter().enumerate() {
            match Self::validate_input(input) {
                Ok((id_komponen_evaluasi, id_kelas_kuliah, id_jenis_evaluasi)) => {
                    let existing_record = Self::find_existing_record(
                        ctx,
                        id_komponen_evaluasi,
                        id_kelas_kuliah,
                        id_jenis_evaluasi,
                    )
                    .await;

                    match existing_record {
                        Ok(existing) => {
                            let active_model = Self::create_active_model(
                                input,
                                id_komponen_evaluasi,
                                id_kelas_kuliah,
                                id_jenis_evaluasi,
                                existing.map(|e| e.id),
                            );

                            let operation_result = if existing.is_some() {
                                active_model.update(&txn).await
                            } else {
                                active_model.insert(&txn).await
                            };

                            match operation_result {
                                Ok(_) => {
                                    success_count += 1;
                                    if index % 10 == 0 {
                                        println!("📊 Processed {} records", index + 1);
                                    }
                                }
                                Err(err) => {
                                    error_count += 1;
                                    eprintln!("❌ Failed to process record {}: {}", index, err);
                                }
                            }
                        }
                        Err(err) => {
                            error_count += 1;
                            eprintln!("❌ Failed to query record {}: {:?}", index, err);
                        }
                    }
                }
                Err(err) => {
                    error_count += 1;
                    eprintln!("❌ Invalid input for record {}: {:?}", index, err);
                }
            }
        }

        // Commit transaction if we have any successes
        if success_count > 0 {
            txn.commit()
                .await
                .map_err(|e| UpsertError::DatabaseError(e.to_string()))?;
        }

        println!(
            "{}",
            format!(
                "✅ Batch completed: {} successful, {} errors",
                success_count, error_count
            )
            .green()
        );

        Ok(())
    }
}

pub struct Worker {
    pub ctx: AppContext,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct WorkerArgs {
    pub feeder_model: FeederModel,
}

#[async_trait]
impl BackgroundWorker<WorkerArgs> for Worker {
    /// Creates a new instance of the Worker with the given application context.
    ///
    /// This function is called when registering the worker with the queue system.
    ///
    /// # Parameters
    /// * `ctx` - The application context containing shared resources
    fn build(ctx: &AppContext) -> Self {
        Self { ctx: ctx.clone() }
    }

    /// Returns the class name of the worker.
    ///
    /// This name is used when enqueueing jobs and identifying the worker in logs.
    fn class_name() -> String {
        WORKER_NAME.to_string()
    }

    /// Returns tags associated with this worker.
    ///
    /// Tags can be used to filter which workers run during startup.
    fn tags() -> Vec<String> {
        vec!["feeder_dikti".to_string(), "master_data".to_string()]
    }

    /// Performs the actual work when a job is processed.
    ///
    /// This is the main function that contains the worker's logic.
    /// It gets executed when a job is dequeued from the job queue.
    ///
    /// # Returns
    /// * `Result<()>` - Ok if the job completed successfully, Err otherwise
    async fn perform(&self, args: WorkerArgs) -> Result<()> {
        match InputData::upsert(&self.ctx, args.feeder_model).await {
            Ok(_) => {
                println!("🎯 Worker {} completed successfully", WORKER_NAME);
                Ok(())
            }
            Err(e) => {
                eprintln!("💥 Worker {} failed: {}", WORKER_NAME, e);
                // Return error to mark job as failed
                Err(e)
            }
        }
    }
}

use chrono::Local;
use loco_rs::prelude::*;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, TransactionTrait};
use serde::{Deserialize, Serialize};

use crate::models::feeder::master::kurikulum::{_entities::kurikulum, feeder_model::ModelInputDetailKurikulum};

pub struct Worker {
    pub ctx: AppContext,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct WorkerArgs {
    pub records: Vec<ModelInputDetailKurikulum>,
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
    /// The implementation returns the struct name as a string.
    fn class_name() -> String {
        "GetDetailKurikulum".to_string()
    }

    /// Returns tags associated with this worker.
    ///
    /// Tags can be used to filter which workers run during startup.
    /// The default implementation returns an empty vector (no tags).
    fn tags() -> Vec<String> {
        Vec::new()
    }

    /// Performs the actual work when a job is processed.
    ///
    /// This function processes a batch of kurikulum detail records:
    /// 1. Converts each `ModelInputDetailKurikulum` to database entity
    /// 2. Checks if record exists by `id_kurikulum`
    /// 3. Updates existing record or inserts new one
    /// 4. Tracks sync timestamp
    ///
    /// # Returns
    /// * `Result<()>` - Ok if all records processed successfully, Err otherwise
    async fn perform(&self, args: WorkerArgs) -> Result<()> {
        println!("=================GetDetailKurikulum=======================");
        println!("📦 Processing batch of {} records", args.records.len());

        let mut success_count = 0;
        let mut error_count = 0;

        for (index, record) in args.records.iter().enumerate() {
            match Self::upsert_record(&self.ctx, record).await {
                Ok(action) => {
                    success_count += 1;
                    println!(
                        "  ✅ Record {}/{}: {} - id_kurikulum={}, nama_kurikulum={}",
                        index + 1,
                        args.records.len(),
                        action,
                        record.id_kurikulum,
                        record.nama_kurikulum
                    );
                }
                Err(e) => {
                    error_count += 1;
                    eprintln!(
                        "  ❌ Record {}/{}: Failed - id_kurikulum={}, nama_kurikulum={}, error: {}",
                        index + 1,
                        args.records.len(),
                        record.id_kurikulum,
                        record.nama_kurikulum,
                        e
                    );
                }
            }
        }

        println!(
            "✅ Batch complete: {} successful, {} errors out of {} total",
            success_count,
            error_count,
            args.records.len()
        );

        if error_count > 0 {
            return Err(Error::Message(format!(
                "Failed to process {} out of {} records",
                error_count,
                args.records.len()
            )));
        }

        Ok(())
    }
}

impl Worker {
    /// Upsert a single kurikulum detail record into the database.
    ///
    /// This function performs an "upsert" operation:
    /// - If a record with the same `id_kurikulum` exists, it updates it
    /// - If no record exists, it inserts a new one
    ///
    /// # Parameters
    /// * `ctx` - Application context for database access
    /// * `record` - The feeder model data to upsert
    ///
    /// # Returns
    /// * `Result<String>` - "INSERTED" or "UPDATED" on success, error otherwise
    async fn upsert_record(ctx: &AppContext, record: &ModelInputDetailKurikulum) -> Result<String> {
        // id_kurikulum is required (not Option in ModelInputDetailKurikulum)
        let id_kurikulum = record.id_kurikulum;

        // nama_kurikulum is required (not Option in ModelInputDetailKurikulum)
        let nama_kurikulum = &record.nama_kurikulum;

        // Start transaction
        let txn = ctx.db.begin().await?;
        let sync_time = Local::now().naive_local();

        // Check if record exists
        let existing = kurikulum::Entity::find()
            .filter(kurikulum::Column::DeletedAt.is_null())
            .filter(kurikulum::Column::IdKurikulum.eq(id_kurikulum))
            .one(&txn)
            .await?;

        let action = if let Some(existing_record) = existing {
            // Update existing record
            let mut active: kurikulum::ActiveModel = existing_record.into_active_model();

            active.nama_kurikulum = Set(Some(nama_kurikulum.clone()));
            active.id_prodi = Set(Some(record.id_prodi));
            active.nama_program_studi = Set(Some(record.nama_program_studi.clone()));
            active.id_semester = Set(Some(record.id_semester.clone()));
            active.semester_mulai_berlaku = Set(Some(record.semester_mulai_berlaku.clone()));
            active.jumlah_sks_lulus = Set(record.jumlah_sks_lulus);
            active.jumlah_sks_wajib = Set(record.jumlah_sks_wajib);
            active.jumlah_sks_pilihan = Set(record.jumlah_sks_pilihan);
            active.status_sync = Set(Some(record.status_sync.clone()));
            active.sync_at = Set(Some(sync_time));
            active.updated_at = Set(Some(sync_time));

            active.update(&txn).await?;
            "UPDATED"
        } else {
            // Insert new record
            let uuid_v7 = uuid7::uuid7();
            let uuid_string = uuid_v7.to_string();
            let pk_id = uuid::Uuid::parse_str(&uuid_string)
                .map_err(|e| Error::Message(format!("Invalid UUID: {}", e)))?;

            let new_record = kurikulum::ActiveModel {
                id: Set(pk_id),
                id_kurikulum: Set(Some(id_kurikulum)),
                nama_kurikulum: Set(Some(nama_kurikulum.clone())),
                id_prodi: Set(Some(record.id_prodi)),
                nama_program_studi: Set(Some(record.nama_program_studi.clone())),
                id_jenj_didik: Set(None),
                jml_sem_normal: Set(None),
                id_semester: Set(Some(record.id_semester.clone())),
                semester_mulai_berlaku: Set(Some(record.semester_mulai_berlaku.clone())),
                jumlah_sks_lulus: Set(record.jumlah_sks_lulus),
                jumlah_sks_wajib: Set(record.jumlah_sks_wajib),
                jumlah_sks_pilihan: Set(record.jumlah_sks_pilihan),
                jumlah_sks_mata_kuliah_wajib: Set(None),
                jumlah_sks_mata_kuliah_pilihan: Set(None),
                status_sync: Set(Some(record.status_sync.clone())),
                sync_at: Set(Some(sync_time)),
                created_at: Set(Some(sync_time)),
                updated_at: Set(Some(sync_time)),
                created_by: Set(None),
                updated_by: Set(None),
                deleted_at: Set(None),
            };

            new_record.insert(&txn).await?;
            "INSERTED"
        };

        // Commit transaction
        txn.commit().await?;

        Ok(action.to_string())
    }
}
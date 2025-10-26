use chrono::Local;
use loco_rs::prelude::*;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, TransactionTrait};
use serde::{Deserialize, Serialize};

use crate::models::feeder::master::dosen::{_entities::dosen, feeder_model::ModelInput};

pub struct Worker {
    pub ctx: AppContext,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct WorkerArgs {
    pub records: Vec<ModelInput>,
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
        "GetListDosen".to_string()
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
    /// This function processes a batch of dosen records:
    /// 1. Converts each `ModelInput` to database entity
    /// 2. Checks if record exists by `id_dosen`
    /// 3. Updates existing record or inserts new one
    /// 4. Tracks sync timestamp
    ///
    /// # Returns
    /// * `Result<()>` - Ok if all records processed successfully, Err otherwise
    async fn perform(&self, args: WorkerArgs) -> Result<()> {
        println!("=================GetListDosen=======================");
        println!("📦 Processing batch of {} records", args.records.len());

        let mut success_count = 0;
        let mut error_count = 0;

        for (index, record) in args.records.iter().enumerate() {
            match Self::upsert_record(&self.ctx, record).await {
                Ok(action) => {
                    success_count += 1;
                    println!(
                        "  ✅ Record {}/{}: {} - id_dosen={}, nidn={:?}",
                        index + 1,
                        args.records.len(),
                        action,
                        record.id_dosen,
                        record.nidn
                    );
                }
                Err(e) => {
                    error_count += 1;
                    eprintln!(
                        "  ❌ Record {}/{}: Failed - id_dosen={}, nidn={:?}, error: {}",
                        index + 1,
                        args.records.len(),
                        record.id_dosen,
                        record.nidn,
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
    /// Upsert a single dosen record into the database.
    ///
    /// This function performs an "upsert" operation:
    /// - If a record with the same `id_dosen` exists, it updates it
    /// - If no record exists, it inserts a new one
    ///
    /// # Parameters
    /// * `ctx` - Application context for database access
    /// * `record` - The feeder model data to upsert
    ///
    /// # Returns
    /// * `Result<String>` - "INSERTED" or "UPDATED" on success, error otherwise
    async fn upsert_record(ctx: &AppContext, record: &ModelInput) -> Result<String> {
        // id_dosen is required (not Option in ModelInput)
        let id_dosen = record.id_dosen;

        // nama_dosen is required (not Option in ModelInput)
        let nama_dosen = &record.nama_dosen;

        // Start transaction
        let txn = ctx.db.begin().await?;
        let sync_time = Local::now().naive_local();

        // Check if record exists
        let existing = dosen::Entity::find()
            .filter(dosen::Column::DeletedAt.is_null())
            .filter(dosen::Column::IdDosen.eq(id_dosen))
            .one(&txn)
            .await?;

        let action = if let Some(existing_record) = existing {
            // Update existing record
            let mut active: dosen::ActiveModel = existing_record.into_active_model();

            active.nama_dosen = Set(Some(nama_dosen.clone()));
            active.nidn = Set(record.nidn.clone());
            active.nip = Set(record.nip.clone());
            active.jenis_kelamin = Set(record.jenis_kelamin.clone());
            active.id_agama = Set(record.id_agama);
            active.nama_agama = Set(record.nama_agama.clone());
            active.tanggal_lahir = Set(record.tanggal_lahir);
            active.id_status_aktif = Set(record.id_status_aktif.clone());
            active.nama_status_aktif = Set(record.nama_status_aktif.clone());
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

            let new_record = dosen::ActiveModel {
                id: Set(pk_id),
                id_dosen: Set(Some(id_dosen)),
                nama_dosen: Set(Some(nama_dosen.clone())),
                nidn: Set(record.nidn.clone()),
                nip: Set(record.nip.clone()),
                jenis_kelamin: Set(record.jenis_kelamin.clone()),
                id_agama: Set(record.id_agama),
                nama_agama: Set(record.nama_agama.clone()),
                tanggal_lahir: Set(record.tanggal_lahir),
                id_status_aktif: Set(record.id_status_aktif.clone()),
                nama_status_aktif: Set(record.nama_status_aktif.clone()),
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

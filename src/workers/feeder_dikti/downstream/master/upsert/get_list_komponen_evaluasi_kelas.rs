use chrono::Local;
use loco_rs::prelude::*;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, TransactionTrait};
use serde::{Deserialize, Serialize};

use crate::models::feeder::master::komponen_evaluasi_kelas::{
    _entities::komponen_evaluasi_kelas, feeder_model::ModelInput,
};

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
        "GetListKomponenEvaluasiKelas".to_string()
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
    /// This function processes a batch of komponen evaluasi kelas records:
    /// 1. Converts each `ModelInput` to database entity
    /// 2. Checks if record exists by `id_komponen_evaluasi`
    /// 3. Updates existing record or inserts new one
    /// 4. Tracks sync timestamp
    ///
    /// # Returns
    /// * `Result<()>` - Ok if all records processed successfully, Err otherwise
    async fn perform(&self, args: WorkerArgs) -> Result<()> {
        println!("=================GetListKomponenEvaluasiKelas=======================");
        println!("📦 Processing batch of {} records", args.records.len());

        let mut success_count = 0;
        let mut error_count = 0;

        for (index, record) in args.records.iter().enumerate() {
            match Self::upsert_record(&self.ctx, record).await {
                Ok(action) => {
                    success_count += 1;
                    println!(
                        "  ✅ Record {}/{}: {} - id_komponen_evaluasi={:?}",
                        index + 1,
                        args.records.len(),
                        action,
                        record.id_komponen_evaluasi
                    );
                }
                Err(e) => {
                    error_count += 1;
                    eprintln!(
                        "  ❌ Record {}/{}: Failed - id_komponen_evaluasi={:?}, error: {}",
                        index + 1,
                        args.records.len(),
                        record.id_komponen_evaluasi,
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
    /// Upsert a single komponen evaluasi kelas record into the database.
    ///
    /// This function performs an "upsert" operation:
    /// - If a record with the same `id_komponen_evaluasi` exists, it updates it
    /// - If no record exists, it inserts a new one
    ///
    /// # Parameters
    /// * `ctx` - Application context for database access
    /// * `record` - The feeder model data to upsert
    ///
    /// # Returns
    /// * `Result<String>` - "INSERTED" or "UPDATED" on success, error otherwise
    async fn upsert_record(ctx: &AppContext, record: &ModelInput) -> Result<String> {
        // Validate required fields
        let id_komponen_evaluasi = record
            .id_komponen_evaluasi
            .ok_or_else(|| Error::Message("Missing id_komponen_evaluasi".to_string()))?;

        let id_kelas_kuliah = record
            .id_kelas_kuliah
            .ok_or_else(|| Error::Message("Missing id_kelas_kuliah".to_string()))?;

        let id_jenis_evaluasi = record
            .id_jenis_evaluasi
            .ok_or_else(|| Error::Message("Missing id_jenis_evaluasi".to_string()))?;

        let nomor_urut = record
            .nomor_urut
            .ok_or_else(|| Error::Message("Missing nomor_urut".to_string()))?;

        let bobot_evaluasi = record
            .bobot_evaluasi
            .ok_or_else(|| Error::Message("Missing bobot_evaluasi".to_string()))?;

        let last_update = record
            .last_update
            .ok_or_else(|| Error::Message("Missing last_update".to_string()))?;

        let tgl_create = record
            .tgl_create
            .ok_or_else(|| Error::Message("Missing tgl_create".to_string()))?;

        // Start transaction
        let txn = ctx.db.begin().await?;
        let sync_time = Local::now().naive_local();

        // Check if record exists
        let existing = komponen_evaluasi_kelas::Entity::find()
            .filter(komponen_evaluasi_kelas::Column::DeletedAt.is_null())
            .filter(komponen_evaluasi_kelas::Column::IdKomponenEvaluasi.eq(id_komponen_evaluasi))
            .filter(komponen_evaluasi_kelas::Column::IdKelasKuliah.eq(id_kelas_kuliah))
            .filter(komponen_evaluasi_kelas::Column::IdJenisEvaluasi.eq(id_jenis_evaluasi))
            .one(&txn)
            .await?;

        let action = if let Some(existing_record) = existing {
            // Update existing record
            let mut active: komponen_evaluasi_kelas::ActiveModel =
                existing_record.into_active_model();

            active.id_kelas_kuliah = Set(id_kelas_kuliah);
            active.id_jenis_evaluasi = Set(id_jenis_evaluasi);
            active.nama = Set(record.nama.clone());
            active.nama_inggris = Set(record.nama_inggris.clone());
            active.nomor_urut = Set(nomor_urut);
            active.bobot_evaluasi = Set(bobot_evaluasi.to_string());
            active.last_update = Set(last_update);
            active.tgl_create = Set(tgl_create);
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

            let new_record = komponen_evaluasi_kelas::ActiveModel {
                id: Set(pk_id),
                id_komponen_evaluasi: Set(id_komponen_evaluasi),
                id_kelas_kuliah: Set(id_kelas_kuliah),
                id_jenis_evaluasi: Set(id_jenis_evaluasi),
                nama: Set(record.nama.clone()),
                nama_inggris: Set(record.nama_inggris.clone()),
                nomor_urut: Set(nomor_urut),
                bobot_evaluasi: Set(bobot_evaluasi.to_string()),
                last_update: Set(last_update),
                tgl_create: Set(tgl_create),
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

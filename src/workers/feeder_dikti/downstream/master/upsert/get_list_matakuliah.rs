use chrono::Local;
use loco_rs::prelude::*;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, TransactionTrait};
use serde::{Deserialize, Serialize};

use crate::models::feeder::master::matakuliah::{_entities::matakuliah, feeder_model::ModelInput};

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
        "GetListMatakuliah".to_string()
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
    /// This function processes a batch of matakuliah records:
    /// 1. Converts each `ModelInput` to database entity
    /// 2. Checks if record exists by `id`
    /// 3. Updates existing record or inserts new one
    /// 4. Tracks sync timestamp
    ///
    /// # Returns
    /// * `Result<()>` - Ok if all records processed successfully, Err otherwise
    async fn perform(&self, args: WorkerArgs) -> Result<()> {
        println!("=================GetListMatakuliah=======================");
        println!("📦 Processing batch of {} records", args.records.len());

        let mut success_count = 0;
        let mut error_count = 0;

        for (index, record) in args.records.iter().enumerate() {
            match Self::upsert_record(&self.ctx, record).await {
                Ok(action) => {
                    success_count += 1;
                    println!(
                        "  ✅ Record {}/{}: {} - id_matkul={:?}, kode_mata_kuliah={:?}",
                        index + 1,
                        args.records.len(),
                        action,
                        record.id_matkul,
                        record.kode_mata_kuliah
                    );
                }
                Err(e) => {
                    error_count += 1;
                    eprintln!(
                        "  ❌ Record {}/{}: Failed - id_matkul={:?}, kode_mata_kuliah={:?}, error: {}",
                        index + 1,
                        args.records.len(),
                        record.id_matkul,
                        record.kode_mata_kuliah,
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
    /// Upsert a single matakuliah record into the database.
    ///
    /// This function performs an "upsert" operation:
    /// - If a record with the same `id` exists, it updates it
    /// - If no record exists, it inserts a new one
    ///
    /// # Parameters
    /// * `ctx` - Application context for database access
    /// * `record` - The feeder model data to upsert
    ///
    /// # Returns
    /// * `Result<String>` - "INSERTED" or "UPDATED" on success, error otherwise
    async fn upsert_record(ctx: &AppContext, record: &ModelInput) -> Result<String> {
        // Use id_matkul as the primary key since id is not provided in the API response
        let id = record
            .id_matkul
            .ok_or_else(|| Error::Message("id_matkul is required".to_string()))?;

        // kode_mata_kuliah is required (not Option in ModelInput)
        let kode_mata_kuliah = &record.kode_mata_kuliah;

        // Start transaction
        let txn = ctx.db.begin().await?;
        let sync_time = Local::now().naive_local();

        // Check if record exists
        let existing = matakuliah::Entity::find()
            .filter(matakuliah::Column::DeletedAt.is_null())
            .filter(matakuliah::Column::IdMatkul.eq(id))
            .one(&txn)
            .await?;

        let action = if let Some(existing_record) = existing {
            // Update existing record
            let mut active: matakuliah::ActiveModel = existing_record.into_active_model();

            active.id_matkul = Set(record.id_matkul);
            active.kode_mata_kuliah = Set(kode_mata_kuliah.clone());
            active.nama_mata_kuliah = Set(record.nama_mata_kuliah.clone());
            active.id_prodi = Set(record.id_prodi);
            active.nama_program_studi = Set(record.nama_program_studi.clone());
            active.id_jenis_mata_kuliah = Set(record.id_jenis_mata_kuliah.clone());
            active.id_kelompok_mata_kuliah = Set(record.id_kelompok_mata_kuliah.clone());
            active.sks_mata_kuliah = Set(record.sks_mata_kuliah);
            active.sks_tatap_muka = Set(record.sks_tatap_muka);
            active.sks_praktek = Set(record.sks_praktek);
            active.sks_praktek_lapangan = Set(record.sks_praktek_lapangan);
            active.sks_simulasi = Set(record.sks_simulasi);
            active.metode_kuliah = Set(record.metode_kuliah.clone());
            active.ada_sap = Set(record.ada_sap);
            active.ada_silabus = Set(record.ada_silabus);
            active.ada_bahan_ajar = Set(record.ada_bahan_ajar);
            active.ada_acara_praktek = Set(record.ada_acara_praktek);
            active.ada_diktat = Set(record.ada_diktat);
            active.tanggal_mulai_efektif = Set(record.tanggal_mulai_efektif);
            active.tanggal_selesai_efektif = Set(record.tanggal_selesai_efektif);
            active.sync_at = Set(Some(sync_time));
            active.updated_at = Set(Some(sync_time));

            active.update(&txn).await?;
            "UPDATED"
        } else {
            // Insert new record
            // Insert new record
            let uuid_v7 = uuid7::uuid7();
            let uuid_string = uuid_v7.to_string();
            let pk_id = uuid::Uuid::parse_str(&uuid_string)
                .map_err(|e| Error::Message(format!("Invalid UUID: {}", e)))?;

            let new_record = matakuliah::ActiveModel {
                id: Set(pk_id),
                id_matkul: Set(record.id_matkul),
                kode_mata_kuliah: Set(kode_mata_kuliah.clone()),
                nama_mata_kuliah: Set(record.nama_mata_kuliah.clone()),
                id_prodi: Set(record.id_prodi),
                nama_program_studi: Set(record.nama_program_studi.clone()),
                id_jenis_mata_kuliah: Set(record.id_jenis_mata_kuliah.clone()),
                id_kelompok_mata_kuliah: Set(record.id_kelompok_mata_kuliah.clone()),
                sks_mata_kuliah: Set(record.sks_mata_kuliah),
                sks_tatap_muka: Set(record.sks_tatap_muka),
                sks_praktek: Set(record.sks_praktek),
                sks_praktek_lapangan: Set(record.sks_praktek_lapangan),
                sks_simulasi: Set(record.sks_simulasi),
                metode_kuliah: Set(record.metode_kuliah.clone()),
                ada_sap: Set(record.ada_sap),
                ada_silabus: Set(record.ada_silabus),
                ada_bahan_ajar: Set(record.ada_bahan_ajar),
                ada_acara_praktek: Set(record.ada_acara_praktek),
                ada_diktat: Set(record.ada_diktat),
                tanggal_mulai_efektif: Set(record.tanggal_mulai_efektif),
                tanggal_selesai_efektif: Set(record.tanggal_selesai_efektif),
                sync_at: Set(Some(sync_time)),
                created_at: Set(Some(sync_time)),
                updated_at: Set(Some(sync_time)),
                created_by: Set(record.created_by),
                updated_by: Set(record.updated_by),
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

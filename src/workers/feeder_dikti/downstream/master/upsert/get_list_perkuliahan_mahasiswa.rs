use chrono::Local;
use loco_rs::prelude::*;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, TransactionTrait};
use serde::{Deserialize, Serialize};

use crate::models::feeder::master::perkuliahan_mahasiswa::{
    _entities::perkuliahan_mahasiswa, feeder_model::ModelInput,
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
        "GetListPerkuliahanMahasiswa".to_string()
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
    /// This function processes a batch of perkuliahan_mahasiswa records:
    /// 1. Converts each `ModelInput` to database entity
    /// 2. Checks if record exists by `id_registrasi_mahasiswa` and `id_semester`
    /// 3. Updates existing record or inserts new one
    /// 4. Tracks sync timestamp
    ///
    /// # Returns
    /// * `Result<()>` - Ok if all records processed successfully, Err otherwise
    async fn perform(&self, args: WorkerArgs) -> Result<()> {
        println!("=================GetListPerkuliahanMahasiswa=======================");
        println!("📦 Processing batch of {} records", args.records.len());

        let mut success_count = 0;
        let mut error_count = 0;

        for (index, record) in args.records.iter().enumerate() {
            match Self::upsert_record(&self.ctx, record).await {
                Ok(action) => {
                    success_count += 1;
                    println!(
                        "  ✅ Record {}/{}: {} - id_registrasi_mahasiswa={:?}, nim={:?}, id_semester={:?}",
                        index + 1,
                        args.records.len(),
                        action,
                        record.id_registrasi_mahasiswa,
                        record.nim,
                        record.id_semester
                    );
                }
                Err(e) => {
                    error_count += 1;
                    eprintln!(
                        "  ❌ Record {}/{}: Failed - id_registrasi_mahasiswa={:?}, nim={:?}, id_semester={:?}, error: {}",
                        index + 1,
                        args.records.len(),
                        record.id_registrasi_mahasiswa,
                        record.nim,
                        record.id_semester,
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
    /// Upsert a single perkuliahan_mahasiswa record into the database.
    ///
    /// This function performs an "upsert" operation:
    /// - If a record with the same `id_registrasi_mahasiswa` and `id_semester` exists, it updates it
    /// - If no record exists, it inserts a new one
    ///
    /// # Parameters
    /// * `ctx` - Application context for database access
    /// * `record` - The feeder model data to upsert
    ///
    /// # Returns
    /// * `Result<String>` - "INSERTED" or "UPDATED" on success, error otherwise
    async fn upsert_record(ctx: &AppContext, record: &ModelInput) -> Result<String> {
        // Start transaction
        let txn = ctx.db.begin().await?;
        let sync_time = Local::now().naive_local();

        // Build query to find existing record
        // We need both id_registrasi_mahasiswa and id_semester to uniquely identify a record
        let mut query = perkuliahan_mahasiswa::Entity::find()
            .filter(perkuliahan_mahasiswa::Column::DeletedAt.is_null());

        // Filter by id_registrasi_mahasiswa if present
        if let Some(id_reg) = record.id_registrasi_mahasiswa {
            query = query.filter(perkuliahan_mahasiswa::Column::IdRegistrasiMahasiswa.eq(id_reg));
        }

        // Filter by id_semester if present
        if let Some(ref id_sem) = record.id_semester {
            query = query.filter(perkuliahan_mahasiswa::Column::IdSemester.eq(id_sem.clone()));
        }

        // Filter by prodi if present
        if let Some(ref id_cp) = record.id_prodi {
            query = query.filter(perkuliahan_mahasiswa::Column::IdProdi.eq(id_cp.clone()));
        }

        // Filter by id_periode_masuk if present
        if let Some(ref id_pm) = record.id_periode_masuk {
            query = query.filter(perkuliahan_mahasiswa::Column::IdPeriodeMasuk.eq(id_pm.clone()));
        }

        let existing = query.one(&txn).await?;

        let action = if let Some(existing_record) = existing {
            // Update existing record
            let mut active: perkuliahan_mahasiswa::ActiveModel =
                existing_record.into_active_model();

            active.id_registrasi_mahasiswa = Set(record.id_registrasi_mahasiswa);
            active.nim = Set(record.nim.clone());
            active.nama_mahasiswa = Set(record.nama_mahasiswa.clone());
            active.id_prodi = Set(record.id_prodi);
            active.nama_program_studi = Set(record.nama_program_studi.clone());
            active.angkatan = Set(record.angkatan.clone());
            active.id_periode_masuk = Set(record.id_periode_masuk.clone());
            active.id_semester = Set(record.id_semester.clone());
            active.nama_semester = Set(record.nama_semester.clone());
            active.id_status_mahasiswa = Set(record.id_status_mahasiswa.clone());
            active.nama_status_mahasiswa = Set(record.nama_status_mahasiswa.clone());
            active.ips = Set(record.ips);
            active.ipk = Set(record.ipk);
            active.sks_semester = Set(record.sks_semester);
            active.sks_total = Set(record.sks_total);
            active.biaya_kuliah_smt = Set(record.biaya_kuliah_smt);
            active.id_pembiayaan = Set(record.id_pembiayaan.clone());
            active.status_sync = Set(record.status_sync.clone());
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

            let new_record = perkuliahan_mahasiswa::ActiveModel {
                id: Set(pk_id),
                id_registrasi_mahasiswa: Set(record.id_registrasi_mahasiswa),
                nim: Set(record.nim.clone()),
                nama_mahasiswa: Set(record.nama_mahasiswa.clone()),
                id_prodi: Set(record.id_prodi),
                nama_program_studi: Set(record.nama_program_studi.clone()),
                angkatan: Set(record.angkatan.clone()),
                id_periode_masuk: Set(record.id_periode_masuk.clone()),
                id_semester: Set(record.id_semester.clone()),
                nama_semester: Set(record.nama_semester.clone()),
                id_status_mahasiswa: Set(record.id_status_mahasiswa.clone()),
                nama_status_mahasiswa: Set(record.nama_status_mahasiswa.clone()),
                ips: Set(record.ips),
                ipk: Set(record.ipk),
                sks_semester: Set(record.sks_semester),
                sks_total: Set(record.sks_total),
                biaya_kuliah_smt: Set(record.biaya_kuliah_smt),
                id_pembiayaan: Set(record.id_pembiayaan.clone()),
                status_sync: Set(record.status_sync.clone()),
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

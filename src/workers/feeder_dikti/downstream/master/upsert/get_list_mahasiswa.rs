use chrono::Local;
use loco_rs::prelude::*;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, TransactionTrait};
use serde::{Deserialize, Serialize};

use crate::models::feeder::master::mahasiswa::{_entities::mahasiswa, feeder_model::ModelInput};

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
        "GetListMahasiswa".to_string()
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
    /// This function processes a batch of mahasiswa records:
    /// 1. Converts each `ModelInput` to database entity
    /// 2. Checks if record exists by `id_mahasiswa`
    /// 3. Updates existing record or inserts new one
    /// 4. Tracks sync timestamp
    ///
    /// # Returns
    /// * `Result<()>` - Ok if all records processed successfully, Err otherwise
    async fn perform(&self, args: WorkerArgs) -> Result<()> {
        println!("=================GetListMahasiswa=======================");
        println!("📦 Processing batch of {} records", args.records.len());

        let mut success_count = 0;
        let mut error_count = 0;

        for (index, record) in args.records.iter().enumerate() {
            match Self::upsert_record(&self.ctx, record).await {
                Ok(action) => {
                    success_count += 1;
                    println!(
                        "  ✅ Record {}/{}: {} - id_mahasiswa={}, nim={:?}",
                        index + 1,
                        args.records.len(),
                        action,
                        record.id_mahasiswa,
                        record.nim
                    );
                }
                Err(e) => {
                    error_count += 1;
                    eprintln!(
                        "  ❌ Record {}/{}: Failed - id_mahasiswa={}, nim={:?}, error: {}",
                        index + 1,
                        args.records.len(),
                        record.id_mahasiswa,
                        record.nim,
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
    /// Upsert a single mahasiswa record into the database.
    ///
    /// This function performs an "upsert" operation:
    /// - If a record with the same `id_mahasiswa` exists, it updates it
    /// - If no record exists, it inserts a new one
    ///
    /// # Parameters
    /// * `ctx` - Application context for database access
    /// * `record` - The feeder model data to upsert
    ///
    /// # Returns
    /// * `Result<String>` - "INSERTED" or "UPDATED" on success, error otherwise
    async fn upsert_record(ctx: &AppContext, record: &ModelInput) -> Result<String> {
        // id_mahasiswa is required (not Option in ModelInput)
        let id_mahasiswa = record.id_mahasiswa;

        // nama_mahasiswa is required (not Option in ModelInput)
        let nama_mahasiswa = &record.nama_mahasiswa;

        // Start transaction
        let txn = ctx.db.begin().await?;
        let sync_time = Local::now().naive_local();

        // Check if record exists
        let existing = mahasiswa::Entity::find()
            .filter(mahasiswa::Column::DeletedAt.is_null())
            .filter(mahasiswa::Column::IdMahasiswa.eq(id_mahasiswa))
            .one(&txn)
            .await?;

        let action = if let Some(existing_record) = existing {
            // Update existing record
            let mut active: mahasiswa::ActiveModel = existing_record.into_active_model();

            active.nama_mahasiswa = Set(nama_mahasiswa.clone());
            active.jenis_kelamin = Set(record.jenis_kelamin.clone());
            active.tanggal_lahir = Set(record.tanggal_lahir);
            active.id_perguruan_tinggi = Set(record.id_perguruan_tinggi);
            active.nipd = Set(record.nipd.clone());
            active.ipk = Set(record.ipk);
            active.total_sks = Set(record.total_sks);
            active.id_sms = Set(record.id_sms);
            active.id_agama = Set(record.id_agama);
            active.nama_agama = Set(record.nama_agama.clone());
            active.id_prodi = Set(record.id_prodi.clone());
            active.nama_program_studi = Set(record.nama_program_studi.clone());
            active.id_status_mahasiswa = Set(record.id_status_mahasiswa);
            active.nama_status_mahasiswa = Set(record.nama_status_mahasiswa.clone());
            active.nim = Set(record.nim.clone());
            active.id_periode = Set(record.id_periode.clone());
            active.nama_periode_masuk = Set(record.nama_periode_masuk.clone());
            active.id_registrasi_mahasiswa = Set(record.id_registrasi_mahasiswa);
            active.id_periode_keluar = Set(record.id_periode_keluar.clone());
            active.tanggal_keluar = Set(record.tanggal_keluar);
            active.last_update = Set(record.last_update);
            active.tgl_create = Set(record.tgl_create);
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

            let new_record = mahasiswa::ActiveModel {
                id: Set(pk_id),
                id_mahasiswa: Set(id_mahasiswa),
                nama_mahasiswa: Set(nama_mahasiswa.clone()),
                jenis_kelamin: Set(record.jenis_kelamin.clone()),
                tanggal_lahir: Set(record.tanggal_lahir),
                id_perguruan_tinggi: Set(record.id_perguruan_tinggi),
                nipd: Set(record.nipd.clone()),
                ipk: Set(record.ipk),
                total_sks: Set(record.total_sks),
                id_sms: Set(record.id_sms),
                id_agama: Set(record.id_agama),
                nama_agama: Set(record.nama_agama.clone()),
                id_prodi: Set(record.id_prodi.clone()),
                nama_program_studi: Set(record.nama_program_studi.clone()),
                id_status_mahasiswa: Set(record.id_status_mahasiswa),
                nama_status_mahasiswa: Set(record.nama_status_mahasiswa.clone()),
                nim: Set(record.nim.clone()),
                id_periode: Set(record.id_periode.clone()),
                nama_periode_masuk: Set(record.nama_periode_masuk.clone()),
                id_registrasi_mahasiswa: Set(record.id_registrasi_mahasiswa),
                id_periode_keluar: Set(record.id_periode_keluar.clone()),
                tanggal_keluar: Set(record.tanggal_keluar),
                last_update: Set(record.last_update),
                tgl_create: Set(record.tgl_create),
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

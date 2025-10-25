use chrono::Local;
use loco_rs::prelude::*;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, TransactionTrait};
use serde::{Deserialize, Serialize};

use crate::models::feeder::master::kartu_rencana_studi_mahasiswa::{
    _entities::kartu_rencana_studi_mahasiswa, feeder_model::ModelInput,
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
        "GetKRSMahasiswa".to_string()
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
    /// This function processes a batch of KRS mahasiswa records:
    /// 1. Converts each `ModelInput` to database entity
    /// 2. Checks if record exists by unique combination of fields
    /// 3. Updates existing record or inserts new one
    /// 4. Tracks sync timestamp
    ///
    /// # Returns
    /// * `Result<()>` - Ok if all records processed successfully, Err otherwise
    async fn perform(&self, args: WorkerArgs) -> Result<()> {
        println!("=================GetKRSMahasiswa=======================");
        println!("📦 Processing batch of {} records", args.records.len());

        let mut success_count = 0;
        let mut error_count = 0;

        for (index, record) in args.records.iter().enumerate() {
            match Self::upsert_record(&self.ctx, record).await {
                Ok(action) => {
                    success_count += 1;
                    println!(
                        "  ✅ Record {}/{}: {} - nim={:?}, id_kelas={:?}, id_periode={:?}",
                        index + 1,
                        args.records.len(),
                        action,
                        record.nim,
                        record.id_kelas,
                        record.id_periode
                    );
                }
                Err(e) => {
                    error_count += 1;
                    eprintln!(
                        "  ❌ Record {}/{}: Failed - nim={:?}, id_kelas={:?}, id_periode={:?}, error: {}",
                        index + 1,
                        args.records.len(),
                        record.nim,
                        record.id_kelas,
                        record.id_periode,
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
    /// Upsert a single KRS mahasiswa record into the database.
    ///
    /// This function performs an "upsert" operation:
    /// - If a record with the same unique combination exists, it updates it
    /// - If no record exists, it inserts a new one
    ///
    /// The unique combination is: id_registrasi_mahasiswa + id_periode + id_kelas
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

        // Build query to find existing record by unique combination
        let mut query = kartu_rencana_studi_mahasiswa::Entity::find()
            .filter(kartu_rencana_studi_mahasiswa::Column::DeletedAt.is_null());

        // Filter by id_registrasi_mahasiswa if present
        if let Some(id_reg) = record.id_registrasi_mahasiswa {
            query = query
                .filter(kartu_rencana_studi_mahasiswa::Column::IdRegistrasiMahasiswa.eq(id_reg));
        }

        // Filter by id_periode if present
        if let Some(ref id_periode) = record.id_periode {
            query = query
                .filter(kartu_rencana_studi_mahasiswa::Column::IdPeriode.eq(id_periode.clone()));
        }

        // Filter by id_kelas if present
        if let Some(id_kelas) = record.id_kelas {
            query = query.filter(kartu_rencana_studi_mahasiswa::Column::IdKelas.eq(id_kelas));
        }

        // Filter by id_kelas if present
        if let Some(id_prodi) = record.id_prodi {
            query = query.filter(kartu_rencana_studi_mahasiswa::Column::IdProdi.eq(id_prodi));
        }

        // Filter by id_kelas if present
        if let Some(id_matkul) = record.id_matkul {
            query = query.filter(kartu_rencana_studi_mahasiswa::Column::IdMatkul.eq(id_matkul));
        }

        let existing = query.one(&txn).await?;

        let action = if let Some(existing_record) = existing {
            // Update existing record
            let mut active: kartu_rencana_studi_mahasiswa::ActiveModel =
                existing_record.into_active_model();

            active.id_registrasi_mahasiswa = Set(record.id_registrasi_mahasiswa);
            active.id_periode = Set(record.id_periode.clone());
            active.id_prodi = Set(record.id_prodi);
            active.nama_program_studi = Set(record.nama_program_studi.clone());
            active.id_matkul = Set(record.id_matkul);
            active.kode_mata_kuliah = Set(record.kode_mata_kuliah.clone());
            active.nama_mata_kuliah = Set(record.nama_mata_kuliah.clone());
            active.id_kelas = Set(record.id_kelas);
            active.nama_kelas_kuliah = Set(record.nama_kelas_kuliah.clone());
            active.sks_mata_kuliah = Set(record.sks_mata_kuliah);
            active.nim = Set(record.nim.clone());
            active.nama_mahasiswa = Set(record.nama_mahasiswa.clone());
            active.angkatan = Set(record.angkatan.clone());
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

            let new_record = kartu_rencana_studi_mahasiswa::ActiveModel {
                id: Set(pk_id),
                id_registrasi_mahasiswa: Set(record.id_registrasi_mahasiswa),
                id_periode: Set(record.id_periode.clone()),
                id_prodi: Set(record.id_prodi),
                nama_program_studi: Set(record.nama_program_studi.clone()),
                id_matkul: Set(record.id_matkul),
                kode_mata_kuliah: Set(record.kode_mata_kuliah.clone()),
                nama_mata_kuliah: Set(record.nama_mata_kuliah.clone()),
                id_kelas: Set(record.id_kelas),
                nama_kelas_kuliah: Set(record.nama_kelas_kuliah.clone()),
                sks_mata_kuliah: Set(record.sks_mata_kuliah),
                nim: Set(record.nim.clone()),
                nama_mahasiswa: Set(record.nama_mahasiswa.clone()),
                angkatan: Set(record.angkatan.clone()),
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

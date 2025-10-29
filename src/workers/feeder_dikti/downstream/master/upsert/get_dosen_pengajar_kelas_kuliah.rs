use chrono::Local;
use loco_rs::prelude::*;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, TransactionTrait};
use serde::{Deserialize, Serialize};

use crate::models::feeder::master::dosen_pengajar_kelas_kuliah::{
    _entities::dosen_pengajar_kelas_kuliah, feeder_model::ModelInput,
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
        "GetDosenPengajarKelasKuliah".to_string()
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
    /// This function processes a batch of dosen pengajar kelas kuliah records:
    /// 1. Converts each `ModelInput` to database entity
    /// 2. Checks if record exists by composite key (id_aktivitas_mengajar, id_registrasi_dosen, id_kelas_kuliah)
    /// 3. Updates existing record or inserts new one
    /// 4. Tracks sync timestamp
    ///
    /// # Returns
    /// * `Result<()>` - Ok if all records processed successfully, Err otherwise
    async fn perform(&self, args: WorkerArgs) -> Result<()> {
        println!("=================GetDosenPengajarKelasKuliah=======================");
        println!("📦 Processing batch of {} records", args.records.len());

        let mut success_count = 0;
        let mut error_count = 0;

        for (index, record) in args.records.iter().enumerate() {
            match Self::upsert_record(&self.ctx, record).await {
                Ok(action) => {
                    success_count += 1;
                    println!(
                        "  ✅ Record {}/{}: {} - id_aktivitas_mengajar={:?}, id_registrasi_dosen={:?}, id_kelas_kuliah={:?}",
                        index + 1,
                        args.records.len(),
                        action,
                        record.id_aktivitas_mengajar,
                        record.id_registrasi_dosen,
                        record.id_kelas_kuliah
                    );
                }
                Err(e) => {
                    error_count += 1;
                    eprintln!(
                        "  ❌ Record {}/{}: Failed - id_aktivitas_mengajar={:?}, id_registrasi_dosen={:?}, id_kelas_kuliah={:?}, error: {}",
                        index + 1,
                        args.records.len(),
                        record.id_aktivitas_mengajar,
                        record.id_registrasi_dosen,
                        record.id_kelas_kuliah,
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
    /// Upsert a single dosen pengajar kelas kuliah record into the database.
    ///
    /// This function performs an "upsert" operation:
    /// - If a record with the same composite key exists, it updates it
    /// - If no record exists, it inserts a new one
    ///
    /// # Parameters
    /// * `ctx` - Application context for database access
    /// * `record` - The feeder model data to upsert
    ///
    /// # Returns
    /// * `Result<String>` - "INSERTED" or "UPDATED" on success, error otherwise
    async fn upsert_record(ctx: &AppContext, record: &ModelInput) -> Result<String> {
        // Validate that required fields exist for composite key
        let id_aktivitas_mengajar = record.id_aktivitas_mengajar;
        let id_registrasi_dosen = record.id_registrasi_dosen;
        let id_kelas_kuliah = record.id_kelas_kuliah;

        // Start transaction
        let txn = ctx.db.begin().await?;
        let sync_time = Local::now().naive_local();

        // Check if record exists using composite key
        let existing = dosen_pengajar_kelas_kuliah::Entity::find()
            .filter(dosen_pengajar_kelas_kuliah::Column::DeletedAt.is_null())
            .filter(
                dosen_pengajar_kelas_kuliah::Column::IdAktivitasMengajar.eq(id_aktivitas_mengajar),
            )
            .filter(dosen_pengajar_kelas_kuliah::Column::IdRegistrasiDosen.eq(id_registrasi_dosen))
            .filter(dosen_pengajar_kelas_kuliah::Column::IdKelasKuliah.eq(id_kelas_kuliah))
            .one(&txn)
            .await?;

        let action = if let Some(existing_record) = existing {
            // Update existing record
            let mut active: dosen_pengajar_kelas_kuliah::ActiveModel =
                existing_record.into_active_model();

            active.id_dosen = Set(record.id_dosen);
            active.nidn = Set(record.nidn.clone());
            active.nuptk = Set(record.nuptk.clone());
            active.nama_dosen = Set(record.nama_dosen.clone());
            active.nama_kelas_kuliah = Set(record.nama_kelas_kuliah.clone());
            active.id_substansi = Set(record.id_substansi);
            active.sks_substansi_total = Set(record.sks_substansi_total);
            active.rencana_minggu_pertemuan = Set(record.rencana_minggu_pertemuan);
            active.realisasi_minggu_pertemuan = Set(record.realisasi_minggu_pertemuan);
            active.id_jenis_evaluasi = Set(record.id_jenis_evaluasi.clone());
            active.nama_jenis_evaluasi = Set(record.nama_jenis_evaluasi.clone());
            active.id_prodi = Set(record.id_prodi);
            active.id_semester = Set(record.id_semester.clone());
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

            let new_record = dosen_pengajar_kelas_kuliah::ActiveModel {
                id: Set(pk_id),
                id_aktivitas_mengajar: Set(id_aktivitas_mengajar),
                id_registrasi_dosen: Set(id_registrasi_dosen),
                id_dosen: Set(record.id_dosen),
                nidn: Set(record.nidn.clone()),
                nuptk: Set(record.nuptk.clone()),
                nama_dosen: Set(record.nama_dosen.clone()),
                id_kelas_kuliah: Set(id_kelas_kuliah),
                nama_kelas_kuliah: Set(record.nama_kelas_kuliah.clone()),
                id_substansi: Set(record.id_substansi),
                sks_substansi_total: Set(record.sks_substansi_total),
                rencana_minggu_pertemuan: Set(record.rencana_minggu_pertemuan),
                realisasi_minggu_pertemuan: Set(record.realisasi_minggu_pertemuan),
                id_jenis_evaluasi: Set(record.id_jenis_evaluasi.clone()),
                nama_jenis_evaluasi: Set(record.nama_jenis_evaluasi.clone()),
                id_prodi: Set(record.id_prodi),
                id_semester: Set(record.id_semester.clone()),
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

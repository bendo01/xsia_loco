use chrono::Local;
use loco_rs::prelude::*;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, TransactionTrait};
use serde::{Deserialize, Serialize};

use crate::models::feeder::master::aktifitas_mengajar_dosen::{
    _entities::aktifitas_mengajar_dosen, feeder_model::ModelInput,
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
        "GetAktifitasMengajarDosen".to_string()
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
    /// This function processes a batch of aktifitas mengajar dosen records:
    /// 1. Converts each `ModelInput` to database entity
    /// 2. Checks if record exists by composite key (id_registrasi_dosen, id_periode, id_prodi, id_matkul, id_kelas)
    /// 3. Updates existing record or inserts new one
    /// 4. Tracks sync timestamp
    ///
    /// # Returns
    /// * `Result<()>` - Ok if all records processed successfully, Err otherwise
    async fn perform(&self, args: WorkerArgs) -> Result<()> {
        println!("=================GetAktifitasMengajarDosen=======================");
        println!("📦 Processing batch of {} records", args.records.len());

        let mut success_count = 0;
        let mut error_count = 0;

        for (index, record) in args.records.iter().enumerate() {
            match Self::upsert_record(&self.ctx, record).await {
                Ok(action) => {
                    success_count += 1;
                    println!(
                        "  ✅ Record {}/{}: {} - id_registrasi_dosen={:?}, id_periode={:?}, id_prodi={:?}, id_matkul={:?}, id_kelas={:?}",
                        index + 1,
                        args.records.len(),
                        action,
                        record.id_registrasi_dosen,
                        record.id_periode,
                        record.id_prodi,
                        record.id_matkul,
                        record.id_kelas
                    );
                }
                Err(e) => {
                    error_count += 1;
                    eprintln!(
                        "  ❌ Record {}/{}: Failed - id_registrasi_dosen={:?}, id_periode={:?}, id_prodi={:?}, id_matkul={:?}, id_kelas={:?}, error: {}",
                        index + 1,
                        args.records.len(),
                        record.id_registrasi_dosen,
                        record.id_periode,
                        record.id_prodi,
                        record.id_matkul,
                        record.id_kelas,
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
    /// Upsert a single aktifitas mengajar dosen record into the database.
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
        let id_registrasi_dosen = record.id_registrasi_dosen;
        let id_periode = record.id_periode.clone();
        let id_prodi = record.id_prodi;
        let id_matkul = record.id_matkul;
        let id_kelas = record.id_kelas;

        // Clone values for later use in insert
        let id_periode_clone = id_periode.clone();

        // Start transaction
        let txn = ctx.db.begin().await?;
        let sync_time = Local::now().naive_local();

        // Check if record exists using composite key
        let existing = aktifitas_mengajar_dosen::Entity::find()
            .filter(aktifitas_mengajar_dosen::Column::DeletedAt.is_null())
            .filter(aktifitas_mengajar_dosen::Column::IdRegistrasiDosen.eq(id_registrasi_dosen))
            .filter(aktifitas_mengajar_dosen::Column::IdPeriode.eq(id_periode))
            .filter(aktifitas_mengajar_dosen::Column::IdProdi.eq(id_prodi))
            .filter(aktifitas_mengajar_dosen::Column::IdMatkul.eq(id_matkul))
            .filter(aktifitas_mengajar_dosen::Column::IdKelas.eq(id_kelas))
            .one(&txn)
            .await?;

        let action = if let Some(existing_record) = existing {
            // Update existing record
            let mut active: aktifitas_mengajar_dosen::ActiveModel = existing_record.into_active_model();

            active.id_dosen = Set(Some(record.id_dosen));
            active.nama_dosen = Set(Some(record.nama_dosen.clone()));
            active.nama_periode = Set(Some(record.nama_periode.clone()));
            active.nama_program_studi = Set(Some(record.nama_program_studi.clone()));
            active.nama_mata_kuliah = Set(Some(record.nama_mata_kuliah.clone()));
            active.nama_kelas_kuliah = Set(Some(record.nama_kelas_kuliah.clone()));
            active.rencana_minggu_pertemuan = Set(record.rencana_minggu_pertemuan);
            active.realisasi_minggu_pertemuan = Set(record.realisasi_minggu_pertemuan);
            active.sync_at = Set(Some(sync_time));
            active.updated_at = Set(Some(sync_time));
            active.rencana_minggu_pertemuan = Set(record.rencana_minggu_pertemuan);
            active.realisasi_minggu_pertemuan = Set(record.realisasi_minggu_pertemuan);
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

            let new_record = aktifitas_mengajar_dosen::ActiveModel {
                id: Set(pk_id),
                id_registrasi_dosen: Set(Some(id_registrasi_dosen)),
                id_dosen: Set(Some(record.id_dosen)),
                nama_dosen: Set(Some(record.nama_dosen.clone())),
                id_periode: Set(Some(id_periode_clone)),
                nama_periode: Set(Some(record.nama_periode.clone())),
                id_prodi: Set(Some(id_prodi)),
                nama_program_studi: Set(Some(record.nama_program_studi.clone())),
                id_matkul: Set(Some(id_matkul)),
                nama_mata_kuliah: Set(Some(record.nama_mata_kuliah.clone())),
                id_kelas: Set(Some(id_kelas)),
                nama_kelas_kuliah: Set(Some(record.nama_kelas_kuliah.clone())),
                rencana_minggu_pertemuan: Set(record.rencana_minggu_pertemuan),
                realisasi_minggu_pertemuan: Set(record.realisasi_minggu_pertemuan),
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
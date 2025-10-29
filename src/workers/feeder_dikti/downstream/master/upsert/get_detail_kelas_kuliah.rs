use chrono::Local;
use loco_rs::prelude::*;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, TransactionTrait};
use serde::{Deserialize, Serialize};

use crate::models::feeder::master::kelas_kuliah::{
    _entities::kelas_kuliah, feeder_model::KelasKuliah,
};

pub struct Worker {
    pub ctx: AppContext,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct WorkerArgs {
    pub records: Vec<KelasKuliah>,
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
        "GetDetailKelasKuliah".to_string()
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
    /// This function processes a batch of kelas kuliah records:
    /// 1. Converts each `KelasKuliah` to database entity
    /// 2. Checks if record exists by `id_kelas_kuliah`
    /// 3. Updates existing record or inserts new one
    /// 4. Tracks sync timestamp
    ///
    /// # Returns
    /// * `Result<()>` - Ok if all records processed successfully, Err otherwise
    async fn perform(&self, args: WorkerArgs) -> Result<()> {
        println!("=================GetDetailKelasKuliah=======================");
        println!("📦 Processing batch of {} records", args.records.len());

        let mut success_count = 0;
        let mut error_count = 0;

        for (index, record) in args.records.iter().enumerate() {
            match Self::upsert_record(&self.ctx, record).await {
                Ok(action) => {
                    success_count += 1;
                    println!(
                        "  ✅ Record {}/{}: {} - id_kelas_kuliah={:?}, nama_kelas_kuliah={:?}",
                        index + 1,
                        args.records.len(),
                        action,
                        record.id_kelas_kuliah,
                        record.nama_kelas_kuliah
                    );
                }
                Err(e) => {
                    error_count += 1;
                    eprintln!(
                        "  ❌ Record {}/{}: Failed - id_kelas_kuliah={:?}, nama_kelas_kuliah={:?}, error: {}",
                        index + 1,
                        args.records.len(),
                        record.id_kelas_kuliah,
                        record.nama_kelas_kuliah,
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
    /// Upsert a single kelas kuliah record into the database.
    ///
    /// This function performs an "upsert" operation:
    /// - If a record with the same `id_kelas_kuliah` exists, it updates it
    /// - If no record exists, it inserts a new one
    ///
    /// # Parameters
    /// * `ctx` - Application context for database access
    /// * `record` - The feeder model data to upsert
    ///
    /// # Returns
    /// * `Result<String>` - "INSERTED" or "UPDATED" on success, error otherwise
    async fn upsert_record(ctx: &AppContext, record: &KelasKuliah) -> Result<String> {
        // Start transaction
        let txn = ctx.db.begin().await?;
        let sync_time = Local::now().naive_local();

        // Check if record exists
        let existing = kelas_kuliah::Entity::find()
            .filter(kelas_kuliah::Column::DeletedAt.is_null())
            .filter(kelas_kuliah::Column::IdKelasKuliah.eq(record.id_kelas_kuliah.clone()))
            .filter(kelas_kuliah::Column::IdProdi.eq(record.id_prodi.clone()))
            .filter(kelas_kuliah::Column::IdSemester.eq(record.id_semester.clone()))
            .filter(kelas_kuliah::Column::IdMatkul.eq(record.id_matkul.clone()))
            .one(&txn)
            .await?;

        let action = if let Some(existing_record) = existing {
            // Update existing record
            let mut active: kelas_kuliah::ActiveModel = existing_record.into_active_model();

            active.id_prodi = Set(record.id_prodi);
            active.nama_program_studi = Set(record.nama_program_studi.clone());
            active.id_semester = Set(record.id_semester.clone());
            active.nama_semester = Set(record.nama_semester.clone());
            active.id_matkul = Set(record.id_matkul);
            active.kode_mata_kuliah = Set(record.kode_mata_kuliah.clone());
            active.nama_mata_kuliah = Set(record.nama_mata_kuliah.clone());
            active.sks_mk = Set(record.sks_mk);
            active.sks_tm = Set(record.sks_tm);
            active.sks_prak = Set(record.sks_prak);
            active.sks_prak_lap = Set(record.sks_prak_lap);
            active.sks_sim = Set(record.sks_sim);
            active.nama_kelas_kuliah = Set(record.nama_kelas_kuliah.clone());
            active.bahasan = Set(record.bahasan.clone());
            active.tanggal_mulai_efektif = Set(record.tanggal_mulai_efektif);
            active.tanggal_akhir_efektif = Set(record.tanggal_akhir_efektif);
            active.kapasitas = Set(record.kapasitas);
            active.tanggal_tutup_daftar = Set(record.tanggal_tutup_daftar);
            active.prodi_penyelenggara = Set(record.prodi_penyelenggara.clone());
            active.perguruan_tinggi_penyelenggara =
                Set(record.perguruan_tinggi_penyelenggara.clone());
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

            let new_record = kelas_kuliah::ActiveModel {
                id: Set(pk_id),
                id_kelas_kuliah: Set(record.id_kelas_kuliah),
                id_prodi: Set(record.id_prodi),
                nama_program_studi: Set(record.nama_program_studi.clone()),
                id_semester: Set(record.id_semester.clone()),
                nama_semester: Set(record.nama_semester.clone()),
                id_matkul: Set(record.id_matkul),
                kode_mata_kuliah: Set(record.kode_mata_kuliah.clone()),
                nama_mata_kuliah: Set(record.nama_mata_kuliah.clone()),
                sks_mk: Set(record.sks_mk),
                sks_tm: Set(record.sks_tm),
                sks_prak: Set(record.sks_prak),
                sks_prak_lap: Set(record.sks_prak_lap),
                sks_sim: Set(record.sks_sim),
                nama_kelas_kuliah: Set(record.nama_kelas_kuliah.clone()),
                bahasan: Set(record.bahasan.clone()),
                tanggal_mulai_efektif: Set(record.tanggal_mulai_efektif),
                tanggal_akhir_efektif: Set(record.tanggal_akhir_efektif),
                kapasitas: Set(record.kapasitas),
                tanggal_tutup_daftar: Set(record.tanggal_tutup_daftar),
                prodi_penyelenggara: Set(record.prodi_penyelenggara.clone()),
                perguruan_tinggi_penyelenggara: Set(record.perguruan_tinggi_penyelenggara.clone()),
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

use chrono::Local;
use loco_rs::prelude::*;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, TransactionTrait};
use serde::{Deserialize, Serialize};

use crate::models::feeder::master::nilai_perkuliahan_kelas::{_entities::nilai_perkuliahan_kelas, feeder_model::ModelInput};

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
        "GetListNilaiPerkuliahanKelas".to_string()
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
    /// This function processes a batch of nilai_perkuliahan_kelas records:
    /// 1. Converts each `ModelInput` to database entity
    /// 2. Checks if record exists by `id_kelas_kuliah`
    /// 3. Updates existing record or inserts new one
    /// 4. Tracks sync timestamp
    ///
    /// # Returns
    /// * `Result<()>` - Ok if all records processed successfully, Err otherwise
    async fn perform(&self, args: WorkerArgs) -> Result<()> {
        println!("=================GetListNilaiPerkuliahanKelas=======================");
        println!("📦 Processing batch of {} records", args.records.len());

        let mut success_count = 0;
        let mut error_count = 0;

        for (index, record) in args.records.iter().enumerate() {
            match Self::upsert_record(&self.ctx, record).await {
                Ok(action) => {
                    success_count += 1;
                    println!(
                        "  ✅ Record {}/{}: {} - id_kelas_kuliah={}, nama_kelas_kuliah={:?}",
                        index + 1,
                        args.records.len(),
                        action,
                        record.id_kelas_kuliah,
                        record.nama_kelas_kuliah.as_deref().unwrap_or("N/A")
                    );
                }
                Err(e) => {
                    error_count += 1;
                    eprintln!(
                        "  ❌ Record {}/{}: Failed - id_kelas_kuliah={}, nama_kelas_kuliah={:?}, error: {}",
                        index + 1,
                        args.records.len(),
                        record.id_kelas_kuliah,
                        record.nama_kelas_kuliah.as_deref().unwrap_or("N/A"),
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
    /// Upsert a single nilai_perkuliahan_kelas record into the database.
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
    async fn upsert_record(ctx: &AppContext, record: &ModelInput) -> Result<String> {
        // id_kelas_kuliah is required (not Option in ModelInput)
        let id_kelas_kuliah = record.id_kelas_kuliah;

        // Start transaction
        let txn = ctx.db.begin().await?;
        let sync_time = Local::now().naive_local();

        // Check if record exists
        let existing = nilai_perkuliahan_kelas::Entity::find()
            .filter(nilai_perkuliahan_kelas::Column::DeletedAt.is_null())
            .filter(nilai_perkuliahan_kelas::Column::IdKelasKuliah.eq(id_kelas_kuliah))
            .one(&txn)
            .await?;

        let action = if let Some(existing_record) = existing {
            // Update existing record
            let mut active: nilai_perkuliahan_kelas::ActiveModel = existing_record.into_active_model();

            active.id_matkul = Set(record.id_matkul);
            active.kode_mata_kuliah = Set(record.kode_mata_kuliah.clone());
            active.nama_mata_kuliah = Set(record.nama_mata_kuliah.clone());
            active.nama_kelas_kuliah = Set(record.nama_kelas_kuliah.clone().unwrap_or_default());
            active.sks_mata_kuliah = Set(record.sks_mata_kuliah);
            active.jumlah_mahasiswa_krs = Set(record.jumlah_mahasiswa_krs);
            active.jumlah_mahasiswa_dapat_nilai = Set(record.jumlah_mahasiswa_dapat_nilai);
            active.sks_tm = Set(record.sks_tm);
            active.sks_prak = Set(record.sks_prak);
            active.sks_prak_lap = Set(record.sks_prak_lap);
            active.sks_sim = Set(record.sks_sim);
            active.bahasan_case = Set(record.bahasan_case.clone());
            active.a_selenggara_pditt = Set(record.a_selenggara_pditt);
            active.a_pengguna_pditt = Set(record.a_pengguna_pditt);
            active.kuota_pditt = Set(record.kuota_pditt);
            active.tgl_mulai_koas = Set(record.tgl_mulai_koas);
            active.tgl_selesai_koas = Set(record.tgl_selesai_koas);
            active.id_mou = Set(record.id_mou);
            active.id_kls_pditt = Set(record.id_kls_pditt);
            active.id_sms = Set(record.id_sms);
            active.id_smt = Set(record.id_smt.clone());
            active.tgl_create = Set(record.tgl_create);
            active.lingkup_kelas = Set(record.lingkup_kelas);
            active.mode_kuliah = Set(record.mode_kuliah.clone());
            active.nm_smt = Set(record.nm_smt.clone());
            active.nama_prodi = Set(record.nama_prodi.clone());
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

            let new_record = nilai_perkuliahan_kelas::ActiveModel {
                id: Set(pk_id),
                id_kelas_kuliah: Set(id_kelas_kuliah),
                id_matkul: Set(record.id_matkul),
                kode_mata_kuliah: Set(record.kode_mata_kuliah.clone()),
                nama_mata_kuliah: Set(record.nama_mata_kuliah.clone()),
                nama_kelas_kuliah: Set(record.nama_kelas_kuliah.clone()),
                sks_mata_kuliah: Set(record.sks_mata_kuliah),
                jumlah_mahasiswa_krs: Set(record.jumlah_mahasiswa_krs),
                jumlah_mahasiswa_dapat_nilai: Set(record.jumlah_mahasiswa_dapat_nilai),
                sks_tm: Set(record.sks_tm),
                sks_prak: Set(record.sks_prak),
                sks_prak_lap: Set(record.sks_prak_lap),
                sks_sim: Set(record.sks_sim),
                bahasan_case: Set(record.bahasan_case.clone()),
                a_selenggara_pditt: Set(record.a_selenggara_pditt),
                a_pengguna_pditt: Set(record.a_pengguna_pditt),
                kuota_pditt: Set(record.kuota_pditt),
                tgl_mulai_koas: Set(record.tgl_mulai_koas),
                tgl_selesai_koas: Set(record.tgl_selesai_koas),
                id_mou: Set(record.id_mou),
                id_kls_pditt: Set(record.id_kls_pditt),
                id_sms: Set(record.id_sms),
                id_smt: Set(record.id_smt.clone()),
                tgl_create: Set(record.tgl_create),
                lingkup_kelas: Set(record.lingkup_kelas),
                mode_kuliah: Set(record.mode_kuliah.clone()),
                nm_smt: Set(record.nm_smt.clone()),
                nama_prodi: Set(record.nama_prodi.clone()),
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
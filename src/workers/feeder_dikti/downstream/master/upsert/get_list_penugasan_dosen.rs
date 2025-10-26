use chrono::Local;
use loco_rs::prelude::*;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, TransactionTrait};
use serde::{Deserialize, Serialize};

use crate::models::feeder::master::penugasan_dosen::{
    _entities::penugasan_dosen, feeder_model::ModelInput,
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
        "GetListPenugasanDosen".to_string()
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
    /// This function processes a batch of penugasan_dosen records:
    /// 1. Converts each `ModelInput` to database entity
    /// 2. Checks if record exists by `id_registrasi_dosen`
    /// 3. Updates existing record or inserts new one
    /// 4. Tracks sync timestamp
    ///
    /// # Returns
    /// * `Result<()>` - Ok if all records processed successfully, Err otherwise
    async fn perform(&self, args: WorkerArgs) -> Result<()> {
        println!("=================GetListPenugasanDosen=======================");
        println!("📦 Processing batch of {} records", args.records.len());

        let mut success_count = 0;
        let mut error_count = 0;

        for (index, record) in args.records.iter().enumerate() {
            match Self::upsert_record(&self.ctx, record).await {
                Ok(action) => {
                    success_count += 1;
                    println!(
                        "  ✅ Record {}/{}: {} - id_registrasi_dosen={}, nama_dosen={:?}",
                        index + 1,
                        args.records.len(),
                        action,
                        record.id_registrasi_dosen,
                        record.nama_dosen
                    );
                }
                Err(e) => {
                    error_count += 1;
                    eprintln!(
                        "  ❌ Record {}/{}: Failed - id_registrasi_dosen={}, nama_dosen={:?}, error: {}",
                        index + 1,
                        args.records.len(),
                        record.id_registrasi_dosen,
                        record.nama_dosen,
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
    /// Upsert a single penugasan_dosen record into the database.
    ///
    /// This function performs an "upsert" operation:
    /// - If a record with the same `id_registrasi_dosen` exists, it updates it
    /// - If no record exists, it inserts a new one
    ///
    /// # Parameters
    /// * `ctx` - Application context for database access
    /// * `record` - The feeder model data to upsert
    ///
    /// # Returns
    /// * `Result<String>` - "INSERTED" or "UPDATED" on success, error otherwise
    async fn upsert_record(ctx: &AppContext, record: &ModelInput) -> Result<String> {
        // id_registrasi_dosen is required (not Option in ModelInput)
        let id_registrasi_dosen = &record.id_registrasi_dosen;

        // Start transaction
        let txn = ctx.db.begin().await?;
        let sync_time = Local::now().naive_local();

        // Check if record exists
        let existing = penugasan_dosen::Entity::find()
            .filter(penugasan_dosen::Column::DeletedAt.is_null())
            .filter(penugasan_dosen::Column::IdRegistrasiDosen.eq(id_registrasi_dosen))
            .one(&txn)
            .await?;

        let action = if let Some(existing_record) = existing {
            // Update existing record
            let mut active: penugasan_dosen::ActiveModel = existing_record.into_active_model();

            active.jk = Set(record.jk.clone());
            active.id_dosen = Set(record.id_dosen.clone());
            active.nama_dosen = Set(record.nama_dosen.clone());
            active.nidn = Set(record.nidn.clone());
            active.nuptk = Set(record.nuptk.clone());
            active.id_tahun_ajaran = Set(record.id_tahun_ajaran.clone());
            active.nama_tahun_ajaran = Set(record.nama_tahun_ajaran.clone());
            active.id_perguruan_tinggi = Set(record.id_perguruan_tinggi.clone());
            active.nama_perguruan_tinggi = Set(record.nama_perguruan_tinggi.clone());
            active.id_prodi = Set(record.id_prodi.clone());
            active.nama_program_studi = Set(record.nama_program_studi.clone());
            active.nomor_surat_tugas = Set(record.nomor_surat_tugas.clone());
            active.tanggal_surat_tugas = Set(record.tanggal_surat_tugas);
            active.mulai_surat_tugas = Set(record.mulai_surat_tugas);
            active.tgl_create = Set(record.tgl_create);
            active.tgl_ptk_keluar = Set(record
                .tgl_ptk_keluar
                .map(|d| d.and_hms_opt(0, 0, 0).unwrap()));
            active.id_stat_pegawai = Set(record.id_stat_pegawai);
            active.id_jns_keluar = Set(record.id_jns_keluar.clone());
            active.id_ikatan_kerja = Set(record.id_ikatan_kerja.clone());
            active.a_sp_homebase = Set(record.a_sp_homebase.clone());
            active.sync_at = Set(Some(sync_time));
            active.updated_at = Set(sync_time);

            active.update(&txn).await?;
            "UPDATED"
        } else {
            // Insert new record
            let uuid_v7 = uuid7::uuid7();
            let uuid_string = uuid_v7.to_string();
            let pk_id = uuid::Uuid::parse_str(&uuid_string)
                .map_err(|e| Error::Message(format!("Invalid UUID: {}", e)))?;

            let new_record = penugasan_dosen::ActiveModel {
                id: Set(pk_id),
                id_registrasi_dosen: Set(Some(id_registrasi_dosen.clone())),
                jk: Set(record.jk.clone()),
                id_dosen: Set(record.id_dosen.clone()),
                nama_dosen: Set(record.nama_dosen.clone()),
                nidn: Set(record.nidn.clone()),
                nuptk: Set(record.nuptk.clone()),
                id_tahun_ajaran: Set(record.id_tahun_ajaran.clone()),
                nama_tahun_ajaran: Set(record.nama_tahun_ajaran.clone()),
                id_perguruan_tinggi: Set(record.id_perguruan_tinggi.clone()),
                nama_perguruan_tinggi: Set(record.nama_perguruan_tinggi.clone()),
                id_prodi: Set(record.id_prodi.clone()),
                nama_program_studi: Set(record.nama_program_studi.clone()),
                nomor_surat_tugas: Set(record.nomor_surat_tugas.clone()),
                tanggal_surat_tugas: Set(record.tanggal_surat_tugas),
                mulai_surat_tugas: Set(record.mulai_surat_tugas),
                tgl_create: Set(record.tgl_create),
                tgl_ptk_keluar: Set(record
                    .tgl_ptk_keluar
                    .map(|d| d.and_hms_opt(0, 0, 0).unwrap())),
                id_stat_pegawai: Set(record.id_stat_pegawai),
                id_jns_keluar: Set(record.id_jns_keluar.clone()),
                id_ikatan_kerja: Set(record.id_ikatan_kerja.clone()),
                a_sp_homebase: Set(record.a_sp_homebase.clone()),
                sync_at: Set(Some(sync_time)),
                created_at: Set(sync_time),
                updated_at: Set(sync_time),
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

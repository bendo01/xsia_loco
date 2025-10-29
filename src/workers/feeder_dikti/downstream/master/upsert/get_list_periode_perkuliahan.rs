use chrono::Local;
use loco_rs::prelude::*;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, TransactionTrait};
use serde::{Deserialize, Serialize};

use crate::models::feeder::master::periode_perkuliahan::{
    _entities::periode_perkuliahan, feeder_model::ModelInputListPeriodePerkuliahan as ModelInput,
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
        "GetListPeriodePerkuliahan".to_string()
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
    /// This function processes a batch of periode perkuliahan records:
    /// 1. Converts each `ModelInput` to database entity
    /// 2. Checks if record exists by unique constraint (id_prodi + id_semester)
    /// 3. Updates existing record or inserts new one
    /// 4. Tracks sync timestamp
    ///
    /// # Returns
    /// * `Result<()>` - Ok if all records processed successfully, Err otherwise
    async fn perform(&self, args: WorkerArgs) -> Result<()> {
        println!("=================GetListPeriodePerkuliahan=======================");
        println!("📦 Processing batch of {} records", args.records.len());

        let mut success_count = 0;
        let mut error_count = 0;

        for (index, record) in args.records.iter().enumerate() {
            match Self::upsert_record(&self.ctx, record).await {
                Ok(action) => {
                    success_count += 1;
                    println!(
                        "  ✅ Record {}/{}: {} - id_prodi={}, id_semester={}",
                        index + 1,
                        args.records.len(),
                        action,
                        record.id_prodi,
                        record.id_semester
                    );
                }
                Err(e) => {
                    error_count += 1;
                    eprintln!(
                        "  ❌ Record {}/{}: Failed - id_prodi={}, id_semester={}, error: {}",
                        index + 1,
                        args.records.len(),
                        record.id_prodi,
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
    /// Upsert a single periode perkuliahan record into the database.
    ///
    /// This function performs an "upsert" operation:
    /// - If a record with the same `id_prodi` and `id_semester` exists, it updates it
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

        // Check if record exists by unique constraint (id_prodi + id_semester)
        let existing = periode_perkuliahan::Entity::find()
            .filter(periode_perkuliahan::Column::DeletedAt.is_null())
            .filter(periode_perkuliahan::Column::IdProdi.eq(record.id_prodi))
            .filter(periode_perkuliahan::Column::IdSemester.eq(&record.id_semester))
            .one(&txn)
            .await?;

        let action = if let Some(existing_record) = existing {
            // Update existing record
            let mut active: periode_perkuliahan::ActiveModel = existing_record.into_active_model();

            active.nama_program_studi = Set(Some(record.nama_program_studi.clone()));
            active.nama_semester = Set(Some(record.nama_semester.clone()));
            active.jumlah_target_mahasiswa_baru = Set(record.jumlah_target_mahasiswa_baru);
            active.tanggal_awal_perkuliahan = Set(record.tanggal_awal_perkuliahan);
            active.tanggal_akhir_perkuliahan = Set(record.tanggal_akhir_perkuliahan);

            // Map GetListPeriodePerkuliahan fields to database columns
            active.jumlah_pendaftar_ikut_seleksi = Set(record.calon_ikut_seleksi);
            active.jumlah_pendaftar_lulus_seleksi = Set(record.calon_lulus_seleksi);
            active.jumlah_daftar_ulang = Set(record.daftar_sbg_mhs);
            active.jumlah_mengundurkan_diri = Set(record.pst_undur_diri);
            active.jumlah_minggu_pertemuan = Set(record.jml_mgu_kul);

            active.metode_kul = Set(record.metode_kul.clone());
            active.metode_kul_eks = Set(record.metode_kul_eks.clone());
            active.tgl_create = Set(record.tgl_create);
            active.last_update = Set(record.last_update);
            active.status_sync = Set(Some(record.status_sync.clone()));
            active.sync_at = Set(Some(sync_time));
            active.updated_at = Set(sync_time);

            active.update(&txn).await?;

            "UPDATED"
        } else {
            // Insert new record
            let uuid_v7 = uuid7::uuid7();
            let uuid_string = uuid_v7.to_string();
            let pk_id: Uuid = Uuid::parse_str(&uuid_string).expect("Invalid UUID string");

            let new_record = periode_perkuliahan::ActiveModel {
                id: Set(pk_id),
                id_prodi: Set(Some(record.id_prodi)),
                nama_program_studi: Set(Some(record.nama_program_studi.clone())),
                id_semester: Set(Some(record.id_semester.clone())),
                nama_semester: Set(Some(record.nama_semester.clone())),
                jumlah_target_mahasiswa_baru: Set(record.jumlah_target_mahasiswa_baru),
                tanggal_awal_perkuliahan: Set(record.tanggal_awal_perkuliahan),
                tanggal_akhir_perkuliahan: Set(record.tanggal_akhir_perkuliahan),

                // Map GetListPeriodePerkuliahan fields to database columns
                jumlah_pendaftar_ikut_seleksi: Set(record.calon_ikut_seleksi),
                jumlah_pendaftar_lulus_seleksi: Set(record.calon_lulus_seleksi),
                jumlah_daftar_ulang: Set(record.daftar_sbg_mhs),
                jumlah_mengundurkan_diri: Set(record.pst_undur_diri),
                jumlah_minggu_pertemuan: Set(record.jml_mgu_kul),

                metode_kul: Set(record.metode_kul.clone()),
                metode_kul_eks: Set(record.metode_kul_eks.clone()),
                tgl_create: Set(record.tgl_create),
                last_update: Set(record.last_update),
                status_sync: Set(Some(record.status_sync.clone())),
                sync_at: Set(Some(sync_time)),
                created_at: Set(sync_time),
                updated_at: Set(sync_time),
                ..Default::default()
            };

            new_record.insert(&txn).await?;

            "INSERTED"
        };

        // Commit transaction
        txn.commit().await?;

        Ok(action.to_string())
    }
}

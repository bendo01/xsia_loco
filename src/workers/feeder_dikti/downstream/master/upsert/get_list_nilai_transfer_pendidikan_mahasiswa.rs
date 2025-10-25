use chrono::Local;
use loco_rs::prelude::*;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, TransactionTrait};
use serde::{Deserialize, Serialize};

use crate::models::feeder::master::nilai_transfer_pendidikan_mahasiswa::{
    _entities::nilai_transfer_pendidikan_mahasiswa, feeder_model::ModelInput,
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
        "GetListNilaiTransferPendidikanMahasiswa".to_string()
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
    /// This function processes a batch of nilai_transfer_pendidikan_mahasiswa records:
    /// 1. Converts each `ModelInput` to database entity
    /// 2. Checks if record exists by `id_transfer`
    /// 3. Updates existing record or inserts new one
    /// 4. Tracks sync timestamp
    ///
    /// # Returns
    /// * `Result<()>` - Ok if all records processed successfully, Err otherwise
    async fn perform(&self, args: WorkerArgs) -> Result<()> {
        println!("=================GetListNilaiTransferPendidikanMahasiswa=======================");
        println!("📦 Processing batch of {} records", args.records.len());

        let mut success_count = 0;
        let mut error_count = 0;

        for (index, record) in args.records.iter().enumerate() {
            match Self::upsert_record(&self.ctx, record).await {
                Ok(action) => {
                    success_count += 1;
                    println!(
                        "  ✅ Record {}/{}: {} - id_transfer={}, nim={}, matkul={}",
                        index + 1,
                        args.records.len(),
                        action,
                        record.id_transfer,
                        record.nim,
                        record.kode_mata_kuliah_asal
                    );
                }
                Err(e) => {
                    error_count += 1;
                    eprintln!(
                        "  ❌ Record {}/{}: Failed - id_transfer={}, nim={}, matkul={}, error: {}",
                        index + 1,
                        args.records.len(),
                        record.id_transfer,
                        record.nim,
                        record.kode_mata_kuliah_asal,
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
    /// Upsert a single nilai_transfer_pendidikan_mahasiswa record into the database.
    ///
    /// This function performs an "upsert" operation:
    /// - If a record with the same `id_transfer` exists, it updates it
    /// - If no record exists, it inserts a new one
    ///
    /// # Parameters
    /// * `ctx` - Application context for database access
    /// * `record` - The feeder model data to upsert
    ///
    /// # Returns
    /// * `Result<String>` - "INSERTED" or "UPDATED" on success, error otherwise
    async fn upsert_record(ctx: &AppContext, record: &ModelInput) -> Result<String> {
        // id_transfer is the unique key for this record
        let id_transfer = record.id_transfer;

        // Start transaction
        let txn = ctx.db.begin().await?;
        let sync_time = Local::now().naive_local();

        // Check if record exists
        let existing = nilai_transfer_pendidikan_mahasiswa::Entity::find()
            .filter(nilai_transfer_pendidikan_mahasiswa::Column::DeletedAt.is_null())
            .filter(nilai_transfer_pendidikan_mahasiswa::Column::IdTransfer.eq(id_transfer))
            .filter(
                nilai_transfer_pendidikan_mahasiswa::Column::IdRegistrasiMahasiswa
                    .eq(id_registrasi_mahasiswa),
            )
            .filter(nilai_transfer_pendidikan_mahasiswa::Column::IdMatkul.eq(id_matkul))
            .filter(nilai_transfer_pendidikan_mahasiswa::Column::IdProdi.eq(id_prodi))
            .filter(
                nilai_transfer_pendidikan_mahasiswa::Column::IdPeriodeMasuk.eq(id_periode_masuk),
            )
            .one(&txn)
            .await?;

        let action = if let Some(existing_record) = existing {
            // Update existing record
            let mut active: nilai_transfer_pendidikan_mahasiswa::ActiveModel =
                existing_record.into_active_model();

            active.id_registrasi_mahasiswa = Set(record.id_registrasi_mahasiswa);
            active.id_matkul = Set(record.id_matkul);
            active.nim = Set(record.nim.clone());
            active.nama_mahasiswa = Set(record.nama_mahasiswa.clone());
            active.id_prodi = Set(record.id_prodi);
            active.nama_program_studi = Set(record.nama_program_studi.clone());
            active.id_periode_masuk = Set(record.id_periode_masuk.clone());
            active.id_semester = Set(record.id_semester.clone());
            active.nama_semester = Set(record.nama_semester.clone());
            active.kode_mata_kuliah_asal = Set(record.kode_mata_kuliah_asal.clone());
            active.nama_mata_kuliah_asal = Set(record.nama_mata_kuliah_asal.clone());
            active.sks_mata_kuliah_asal = Set(record.sks_mata_kuliah_asal);
            active.nilai_huruf_asal = Set(record.nilai_huruf_asal.clone());
            active.kode_matkul_diakui = Set(record.kode_matkul_diakui.clone());
            active.nama_mata_kuliah_diakui = Set(record.nama_mata_kuliah_diakui.clone());
            active.sks_mata_kuliah_diakui = Set(record.sks_mata_kuliah_diakui);
            active.nilai_huruf_diakui = Set(record.nilai_huruf_diakui.clone());
            active.nilai_angka_diakui = Set(record.nilai_angka_diakui);
            active.id_perguruan_tinggi = Set(record.id_perguruan_tinggi);
            active.id_aktivitas = Set(record.id_aktivitas);
            active.judul = Set(record.judul.clone());
            active.id_jenis_aktivitas = Set(record.id_jenis_aktivitas);
            active.nama_jenis_aktivitas = Set(record.nama_jenis_aktivitas.clone());
            active.status_sync = Set(record.status_sync.clone());
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

            let new_record = nilai_transfer_pendidikan_mahasiswa::ActiveModel {
                id: Set(pk_id),
                id_transfer: Set(id_transfer),
                id_registrasi_mahasiswa: Set(record.id_registrasi_mahasiswa),
                id_matkul: Set(record.id_matkul),
                nim: Set(record.nim.clone()),
                nama_mahasiswa: Set(record.nama_mahasiswa.clone()),
                id_prodi: Set(record.id_prodi),
                nama_program_studi: Set(record.nama_program_studi.clone()),
                id_periode_masuk: Set(record.id_periode_masuk.clone()),
                id_semester: Set(record.id_semester.clone()),
                nama_semester: Set(record.nama_semester.clone()),
                kode_mata_kuliah_asal: Set(record.kode_mata_kuliah_asal.clone()),
                nama_mata_kuliah_asal: Set(record.nama_mata_kuliah_asal.clone()),
                sks_mata_kuliah_asal: Set(record.sks_mata_kuliah_asal),
                nilai_huruf_asal: Set(record.nilai_huruf_asal.clone()),
                kode_matkul_diakui: Set(record.kode_matkul_diakui.clone()),
                nama_mata_kuliah_diakui: Set(record.nama_mata_kuliah_diakui.clone()),
                sks_mata_kuliah_diakui: Set(record.sks_mata_kuliah_diakui),
                nilai_huruf_diakui: Set(record.nilai_huruf_diakui.clone()),
                nilai_angka_diakui: Set(record.nilai_angka_diakui),
                id_perguruan_tinggi: Set(record.id_perguruan_tinggi),
                id_aktivitas: Set(record.id_aktivitas),
                judul: Set(record.judul.clone()),
                id_jenis_aktivitas: Set(record.id_jenis_aktivitas),
                nama_jenis_aktivitas: Set(record.nama_jenis_aktivitas.clone()),
                status_sync: Set(record.status_sync.clone()),
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

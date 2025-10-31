use chrono::Local;
use loco_rs::prelude::*;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, TransactionTrait};
use serde::{Deserialize, Serialize};

use crate::models::feeder::master::detail_nilai_perkuliahan_kelas::{
    _entities::detail_nilai_perkuliahan_kelas, feeder_model::ModelInput,
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
        "GetDetailNilaiPerkuliahanKelas".to_string()
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
    /// This function processes a batch of detail nilai perkuliahan kelas records:
    /// 1. Converts each `ModelInput` to database entity
    /// 2. Checks if record exists by composite key (id_registrasi_mahasiswa + id_kelas_kuliah)
    /// 3. Updates existing record or inserts new one
    /// 4. Tracks sync timestamp
    ///
    /// # Returns
    /// * `Result<()>` - Ok if all records processed successfully, Err otherwise
    async fn perform(&self, args: WorkerArgs) -> Result<()> {
        println!("=================GetDetailNilaiPerkuliahanKelas=======================");
        println!("📦 Processing batch of {} records", args.records.len());

        let mut success_count = 0;
        let mut error_count = 0;

        for (index, record) in args.records.iter().enumerate() {
            match Self::upsert_record(&self.ctx, record).await {
                Ok(action) => {
                    success_count += 1;
                    println!(
                        "  ✅ Record {}/{}: {} - id_registrasi_mahasiswa={:?}, nim={:?}, nama_mahasiswa={:?}",
                        index + 1,
                        args.records.len(),
                        action,
                        record.id_registrasi_mahasiswa,
                        record.nim,
                        record.nama_mahasiswa
                    );
                }
                Err(e) => {
                    error_count += 1;
                    eprintln!(
                        "  ❌ Record {}/{}: Failed - id_registrasi_mahasiswa={:?}, nim={:?}, error: {}",
                        index + 1,
                        args.records.len(),
                        record.id_registrasi_mahasiswa,
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
    /// Upsert a single detail nilai perkuliahan kelas record into the database.
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
        // Validate that required fields exist
        let id_registrasi_mahasiswa = record
            .id_registrasi_mahasiswa
            .ok_or_else(|| Error::Message("Missing id_registrasi_mahasiswa".to_string()))?;
        
        let id_kelas_kuliah = record
            .id_kelas_kuliah
            .ok_or_else(|| Error::Message("Missing id_kelas_kuliah".to_string()))?;

        // Start transaction
        let txn = ctx.db.begin().await?;
        let sync_time = Local::now().naive_local();

        // Check if record exists using composite key
        let existing = detail_nilai_perkuliahan_kelas::Entity::find()
            .filter(detail_nilai_perkuliahan_kelas::Column::DeletedAt.is_null())
            .filter(detail_nilai_perkuliahan_kelas::Column::IdRegistrasiMahasiswa.eq(id_registrasi_mahasiswa))
            .filter(detail_nilai_perkuliahan_kelas::Column::IdKelasKuliah.eq(id_kelas_kuliah))
            .one(&txn)
            .await?;

        let action = if let Some(existing_record) = existing {
            // Update existing record
            let mut active: detail_nilai_perkuliahan_kelas::ActiveModel = existing_record.into_active_model();

            active.id_prodi = Set(record.id_prodi);
            active.nama_program_studi = Set(record.nama_program_studi.clone());
            active.id_semester = Set(record.id_semester.clone());
            active.nama_semester = Set(record.nama_semester.clone());
            active.id_matkul = Set(record.id_matkul);
            active.kode_mata_kuliah = Set(record.kode_mata_kuliah.clone());
            active.nama_mata_kuliah = Set(record.nama_mata_kuliah.clone());
            active.sks_mata_kuliah = Set(record.sks_mata_kuliah);
            active.nama_kelas_kuliah = Set(record.nama_kelas_kuliah.clone());
            active.id_mahasiswa = Set(record.id_mahasiswa);
            active.nim = Set(record.nim.clone());
            active.nama_mahasiswa = Set(record.nama_mahasiswa.clone());
            active.jurusan = Set(record.jurusan.clone());
            active.angkatan = Set(record.angkatan.clone());
            active.nilai_angka = Set(record.nilai_angka);
            active.nilai_indeks = Set(record.nilai_indeks);
            active.nilai_huruf = Set(record.nilai_huruf.clone());
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

            let new_record = detail_nilai_perkuliahan_kelas::ActiveModel {
                id: Set(pk_id),
                id_prodi: Set(record.id_prodi),
                nama_program_studi: Set(record.nama_program_studi.clone()),
                id_semester: Set(record.id_semester.clone()),
                nama_semester: Set(record.nama_semester.clone()),
                id_matkul: Set(record.id_matkul),
                kode_mata_kuliah: Set(record.kode_mata_kuliah.clone()),
                nama_mata_kuliah: Set(record.nama_mata_kuliah.clone()),
                sks_mata_kuliah: Set(record.sks_mata_kuliah),
                id_kelas_kuliah: Set(Some(id_kelas_kuliah)),
                nama_kelas_kuliah: Set(record.nama_kelas_kuliah.clone()),
                id_registrasi_mahasiswa: Set(Some(id_registrasi_mahasiswa)),
                id_mahasiswa: Set(record.id_mahasiswa),
                nim: Set(record.nim.clone()),
                nama_mahasiswa: Set(record.nama_mahasiswa.clone()),
                jurusan: Set(record.jurusan.clone()),
                angkatan: Set(record.angkatan.clone()),
                nilai_angka: Set(record.nilai_angka),
                nilai_indeks: Set(record.nilai_indeks),
                nilai_huruf: Set(record.nilai_huruf.clone()),
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
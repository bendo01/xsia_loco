use chrono::Local;
use loco_rs::prelude::*;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, TransactionTrait};
use serde::{Deserialize, Serialize};

use crate::models::feeder::master::transkrip_mahasiswa::{
    _entities::transkrip_mahasiswa, feeder_model::ModelInput,
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
        "GetTranskripMahasiswa".to_string()
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
    /// This function processes a batch of transkrip mahasiswa records:
    /// 1. Converts each `ModelInput` to database entity
    /// 2. Checks if record exists by unique combination of fields
    /// 3. Updates existing record or inserts new one
    /// 4. Tracks sync timestamp
    ///
    /// # Returns
    /// * `Result<()>` - Ok if all records processed successfully, Err otherwise
    async fn perform(&self, args: WorkerArgs) -> Result<()> {
        println!("=================GetTranskripMahasiswa=======================");
        println!("📦 Processing batch of {} records", args.records.len());

        let mut success_count = 0;
        let mut error_count = 0;

        for (index, record) in args.records.iter().enumerate() {
            match Self::upsert_record(&self.ctx, record).await {
                Ok(action) => {
                    success_count += 1;
                    println!(
                        "  ✅ Record {}/{}: {} - id_reg={:?}, id_matkul={:?}, smt={:?}",
                        index + 1,
                        args.records.len(),
                        action,
                        record.id_registrasi_mahasiswa,
                        record.id_matkul,
                        record.smt_diambil
                    );
                }
                Err(e) => {
                    error_count += 1;
                    eprintln!(
                        "  ❌ Record {}/{}: Failed - id_reg={:?}, id_matkul={:?}, smt={:?}, error: {}",
                        index + 1,
                        args.records.len(),
                        record.id_registrasi_mahasiswa,
                        record.id_matkul,
                        record.smt_diambil,
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
    /// Upsert a single transkrip mahasiswa record into the database.
    ///
    /// This function performs an "upsert" operation:
    /// - If a record with the same unique combination exists, it updates it
    /// - If no record exists, it inserts a new one
    ///
    /// The unique combination can be:
    /// 1. id_nilai_transfer (for transfer credits)
    /// 2. id_konversi_aktivitas (for converted activities)
    /// 3. id_registrasi_mahasiswa + id_matkul + smt_diambil (for regular courses)
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
        let mut query = transkrip_mahasiswa::Entity::find()
            .filter(transkrip_mahasiswa::Column::DeletedAt.is_null());

        // Priority 1: Check by id_nilai_transfer (transfer credits)
        if let Some(ref id_transfer) = record.id_nilai_transfer {
            if !id_transfer.is_empty() {
                query = query
                    .filter(transkrip_mahasiswa::Column::IdNilaiTransfer.eq(id_transfer.clone()));
            }
        }
        // Priority 2: Check by id_konversi_aktivitas (converted activities)
        else if let Some(ref id_konversi) = record.id_konversi_aktivitas {
            if !id_konversi.is_empty() {
                query = query.filter(
                    transkrip_mahasiswa::Column::IdKonversiAktivitas.eq(id_konversi.clone()),
                );
            }
        }
        // Priority 3: Check by regular combination
        else {
            if let Some(id_reg) = record.id_registrasi_mahasiswa {
                query = query.filter(transkrip_mahasiswa::Column::IdRegistrasiMahasiswa.eq(id_reg));
            }

            if let Some(id_matkul) = record.id_matkul {
                query = query.filter(transkrip_mahasiswa::Column::IdMatkul.eq(id_matkul));
            }

            if let Some(ref smt) = record.smt_diambil {
                query = query.filter(transkrip_mahasiswa::Column::SmtDiambil.eq(smt.clone()));
            }

            if let Some(ref id_kelas_kuliah) = record.id_kelas_kuliah {
                query = query
                    .filter(transkrip_mahasiswa::Column::IdKelasKuliah.eq(id_kelas_kuliah.clone()));
            }
        }

        let existing = query.one(&txn).await?;

        let action = if let Some(existing_record) = existing {
            // Update existing record
            let mut active: transkrip_mahasiswa::ActiveModel = existing_record.into_active_model();

            active.id_registrasi_mahasiswa = Set(record.id_registrasi_mahasiswa);
            active.id_matkul = Set(record.id_matkul);
            active.id_kelas_kuliah = Set(record.id_kelas_kuliah);
            active.id_nilai_transfer = Set(record.id_nilai_transfer.clone());
            active.id_konversi_aktivitas = Set(record.id_konversi_aktivitas.clone());
            active.smt_diambil = Set(record.smt_diambil.clone());
            active.kode_mata_kuliah = Set(record.kode_mata_kuliah.clone());
            active.nama_mata_kuliah = Set(record.nama_mata_kuliah.clone());
            active.sks_mata_kuliah = Set(record.sks_mata_kuliah);
            active.nilai_angka = Set(record.nilai_angka);
            active.nilai_huruf = Set(record.nilai_huruf.clone());
            active.nilai_indeks = Set(record.nilai_indeks);
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

            let new_record = transkrip_mahasiswa::ActiveModel {
                id: Set(pk_id),
                id_registrasi_mahasiswa: Set(record.id_registrasi_mahasiswa),
                id_matkul: Set(record.id_matkul),
                id_kelas_kuliah: Set(record.id_kelas_kuliah),
                id_nilai_transfer: Set(record.id_nilai_transfer.clone()),
                id_konversi_aktivitas: Set(record.id_konversi_aktivitas.clone()),
                smt_diambil: Set(record.smt_diambil.clone()),
                kode_mata_kuliah: Set(record.kode_mata_kuliah.clone()),
                nama_mata_kuliah: Set(record.nama_mata_kuliah.clone()),
                sks_mata_kuliah: Set(record.sks_mata_kuliah),
                nilai_angka: Set(record.nilai_angka),
                nilai_huruf: Set(record.nilai_huruf.clone()),
                nilai_indeks: Set(record.nilai_indeks),
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

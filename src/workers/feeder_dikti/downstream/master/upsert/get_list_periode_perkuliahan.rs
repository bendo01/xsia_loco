use chrono::Local;
use loco_rs::prelude::*;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, TransactionTrait};
use serde::{Deserialize, Serialize};

use crate::models::feeder::master::periode_perkuliahan::{
    _entities::periode_perkuliahan, feeder_model::ModelInputListPeriodePerkuliahan,
};

pub struct Worker {
    pub ctx: AppContext,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct WorkerArgs {
    pub records: Vec<ModelInputListPeriodePerkuliahan>,
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
    /// 1. Converts each `ModelInputListPeriodePerkuliahan` to database entity
    /// 2. Checks if record exists by `id_prodi` and `id_semester`
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
                        "  ✅ Record {}/{}: {} - id_prodi={:?}, id_semester={:?}",
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
                        "  ❌ Record {}/{}: Failed - id_prodi={:?}, id_semester={:?}, error: {}",
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
    async fn upsert_record(ctx: &AppContext, record: &ModelInputListPeriodePerkuliahan) -> Result<String> {
        // Start transaction
        let txn = ctx.db.begin().await?;
        let sync_time = Local::now().naive_local();

        // Parse UUID for id_prodi
        let id_prodi_uuid = record.id_prodi.as_ref()
            .and_then(|id| Uuid::parse_str(id).ok());

        // Check if record exists
        let mut query = periode_perkuliahan::Entity::find()
            .filter(periode_perkuliahan::Column::DeletedAt.is_null());

        if let Some(uuid) = &id_prodi_uuid {
            query = query.filter(periode_perkuliahan::Column::IdProdi.eq(*uuid));
        }

        if let Some(id_semester) = &record.id_semester {
            query = query.filter(periode_perkuliahan::Column::IdSemester.eq(id_semester.clone()));
        }

        let existing = query.one(&txn).await?;

        // Parse date fields
        let parse_date = |date_str: &Option<String>| -> Result<Option<chrono::NaiveDate>> {
            if let Some(date_str) = date_str {
                // Parse DD-MM-YYYY format
                let parts: Vec<&str> = date_str.split('-').collect();
                if parts.len() == 3 {
                    let day = parts[0].parse::<i32>()?;
                    let month = parts[1].parse::<i32>()?;
                    let year = parts[2].parse::<i32>()?;
                    let naive_date = chrono::NaiveDate::from_ymd_opt(year, month as u32, day as u32)
                        .ok_or_else(|| Error::Message(format!("Invalid date format: {}", date_str)))?;
                    Ok(Some(naive_date))
                } else {
                    Ok(None)
                }
            } else {
                Ok(None)
            }
        };

        let tanggal_awal_perkuliahan = parse_date(&record.tanggal_awal_perkuliahan)?;
        let tanggal_akhir_perkuliahan = parse_date(&record.tanggal_akhir_perkuliahan)?;
        let tgl_create = parse_date(&record.tgl_create)?;
        let last_update = parse_date(&record.last_update)?;

        let action = if let Some(existing_record) = existing {
            // Update existing record
            let mut active: periode_perkuliahan::ActiveModel = existing_record.into_active_model();

            active.id_prodi = Set(id_prodi_uuid);
            active.nama_program_studi = Set(record.nama_program_studi.clone());
            active.id_semester = Set(record.id_semester.clone());
            active.nama_semester = Set(record.nama_semester.clone());
            active.jumlah_target_mahasiswa_baru = Set(record.jumlah_target_mahasiswa_baru.clone());
            active.tanggal_awal_perkuliahan = Set(tanggal_awal_perkuliahan);
            active.tanggal_akhir_perkuliahan = Set(tanggal_akhir_perkuliahan);
            // Map list fields to detail fields
            active.jumlah_pendaftar_ikut_seleksi = Set(record.calon_ikut_seleksi.clone());
            active.jumlah_pendaftar_lulus_seleksi = Set(record.calon_lulus_seleksi.clone());
            active.jumlah_daftar_ulang = Set(record.daftar_sbg_mhs.clone());
            active.jumlah_mengundurkan_diri = Set(record.pst_undur_diri.clone());
            active.jumlah_minggu_pertemuan = Set(record.jml_mgu_kul.clone());
            active.metode_kul = Set(record.metode_kul.clone());
            active.metode_kul_eks = Set(record.metode_kul_eks.clone());
            active.tgl_create = Set(tgl_create);
            active.last_update = Set(last_update);
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

            let new_record = periode_perkuliahan::ActiveModel {
                id: Set(pk_id),
                id_prodi: Set(id_prodi_uuid),
                nama_program_studi: Set(record.nama_program_studi.clone()),
                id_semester: Set(record.id_semester.clone()),
                nama_semester: Set(record.nama_semester.clone()),
                jumlah_target_mahasiswa_baru: Set(record.jumlah_target_mahasiswa_baru.clone()),
                tanggal_awal_perkuliahan: Set(tanggal_awal_perkuliahan),
                tanggal_akhir_perkuliahan: Set(tanggal_akhir_perkuliahan),
                // Map list fields to detail fields
                jumlah_pendaftar_ikut_seleksi: Set(record.calon_ikut_seleksi.clone()),
                jumlah_pendaftar_lulus_seleksi: Set(record.calon_lulus_seleksi.clone()),
                jumlah_daftar_ulang: Set(record.daftar_sbg_mhs.clone()),
                jumlah_mengundurkan_diri: Set(record.pst_undur_diri.clone()),
                jumlah_minggu_pertemuan: Set(record.jml_mgu_kul.clone()),
                metode_kul: Set(record.metode_kul.clone()),
                metode_kul_eks: Set(record.metode_kul_eks.clone()),
                tgl_create: Set(tgl_create),
                last_update: Set(last_update),
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
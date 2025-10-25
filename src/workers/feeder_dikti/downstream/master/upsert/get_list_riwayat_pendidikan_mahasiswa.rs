use chrono::Local;
use loco_rs::prelude::*;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, TransactionTrait};
use serde::{Deserialize, Serialize};

use crate::models::feeder::master::riwayat_pendidikan_mahasiswa::{
    _entities::riwayat_pendidikan_mahasiswa, feeder_model::ModelInput,
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
        "GetListRiwayatPendidikanMahasiswa".to_string()
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
    /// This function processes a batch of riwayat_pendidikan_mahasiswa records:
    /// 1. Converts each `ModelInput` to database entity
    /// 2. Checks if record exists by `id_registrasi_mahasiswa`
    /// 3. Updates existing record or inserts new one
    /// 4. Tracks sync timestamp
    ///
    /// # Returns
    /// * `Result<()>` - Ok if all records processed successfully, Err otherwise
    async fn perform(&self, args: WorkerArgs) -> Result<()> {
        println!("=================GetListRiwayatPendidikanMahasiswa=======================");
        println!("📦 Processing batch of {} records", args.records.len());

        let mut success_count = 0;
        let mut error_count = 0;

        for (index, record) in args.records.iter().enumerate() {
            match Self::upsert_record(&self.ctx, record).await {
                Ok(action) => {
                    success_count += 1;
                    println!(
                        "  ✅ Record {}/{}: {} - id_registrasi_mahasiswa={:?}, nim={:?}",
                        index + 1,
                        args.records.len(),
                        action,
                        record.id_registrasi_mahasiswa,
                        record.nim
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
    /// Upsert a single riwayat_pendidikan_mahasiswa record into the database.
    ///
    /// This function performs an "upsert" operation:
    /// - If a record with the same `id_registrasi_mahasiswa` exists, it updates it
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

        // Build query to find existing record by id_registrasi_mahasiswa
        let existing = riwayat_pendidikan_mahasiswa::Entity::find()
            .filter(riwayat_pendidikan_mahasiswa::Column::DeletedAt.is_null())
            .filter(
                riwayat_pendidikan_mahasiswa::Column::IdRegistrasiMahasiswa
                    .eq(record.id_registrasi_mahasiswa.clone()),
            )
            .filter(
                riwayat_pendidikan_mahasiswa::Column::IdPerguruanTinggi
                    .eq(record.id_perguruan_tinggi.clone()),
            )
            .filter(riwayat_pendidikan_mahasiswa::Column::IdProdi.eq(record.id_prodi.clone()))
            .one(&txn)
            .await?;

        let action = if let Some(existing_record) = existing {
            // Update existing record
            let mut active: riwayat_pendidikan_mahasiswa::ActiveModel =
                existing_record.into_active_model();

            active.id_registrasi_mahasiswa = Set(record.id_registrasi_mahasiswa);
            active.id_mahasiswa = Set(record.id_mahasiswa);
            active.id_perguruan_tinggi = Set(record.id_perguruan_tinggi);
            active.id_prodi = Set(record.id_prodi);
            active.nim = Set(record.nim.clone());
            active.nama_mahasiswa = Set(record.nama_mahasiswa.clone());
            active.nama_perguruan_tinggi = Set(record.nama_perguruan_tinggi.clone());
            active.nama_program_studi = Set(record.nama_program_studi.clone());
            active.nama_jenis_daftar = Set(record.nama_jenis_daftar.clone());
            active.keterangan_keluar = Set(record.keterangan_keluar.clone());
            active.nama_program_studi_asal = Set(record.nama_program_studi_asal.clone());
            active.nama_perguruan_tinggi_asal = Set(record.nama_perguruan_tinggi_asal.clone());
            active.nama_periode_masuk = Set(record.nama_periode_masuk.clone());
            active.nm_bidang_minat = Set(record.nm_bidang_minat.clone());
            active.nama_pembiayaan_awal = Set(record.nama_pembiayaan_awal.clone());
            active.nama_ibu_kandung = Set(record.nama_ibu_kandung.clone());
            active.status_sync = Set(record.status_sync.clone());
            active.id_jenis_daftar = Set(record.id_jenis_daftar);
            active.id_jalur_daftar = Set(record.id_jalur_daftar);
            active.id_periode_masuk = Set(record.id_periode_masuk.clone());
            active.id_jenis_keluar = Set(record.id_jenis_keluar);
            active.id_pembiayaan = Set(record.id_pembiayaan);
            active.id_periode_keluar = Set(record.id_periode_keluar.clone());
            active.id_perguruan_tinggi_asal = Set(record.id_perguruan_tinggi_asal);
            active.id_prodi_asal = Set(record.id_prodi_asal);
            active.jenis_kelamin = Set(record.jenis_kelamin.clone());
            active.sks_diakui = Set(record.sks_diakui);
            active.biaya_masuk = Set(record.biaya_masuk);
            active.id_bidang_minat = Set(record.id_bidang_minat.clone());
            active.tanggal_daftar = Set(record.tanggal_daftar);
            active.tanggal_keluar = Set(record.tanggal_keluar);
            active.last_update = Set(record.last_update);
            active.tgl_create = Set(record.tgl_create);
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

            let new_record = riwayat_pendidikan_mahasiswa::ActiveModel {
                id: Set(pk_id),
                id_registrasi_mahasiswa: Set(record.id_registrasi_mahasiswa),
                id_mahasiswa: Set(record.id_mahasiswa),
                id_perguruan_tinggi: Set(record.id_perguruan_tinggi),
                id_prodi: Set(record.id_prodi),
                nim: Set(record.nim.clone()),
                nama_mahasiswa: Set(record.nama_mahasiswa.clone()),
                nama_perguruan_tinggi: Set(record.nama_perguruan_tinggi.clone()),
                nama_program_studi: Set(record.nama_program_studi.clone()),
                nama_jenis_daftar: Set(record.nama_jenis_daftar.clone()),
                keterangan_keluar: Set(record.keterangan_keluar.clone()),
                nama_program_studi_asal: Set(record.nama_program_studi_asal.clone()),
                nama_perguruan_tinggi_asal: Set(record.nama_perguruan_tinggi_asal.clone()),
                nama_periode_masuk: Set(record.nama_periode_masuk.clone()),
                nm_bidang_minat: Set(record.nm_bidang_minat.clone()),
                nama_pembiayaan_awal: Set(record.nama_pembiayaan_awal.clone()),
                nama_ibu_kandung: Set(record.nama_ibu_kandung.clone()),
                status_sync: Set(record.status_sync.clone()),
                id_jenis_daftar: Set(record.id_jenis_daftar),
                id_jalur_daftar: Set(record.id_jalur_daftar),
                id_periode_masuk: Set(record.id_periode_masuk.clone()),
                id_jenis_keluar: Set(record.id_jenis_keluar),
                id_pembiayaan: Set(record.id_pembiayaan),
                id_periode_keluar: Set(record.id_periode_keluar.clone()),
                id_perguruan_tinggi_asal: Set(record.id_perguruan_tinggi_asal),
                id_prodi_asal: Set(record.id_prodi_asal),
                jenis_kelamin: Set(record.jenis_kelamin.clone()),
                sks_diakui: Set(record.sks_diakui),
                biaya_masuk: Set(record.biaya_masuk),
                id_bidang_minat: Set(record.id_bidang_minat.clone()),
                tanggal_daftar: Set(record.tanggal_daftar),
                tanggal_keluar: Set(record.tanggal_keluar),
                last_update: Set(record.last_update),
                tgl_create: Set(record.tgl_create),
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

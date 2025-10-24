use chrono::Local;
use loco_rs::prelude::*;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, TransactionTrait};
use serde::{Deserialize, Serialize};

use crate::models::feeder::master::biodata_mahasiswa::{
    _entities::biodata_mahasiswa, feeder_model::ModelInput,
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
        "GetBiodataMahasiswa".to_string()
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
    /// This function processes a batch of biodata mahasiswa records:
    /// 1. Converts each `ModelInput` to database entity
    /// 2. Checks if record exists by `id_mahasiswa`
    /// 3. Updates existing record or inserts new one
    /// 4. Tracks sync timestamp
    ///
    /// # Returns
    /// * `Result<()>` - Ok if all records processed successfully, Err otherwise
    async fn perform(&self, args: WorkerArgs) -> Result<()> {
        println!("=================GetBiodataMahasiswa=======================");
        println!("📦 Processing batch of {} records", args.records.len());

        let mut success_count = 0;
        let mut error_count = 0;

        for (index, record) in args.records.iter().enumerate() {
            match Self::upsert_record(&self.ctx, record).await {
                Ok(action) => {
                    success_count += 1;
                    println!(
                        "  ✅ Record {}/{}: {} - id_mahasiswa={:?}, nik={:?}",
                        index + 1,
                        args.records.len(),
                        action,
                        record.id_mahasiswa,
                        record.nik
                    );
                }
                Err(e) => {
                    error_count += 1;
                    eprintln!(
                        "  ❌ Record {}/{}: Failed - id_mahasiswa={:?}, nik={:?}, error: {}",
                        index + 1,
                        args.records.len(),
                        record.id_mahasiswa,
                        record.nik,
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
    /// Upsert a single biodata mahasiswa record into the database.
    ///
    /// This function performs an "upsert" operation:
    /// - If a record with the same `id_mahasiswa` exists, it updates it
    /// - If no record exists, it inserts a new one
    ///
    /// # Parameters
    /// * `ctx` - Application context for database access
    /// * `record` - The feeder model data to upsert
    ///
    /// # Returns
    /// * `Result<String>` - "INSERTED" or "UPDATED" on success, error otherwise
    async fn upsert_record(ctx: &AppContext, record: &ModelInput) -> Result<String> {
        // Validate that id_mahasiswa exists (it's the unique key)
        let id_mahasiswa = record
            .id_mahasiswa
            .ok_or_else(|| Error::Message("Missing id_mahasiswa".to_string()))?;

        // Start transaction
        let txn = ctx.db.begin().await?;
        let sync_time = Local::now().naive_local();

        // Check if record exists
        let existing = biodata_mahasiswa::Entity::find()
            .filter(biodata_mahasiswa::Column::DeletedAt.is_null())
            .filter(biodata_mahasiswa::Column::IdMahasiswa.eq(id_mahasiswa))
            .one(&txn)
            .await?;

        let action = if let Some(existing_record) = existing {
            // Update existing record
            let mut active: biodata_mahasiswa::ActiveModel = existing_record.into_active_model();

            active.nama_mahasiswa = Set(record.nama_mahasiswa.clone());
            active.jenis_kelamin = Set(record.jenis_kelamin.clone());
            active.tempat_lahir = Set(record.tempat_lahir.clone());
            active.tanggal_lahir = Set(record.tanggal_lahir);
            active.id_agama = Set(record.id_agama);
            active.nama_agama = Set(record.nama_agama.clone());
            active.nik = Set(record.nik.clone());
            active.nisn = Set(record.nisn.clone());
            active.npwp = Set(record.npwp.clone());
            active.id_negara = Set(record.id_negara.clone());
            active.kewarganegaraan = Set(record.kewarganegaraan.clone());
            active.jalan = Set(record.jalan.clone());
            active.dusun = Set(record.dusun.clone());
            active.rt = Set(record.rt);
            active.rw = Set(record.rw);
            active.kelurahan = Set(record.kelurahan.clone());
            active.kode_pos = Set(record.kode_pos.clone());
            active.id_wilayah = Set(record.id_wilayah.clone());
            active.nama_wilayah = Set(record.nama_wilayah.clone());
            active.id_jenis_tinggal = Set(record.id_jenis_tinggal.clone());
            active.nama_jenis_tinggal = Set(record.nama_jenis_tinggal.clone());
            active.id_alat_transportasi = Set(record.id_alat_transportasi.clone());
            active.nama_alat_transportasi = Set(record.nama_alat_transportasi.clone());
            active.telepon = Set(record.telepon.clone());
            active.handphone = Set(record.handphone.clone());
            active.email = Set(record.email.clone());
            active.penerima_kps = Set(record.penerima_kps);
            active.nomor_kps = Set(record.nomor_kps.clone());
            active.nik_ayah = Set(record.nik_ayah.clone());
            active.nama_ayah = Set(record.nama_ayah.clone());
            active.tanggal_lahir_ayah = Set(record.tanggal_lahir_ayah);
            active.id_pendidikan_ayah = Set(record.id_pendidikan_ayah);
            active.nama_pendidikan_ayah = Set(record.nama_pendidikan_ayah.clone());
            active.id_pekerjaan_ayah = Set(record.id_pekerjaan_ayah);
            active.nama_pekerjaan_ayah = Set(record.nama_pekerjaan_ayah.clone());
            active.id_penghasilan_ayah = Set(record.id_penghasilan_ayah);
            active.nama_penghasilan_ayah = Set(record.nama_penghasilan_ayah.clone());
            active.nik_ibu = Set(record.nik_ibu.clone());
            active.nama_ibu_kandung = Set(record.nama_ibu_kandung.clone());
            active.tanggal_lahir_ibu = Set(record.tanggal_lahir_ibu);
            active.id_pendidikan_ibu = Set(record.id_pendidikan_ibu);
            active.nama_pendidikan_ibu = Set(record.nama_pendidikan_ibu.clone());
            active.id_pekerjaan_ibu = Set(record.id_pekerjaan_ibu);
            active.nama_pekerjaan_ibu = Set(record.nama_pekerjaan_ibu.clone());
            active.id_penghasilan_ibu = Set(record.id_penghasilan_ibu);
            active.nama_penghasilan_ibu = Set(record.nama_penghasilan_ibu.clone());
            active.nama_wali = Set(record.nama_wali.clone());
            active.tanggal_lahir_wali = Set(record.tanggal_lahir_wali);
            active.id_pendidikan_wali = Set(record.id_pendidikan_wali);
            active.nama_pendidikan_wali = Set(record.nama_pendidikan_wali.clone());
            active.id_pekerjaan_wali = Set(record.id_pekerjaan_wali);
            active.nama_pekerjaan_wali = Set(record.nama_pekerjaan_wali.clone());
            active.id_penghasilan_wali = Set(record.id_penghasilan_wali);
            active.nama_penghasilan_wali = Set(record.nama_penghasilan_wali.clone());
            active.id_kebutuhan_khusus_mahasiswa = Set(record.id_kebutuhan_khusus_mahasiswa);
            active.nama_kebutuhan_khusus_mahasiswa =
                Set(record.nama_kebutuhan_khusus_mahasiswa.clone());
            active.id_kebutuhan_khusus_ayah = Set(record.id_kebutuhan_khusus_ayah);
            active.nama_kebutuhan_khusus_ayah = Set(record.nama_kebutuhan_khusus_ayah.clone());
            active.id_kebutuhan_khusus_ibu = Set(record.id_kebutuhan_khusus_ibu);
            active.nama_kebutuhan_khusus_ibu = Set(record.nama_kebutuhan_khusus_ibu.clone());
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

            let new_record = biodata_mahasiswa::ActiveModel {
                id: Set(pk_id),
                id_mahasiswa: Set(Some(id_mahasiswa)),
                nama_mahasiswa: Set(record.nama_mahasiswa.clone()),
                jenis_kelamin: Set(record.jenis_kelamin.clone()),
                tempat_lahir: Set(record.tempat_lahir.clone()),
                tanggal_lahir: Set(record.tanggal_lahir),
                id_agama: Set(record.id_agama),
                nama_agama: Set(record.nama_agama.clone()),
                nik: Set(record.nik.clone()),
                nisn: Set(record.nisn.clone()),
                npwp: Set(record.npwp.clone()),
                id_negara: Set(record.id_negara.clone()),
                kewarganegaraan: Set(record.kewarganegaraan.clone()),
                jalan: Set(record.jalan.clone()),
                dusun: Set(record.dusun.clone()),
                rt: Set(record.rt),
                rw: Set(record.rw),
                kelurahan: Set(record.kelurahan.clone()),
                kode_pos: Set(record.kode_pos.clone()),
                id_wilayah: Set(record.id_wilayah.clone()),
                nama_wilayah: Set(record.nama_wilayah.clone()),
                id_jenis_tinggal: Set(record.id_jenis_tinggal.clone()),
                nama_jenis_tinggal: Set(record.nama_jenis_tinggal.clone()),
                id_alat_transportasi: Set(record.id_alat_transportasi.clone()),
                nama_alat_transportasi: Set(record.nama_alat_transportasi.clone()),
                telepon: Set(record.telepon.clone()),
                handphone: Set(record.handphone.clone()),
                email: Set(record.email.clone()),
                penerima_kps: Set(record.penerima_kps),
                nomor_kps: Set(record.nomor_kps.clone()),
                nik_ayah: Set(record.nik_ayah.clone()),
                nama_ayah: Set(record.nama_ayah.clone()),
                tanggal_lahir_ayah: Set(record.tanggal_lahir_ayah),
                id_pendidikan_ayah: Set(record.id_pendidikan_ayah),
                nama_pendidikan_ayah: Set(record.nama_pendidikan_ayah.clone()),
                id_pekerjaan_ayah: Set(record.id_pekerjaan_ayah),
                nama_pekerjaan_ayah: Set(record.nama_pekerjaan_ayah.clone()),
                id_penghasilan_ayah: Set(record.id_penghasilan_ayah),
                nama_penghasilan_ayah: Set(record.nama_penghasilan_ayah.clone()),
                nik_ibu: Set(record.nik_ibu.clone()),
                nama_ibu_kandung: Set(record.nama_ibu_kandung.clone()),
                tanggal_lahir_ibu: Set(record.tanggal_lahir_ibu),
                id_pendidikan_ibu: Set(record.id_pendidikan_ibu),
                nama_pendidikan_ibu: Set(record.nama_pendidikan_ibu.clone()),
                id_pekerjaan_ibu: Set(record.id_pekerjaan_ibu),
                nama_pekerjaan_ibu: Set(record.nama_pekerjaan_ibu.clone()),
                id_penghasilan_ibu: Set(record.id_penghasilan_ibu),
                nama_penghasilan_ibu: Set(record.nama_penghasilan_ibu.clone()),
                nama_wali: Set(record.nama_wali.clone()),
                tanggal_lahir_wali: Set(record.tanggal_lahir_wali),
                id_pendidikan_wali: Set(record.id_pendidikan_wali),
                nama_pendidikan_wali: Set(record.nama_pendidikan_wali.clone()),
                id_pekerjaan_wali: Set(record.id_pekerjaan_wali),
                nama_pekerjaan_wali: Set(record.nama_pekerjaan_wali.clone()),
                id_penghasilan_wali: Set(record.id_penghasilan_wali),
                nama_penghasilan_wali: Set(record.nama_penghasilan_wali.clone()),
                id_kebutuhan_khusus_mahasiswa: Set(record.id_kebutuhan_khusus_mahasiswa),
                nama_kebutuhan_khusus_mahasiswa: Set(record
                    .nama_kebutuhan_khusus_mahasiswa
                    .clone()),
                id_kebutuhan_khusus_ayah: Set(record.id_kebutuhan_khusus_ayah),
                nama_kebutuhan_khusus_ayah: Set(record.nama_kebutuhan_khusus_ayah.clone()),
                id_kebutuhan_khusus_ibu: Set(record.id_kebutuhan_khusus_ibu),
                nama_kebutuhan_khusus_ibu: Set(record.nama_kebutuhan_khusus_ibu.clone()),
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

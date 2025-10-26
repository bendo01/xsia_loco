use chrono::Local;
use loco_rs::prelude::*;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, TransactionTrait};
use serde::{Deserialize, Serialize};

use crate::models::feeder::master::biodata_dosen::{
    _entities::biodata_dosen, feeder_model::ModelInput,
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
        "GetBiodataDosen".to_string()
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
    /// This function processes a batch of biodata dosen records:
    /// 1. Converts each `ModelInput` to database entity
    /// 2. Checks if record exists by `id_dosen`
    /// 3. Updates existing record or inserts new one
    /// 4. Tracks sync timestamp
    ///
    /// # Returns
    /// * `Result<()>` - Ok if all records processed successfully, Err otherwise
    async fn perform(&self, args: WorkerArgs) -> Result<()> {
        println!("=================GetBiodataDosen=======================");
        println!("📦 Processing batch of {} records", args.records.len());

        let mut success_count = 0;
        let mut error_count = 0;

        for (index, record) in args.records.iter().enumerate() {
            match Self::upsert_record(&self.ctx, record).await {
                Ok(action) => {
                    success_count += 1;
                    println!(
                        "  ✅ Record {}/{}: {} - id_dosen={:?}, nidn={:?}",
                        index + 1,
                        args.records.len(),
                        action,
                        record.id_dosen,
                        record.nidn
                    );
                }
                Err(e) => {
                    error_count += 1;
                    eprintln!(
                        "  ❌ Record {}/{}: Failed - id_dosen={:?}, nidn={:?}, error: {}",
                        index + 1,
                        args.records.len(),
                        record.id_dosen,
                        record.nidn,
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
    /// Upsert a single biodata dosen record into the database.
    ///
    /// This function performs an "upsert" operation:
    /// - If a record with the same `id_dosen` exists, it updates it
    /// - If no record exists, it inserts a new one
    ///
    /// # Parameters
    /// * `ctx` - Application context for database access
    /// * `record` - The feeder model data to upsert
    ///
    /// # Returns
    /// * `Result<String>` - "INSERTED" or "UPDATED" on success, error otherwise
    async fn upsert_record(ctx: &AppContext, record: &ModelInput) -> Result<String> {
        // Validate that id_dosen exists (it's the unique key)
        let id_dosen = record
            .id_dosen
            .clone()
            .ok_or_else(|| Error::Message("Missing id_dosen".to_string()))?;

        // Start transaction
        let txn = ctx.db.begin().await?;
        let sync_time = Local::now().naive_local();

        // Check if record exists
        let existing = biodata_dosen::Entity::find()
            .filter(biodata_dosen::Column::DeletedAt.is_null())
            .filter(biodata_dosen::Column::IdDosen.eq(&id_dosen))
            .one(&txn)
            .await?;

        let action = if let Some(existing_record) = existing {
            // Update existing record
            let mut active: biodata_dosen::ActiveModel = existing_record.into_active_model();

            active.nama_dosen = Set(record.nama_dosen.clone());
            active.tempat_lahir = Set(record.tempat_lahir.clone());
            active.tanggal_lahir = Set(record.tanggal_lahir);
            active.jenis_kelamin = Set(record.jenis_kelamin.clone());
            active.id_agama = Set(record.id_agama.map(|v| v.to_string()));
            active.nama_agama = Set(record.nama_agama.clone());
            active.id_status_aktif = Set(record.id_status_aktif.clone());
            active.nama_status_aktif = Set(record.nama_status_aktif.clone());
            active.nidn = Set(record.nidn.clone());
            active.nama_ibu_kandung = Set(record.nama_ibu_kandung.clone());
            active.nik = Set(record.nik.clone());
            active.nip = Set(record.nip.clone());
            active.npwp = Set(record.npwp.clone());
            active.id_jenis_sdm = Set(record.id_jenis_sdm.clone());
            active.nama_jenis_sdm = Set(record.nama_jenis_sdm.clone());
            active.no_sk_cpns = Set(record.no_sk_cpns.clone());
            active.tanggal_sk_cpns = Set(record.tanggal_sk_cpns);
            active.no_sk_pengangkatan = Set(record.no_sk_pengangkatan.clone());
            active.mulai_sk_pengangkatan = Set(record.mulai_sk_pengangkatan.clone());
            active.id_lembaga_pengangkatan = Set(record.id_lembaga_pengangkatan.clone());
            active.nama_lembaga_pengangkatan = Set(record.nama_lembaga_pengangkatan.clone());
            active.id_pangkat_golongan = Set(record.id_pangkat_golongan.clone());
            active.nama_pangkat_golongan = Set(record.nama_pangkat_golongan.clone());
            active.id_sumber_gaji = Set(record.id_sumber_gaji.clone());
            active.nama_sumber_gaji = Set(record.nama_sumber_gaji.clone());
            active.jalan = Set(record.jalan.clone());
            active.dusun = Set(record.dusun.clone());
            active.rt = Set(record.rt.clone());
            active.rw = Set(record.rw.clone());
            active.ds_kel = Set(record.ds_kel.clone());
            active.kode_pos = Set(record.kode_pos.clone());
            active.id_wilayah = Set(record.id_wilayah.clone());
            active.nama_wilayah = Set(record.nama_wilayah.clone());
            active.telepon = Set(record.telepon.clone());
            active.handphone = Set(record.handphone.clone());
            active.email = Set(record.email.clone());
            active.status_pernikahan = Set(record.status_pernikahan.clone());
            active.nama_suami_istri = Set(record.nama_suami_istri.clone());
            active.nip_suami_istri = Set(record.nip_suami_istri.clone());
            active.tanggal_mulai_pns = Set(record.tanggal_mulai_pns);
            active.id_pekerjaan_suami_istri = Set(record.id_pekerjaan_suami_istri);
            active.nama_pekerjaan_suami_istri = Set(record.nama_pekerjaan_suami_istri.clone());
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

            let new_record = biodata_dosen::ActiveModel {
                id: Set(pk_id),
                id_dosen: Set(Some(id_dosen)),
                nama_dosen: Set(record.nama_dosen.clone()),
                tempat_lahir: Set(record.tempat_lahir.clone()),
                tanggal_lahir: Set(record.tanggal_lahir),
                jenis_kelamin: Set(record.jenis_kelamin.clone()),
                id_agama: Set(record.id_agama.map(|v| v.to_string())),
                nama_agama: Set(record.nama_agama.clone()),
                id_status_aktif: Set(record.id_status_aktif.clone()),
                nama_status_aktif: Set(record.nama_status_aktif.clone()),
                nidn: Set(record.nidn.clone()),
                nama_ibu_kandung: Set(record.nama_ibu_kandung.clone()),
                nik: Set(record.nik.clone()),
                nip: Set(record.nip.clone()),
                npwp: Set(record.npwp.clone()),
                id_jenis_sdm: Set(record.id_jenis_sdm.clone()),
                nama_jenis_sdm: Set(record.nama_jenis_sdm.clone()),
                no_sk_cpns: Set(record.no_sk_cpns.clone()),
                tanggal_sk_cpns: Set(record.tanggal_sk_cpns),
                no_sk_pengangkatan: Set(record.no_sk_pengangkatan.clone()),
                mulai_sk_pengangkatan: Set(record.mulai_sk_pengangkatan.clone()),
                id_lembaga_pengangkatan: Set(record.id_lembaga_pengangkatan.clone()),
                nama_lembaga_pengangkatan: Set(record.nama_lembaga_pengangkatan.clone()),
                id_pangkat_golongan: Set(record.id_pangkat_golongan.clone()),
                nama_pangkat_golongan: Set(record.nama_pangkat_golongan.clone()),
                id_sumber_gaji: Set(record.id_sumber_gaji.clone()),
                nama_sumber_gaji: Set(record.nama_sumber_gaji.clone()),
                jalan: Set(record.jalan.clone()),
                dusun: Set(record.dusun.clone()),
                rt: Set(record.rt.clone()),
                rw: Set(record.rw.clone()),
                ds_kel: Set(record.ds_kel.clone()),
                kode_pos: Set(record.kode_pos.clone()),
                id_wilayah: Set(record.id_wilayah.clone()),
                nama_wilayah: Set(record.nama_wilayah.clone()),
                telepon: Set(record.telepon.clone()),
                handphone: Set(record.handphone.clone()),
                email: Set(record.email.clone()),
                status_pernikahan: Set(record.status_pernikahan.clone()),
                nama_suami_istri: Set(record.nama_suami_istri.clone()),
                nip_suami_istri: Set(record.nip_suami_istri.clone()),
                tanggal_mulai_pns: Set(record.tanggal_mulai_pns),
                id_pekerjaan_suami_istri: Set(record.id_pekerjaan_suami_istri),
                nama_pekerjaan_suami_istri: Set(record.nama_pekerjaan_suami_istri.clone()),
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

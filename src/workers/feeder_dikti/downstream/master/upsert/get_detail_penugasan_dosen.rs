use chrono::Local;
use loco_rs::prelude::*;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, TransactionTrait};
use serde::{Deserialize, Serialize};

use crate::models::feeder::master::penugasan_dosen::{
    _entities::penugasan_dosen, feeder_model::ModelInputDetailPenugasanDosen,
};

pub struct Worker {
    pub ctx: AppContext,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct WorkerArgs {
    pub records: Vec<ModelInputDetailPenugasanDosen>,
}

#[async_trait]
impl BackgroundWorker<WorkerArgs> for Worker {
    fn build(ctx: &AppContext) -> Self {
        Self { ctx: ctx.clone() }
    }

    fn class_name() -> String {
        "GetDetailPenugasanDosen".to_string()
    }

    fn tags() -> Vec<String> {
        Vec::new()
    }

    async fn perform(&self, args: WorkerArgs) -> Result<()> {
        println!("=================GetDetailPenugasanDosen=======================");
        println!("📦 Processing batch of {} records", args.records.len());

        let mut success_count = 0;
        let mut error_count = 0;

        for (index, record) in args.records.iter().enumerate() {
            match Self::upsert_record(&self.ctx, record).await {
                Ok(action) => {
                    success_count += 1;
                    println!(
                        "  ✅ Record {}/{}: {} - id_registrasi_dosen={:?}, nama_dosen={:?}",
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
                        "  ❌ Record {}/{}: Failed - id_registrasi_dosen={:?}, nama_dosen={:?}, error: {}",
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
    async fn upsert_record(ctx: &AppContext, record: &ModelInputDetailPenugasanDosen) -> Result<String> {
        // Validate that id_registrasi_dosen exists (it's the unique key)
        let id_registrasi_dosen = record
            .id_registrasi_dosen
            .ok_or_else(|| Error::Message("Missing id_registrasi_dosen".to_string()))?;

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

            active.id_tahun_ajaran = Set(record.id_tahun_ajaran.clone());
            active.nama_tahun_ajaran = Set(record.nama_tahun_ajaran.clone());
            active.id_perguruan_tinggi = Set(record.id_perguruan_tinggi);
            active.nama_perguruan_tinggi = Set(record.nama_perguruan_tinggi.clone());
            active.nidn = Set(record.nidn.clone());
            active.nuptk = Set(record.nuptk.clone());
            active.id_dosen = Set(record.id_dosen);
            active.nama_dosen = Set(record.nama_dosen.clone());
            active.id_prodi = Set(record.id_prodi);
            active.nama_program_studi = Set(record.nama_program_studi.clone());
            active.nomor_surat_tugas = Set(record.nomor_surat_tugas.clone());
            active.tanggal_surat_tugas = Set(record.tanggal_surat_tugas.map(|d| d.format("%d-%m-%Y").to_string()));
            active.mulai_surat_tugas = Set(record.mulai_surat_tugas.map(|d| d.format("%d-%m-%Y").to_string()));
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
                id_registrasi_dosen: Set(Some(id_registrasi_dosen)),
                id_tahun_ajaran: Set(record.id_tahun_ajaran.clone()),
                nama_tahun_ajaran: Set(record.nama_tahun_ajaran.clone()),
                id_perguruan_tinggi: Set(record.id_perguruan_tinggi),
                nama_perguruan_tinggi: Set(record.nama_perguruan_tinggi.clone()),
                nidn: Set(record.nidn.clone()),
                nuptk: Set(record.nuptk.clone()),
                id_dosen: Set(record.id_dosen),
                nama_dosen: Set(record.nama_dosen.clone()),
                id_prodi: Set(record.id_prodi),
                nama_program_studi: Set(record.nama_program_studi.clone()),
                nomor_surat_tugas: Set(record.nomor_surat_tugas.clone()),
                tanggal_surat_tugas: Set(record.tanggal_surat_tugas.map(|d| d.format("%d-%m-%Y").to_string())),
                mulai_surat_tugas: Set(record.mulai_surat_tugas.map(|d| d.format("%d-%m-%Y").to_string())),
                sync_at: Set(Some(sync_time)),
                created_at: Set(sync_time),
                updated_at: Set(sync_time),
                created_by: Set(None),
                updated_by: Set(None),
                deleted_at: Set(None),
                jenis_kelamin: Set(None),
                tgl_create: Set(None),
                tgl_ptk_keluar: Set(None),
                id_stat_pegawai: Set(None),
                id_jns_keluar: Set(None),
                id_ikatan_kerja: Set(None),
                apakah_homebase: Set(None),
            };

            new_record.insert(&txn).await?;
            "INSERTED"
        };

        // Commit transaction
        txn.commit().await?;

        Ok(action.to_string())
    }
}
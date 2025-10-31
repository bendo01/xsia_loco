use chrono::Local;
use loco_rs::prelude::*;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, TransactionTrait};
use serde::{Deserialize, Serialize};

use crate::models::feeder::master::penugasan_dosen::{
    _entities::penugasan_dosen, feeder_model::ModelInputListPenugasanSemuaDosen,
};

pub struct Worker {
    pub ctx: AppContext,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct WorkerArgs {
    pub records: Vec<ModelInputListPenugasanSemuaDosen>,
}

#[async_trait]
impl BackgroundWorker<WorkerArgs> for Worker {
    fn build(ctx: &AppContext) -> Self {
        Self { ctx: ctx.clone() }
    }

    fn class_name() -> String {
        "GetListPenugasanSemuaDosen".to_string()
    }

    fn tags() -> Vec<String> {
        Vec::new()
    }

    async fn perform(&self, args: WorkerArgs) -> Result<()> {
        println!("=================GetListPenugasanSemuaDosen=======================");
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
    async fn upsert_record(ctx: &AppContext, record: &ModelInputListPenugasanSemuaDosen) -> Result<String> {
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

            active.id_dosen = Set(record.id_dosen);
            active.nama_dosen = Set(record.nama_dosen.clone());
            active.nuptk = Set(record.nuptk.clone());
            active.jenis_kelamin = Set(record.jenis_kelamin.clone());
            active.id_tahun_ajaran = Set(record.id_tahun_ajaran.clone());
            active.nama_tahun_ajaran = Set(record.nama_tahun_ajaran.clone());
            active.id_prodi = Set(record.id_prodi);
            active.nama_program_studi = Set(record.nama_program_studi.clone());
            active.nomor_surat_tugas = Set(record.nomor_surat_tugas.clone());
            active.tanggal_surat_tugas = Set(record.tanggal_surat_tugas.map(|d| d.format("%d-%m-%Y").to_string()));
            active.apakah_homebase = Set(record.apakah_homebase.as_ref().and_then(|s| {
                match s.as_str() {
                    "1" => Some(true),
                    "0" => Some(false),
                    _ => None,
                }
            }));
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
                id_dosen: Set(record.id_dosen),
                nama_dosen: Set(record.nama_dosen.clone()),
                nuptk: Set(record.nuptk.clone()),
                jenis_kelamin: Set(record.jenis_kelamin.clone()),
                id_tahun_ajaran: Set(record.id_tahun_ajaran.clone()),
                nama_tahun_ajaran: Set(record.nama_tahun_ajaran.clone()),
                id_prodi: Set(record.id_prodi),
                nama_program_studi: Set(record.nama_program_studi.clone()),
                nomor_surat_tugas: Set(record.nomor_surat_tugas.clone()),
                tanggal_surat_tugas: Set(record.tanggal_surat_tugas.map(|d| d.format("%d-%m-%Y").to_string())),
                apakah_homebase: Set(record.apakah_homebase.as_ref().and_then(|s| {
                    match s.as_str() {
                        "1" => Some(true),
                        "0" => Some(false),
                        _ => None,
                    }
                })),
                sync_at: Set(Some(sync_time)),
                created_at: Set(sync_time),
                updated_at: Set(sync_time),
                created_by: Set(None),
                updated_by: Set(None),
                deleted_at: Set(None),
                nidn: Set(None),
                id_perguruan_tinggi: Set(None),
                nama_perguruan_tinggi: Set(None),
                mulai_surat_tugas: Set(None),
                tgl_create: Set(None),
                tgl_ptk_keluar: Set(None),
                id_stat_pegawai: Set(None),
                id_jns_keluar: Set(None),
                id_ikatan_kerja: Set(None),
            };

            new_record.insert(&txn).await?;
            "INSERTED"
        };

        // Commit transaction
        txn.commit().await?;

        Ok(action.to_string())
    }
}
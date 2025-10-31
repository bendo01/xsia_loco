use chrono::Local;
use loco_rs::prelude::*;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, TransactionTrait};
use serde::{Deserialize, Serialize};

use crate::models::feeder::master::matakuliah::{
    _entities::matakuliah, feeder_model::ModelInputListMatakuliah,
};

pub struct Worker {
    pub ctx: AppContext,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct WorkerArgs {
    pub records: Vec<ModelInputListMatakuliah>,
}

#[async_trait]
impl BackgroundWorker<WorkerArgs> for Worker {
    fn build(ctx: &AppContext) -> Self {
        Self { ctx: ctx.clone() }
    }

    fn class_name() -> String {
        "GetListMatakuliah".to_string()
    }

    fn tags() -> Vec<String> {
        Vec::new()
    }

    async fn perform(&self, args: WorkerArgs) -> Result<()> {
        println!("=================GetListMatakuliah=======================");
        println!("📦 Processing batch of {} records", args.records.len());

        let mut success_count = 0;
        let mut error_count = 0;

        for (index, record) in args.records.iter().enumerate() {
            match Self::upsert_record(&self.ctx, record).await {
                Ok(action) => {
                    success_count += 1;
                    println!(
                        "  ✅ Record {}/{}: {} - id_matkul={}, kode={}",
                        index + 1,
                        args.records.len(),
                        action,
                        record.id_matkul,
                        record.kode_mata_kuliah
                    );
                }
                Err(e) => {
                    error_count += 1;
                    eprintln!(
                        "  ❌ Record {}/{}: Failed - id_matkul={}, kode={}, error: {}",
                        index + 1,
                        args.records.len(),
                        record.id_matkul,
                        record.kode_mata_kuliah,
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
    async fn upsert_record(ctx: &AppContext, record: &ModelInputListMatakuliah) -> Result<String> {
        let id_matkul = record.id_matkul;

        // Start transaction
        let txn = ctx.db.begin().await?;
        let sync_time = Local::now().naive_local();

        // Check if record exists
        let existing = matakuliah::Entity::find()
            .filter(matakuliah::Column::DeletedAt.is_null())
            .filter(matakuliah::Column::IdMatkul.eq(id_matkul))
            .one(&txn)
            .await?;

        let action = if let Some(existing_record) = existing {
            // Update existing record
            let mut active: matakuliah::ActiveModel = existing_record.into_active_model();

            active.kode_mata_kuliah = Set(Some(record.kode_mata_kuliah.clone()));
            active.nama_mata_kuliah = Set(Some(record.nama_mata_kuliah.clone()));
            active.id_prodi = Set(Some(record.id_prodi));
            active.nama_program_studi = Set(Some(record.nama_program_studi.clone()));
            active.id_jenis_mata_kuliah = Set(record.id_jenis_mata_kuliah.clone());
            active.nama_jenis_mata_kuliah = Set(record.nama_jenis_mata_kuliah.clone());
            active.id_kelompok_mata_kuliah = Set(record.id_kelompok_mata_kuliah.clone());
            active.nama_kelompok_mata_kuliah = Set(record.nama_kelompok_mata_kuliah.clone());
            active.sks_mata_kuliah = Set(record.sks_mata_kuliah);
            active.sks_tatap_muka = Set(record.sks_tatap_muka);
            active.sks_praktek = Set(record.sks_praktek);
            active.sks_praktek_lapangan = Set(record.sks_praktek_lapangan);
            active.sks_simulasi = Set(record.sks_simulasi);
            active.metode_kuliah = Set(record.metode_kuliah.clone());
            active.ada_sap = Set(record.ada_sap);
            active.ada_silabus = Set(record.ada_silabus);
            active.ada_bahan_ajar = Set(record.ada_bahan_ajar);
            active.ada_acara_praktek = Set(record.ada_acara_praktek);
            active.ada_diktat = Set(record.ada_diktat);
            active.tanggal_mulai_efektif = Set(record.tanggal_mulai_efektif.map(|d| d.and_hms_opt(0, 0, 0).unwrap()));
            active.tanggal_selesai_efektif = Set(record.tanggal_selesai_efektif.map(|d| d.and_hms_opt(0, 0, 0).unwrap()));
            active.id_jenj_didik = Set(record.id_jenj_didik.clone());
            active.tgl_create = Set(record.tgl_create);
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

            let new_record = matakuliah::ActiveModel {
                id: Set(pk_id),
                id_matkul: Set(Some(id_matkul)),
                kode_mata_kuliah: Set(Some(record.kode_mata_kuliah.clone())),
                nama_mata_kuliah: Set(Some(record.nama_mata_kuliah.clone())),
                id_prodi: Set(Some(record.id_prodi)),
                nama_program_studi: Set(Some(record.nama_program_studi.clone())),
                id_jenis_mata_kuliah: Set(record.id_jenis_mata_kuliah.clone()),
                nama_jenis_mata_kuliah: Set(record.nama_jenis_mata_kuliah.clone()),
                id_kelompok_mata_kuliah: Set(record.id_kelompok_mata_kuliah.clone()),
                nama_kelompok_mata_kuliah: Set(record.nama_kelompok_mata_kuliah.clone()),
                sks_mata_kuliah: Set(record.sks_mata_kuliah),
                sks_tatap_muka: Set(record.sks_tatap_muka),
                sks_praktek: Set(record.sks_praktek),
                sks_praktek_lapangan: Set(record.sks_praktek_lapangan),
                sks_simulasi: Set(record.sks_simulasi),
                metode_kuliah: Set(record.metode_kuliah.clone()),
                ada_sap: Set(record.ada_sap),
                ada_silabus: Set(record.ada_silabus),
                ada_bahan_ajar: Set(record.ada_bahan_ajar),
                ada_acara_praktek: Set(record.ada_acara_praktek),
                ada_diktat: Set(record.ada_diktat),
                tanggal_mulai_efektif: Set(record.tanggal_mulai_efektif.map(|d| d.and_hms_opt(0, 0, 0).unwrap())),
                tanggal_selesai_efektif: Set(record.tanggal_selesai_efektif.map(|d| d.and_hms_opt(0, 0, 0).unwrap())),
                id_jenj_didik: Set(record.id_jenj_didik.clone()),
                tgl_create: Set(record.tgl_create),
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
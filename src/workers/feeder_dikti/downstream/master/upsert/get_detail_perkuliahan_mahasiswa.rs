use chrono::Local;
use loco_rs::prelude::*;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, TransactionTrait};
use serde::{Deserialize, Serialize};

use crate::models::feeder::master::perkuliahan_mahasiswa::{
    _entities::perkuliahan_mahasiswa, feeder_model::ModelInputDetailPerkuliahanMahasiswa,
};

pub struct Worker {
    pub ctx: AppContext,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct WorkerArgs {
    pub records: Vec<ModelInputDetailPerkuliahanMahasiswa>,
}

#[async_trait]
impl BackgroundWorker<WorkerArgs> for Worker {
    fn build(ctx: &AppContext) -> Self {
        Self { ctx: ctx.clone() }
    }

    fn class_name() -> String {
        "GetDetailPerkuliahanMahasiswa".to_string()
    }

    fn tags() -> Vec<String> {
        Vec::new()
    }

    async fn perform(&self, args: WorkerArgs) -> Result<()> {
        println!("=================GetDetailPerkuliahanMahasiswa=======================");
        println!("📦 Processing batch of {} records", args.records.len());

        let mut success_count = 0;
        let mut error_count = 0;

        for (index, record) in args.records.iter().enumerate() {
            match Self::upsert_record(&self.ctx, record).await {
                Ok(action) => {
                    success_count += 1;
                    println!(
                        "  ✅ Record {}/{}: {} - id_registrasi_mahasiswa={:?}, nim={:?}, id_semester={:?}",
                        index + 1,
                        args.records.len(),
                        action,
                        record.id_registrasi_mahasiswa,
                        record.nim,
                        record.id_semester
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
    async fn upsert_record(ctx: &AppContext, record: &ModelInputDetailPerkuliahanMahasiswa) -> Result<String> {
        // Validate that required fields exist - using composite key (id_registrasi_mahasiswa + id_semester)
        let id_registrasi_mahasiswa = record
            .id_registrasi_mahasiswa
            .ok_or_else(|| Error::Message("Missing id_registrasi_mahasiswa".to_string()))?;

        let id_semester = record
            .id_semester
            .as_ref()
            .ok_or_else(|| Error::Message("Missing id_semester".to_string()))?;

        // Start transaction
        let txn = ctx.db.begin().await?;
        let sync_time = Local::now().naive_local();

        // Check if record exists using composite key
        let existing = perkuliahan_mahasiswa::Entity::find()
            .filter(perkuliahan_mahasiswa::Column::DeletedAt.is_null())
            .filter(perkuliahan_mahasiswa::Column::IdRegistrasiMahasiswa.eq(id_registrasi_mahasiswa))
            .filter(perkuliahan_mahasiswa::Column::IdSemester.eq(id_semester))
            .one(&txn)
            .await?;

        let action = if let Some(existing_record) = existing {
            // Update existing record
            let mut active: perkuliahan_mahasiswa::ActiveModel = existing_record.into_active_model();

            active.id_prodi = Set(record.id_prodi);
            active.nama_program_studi = Set(record.nama_program_studi.clone());
            active.angkatan = Set(record.angkatan.clone());
            active.nim = Set(record.nim.clone());
            active.nama_mahasiswa = Set(record.nama_mahasiswa.clone());
            active.nama_semester = Set(record.nama_semester.clone());
            active.id_status_mahasiswa = Set(record.id_status_mahasiswa.clone());
            active.nama_status_mahasiswa = Set(record.nama_status_mahasiswa.clone());
            active.ips = Set(record.ips);
            active.ipk = Set(record.ipk);
            active.sks_semester = Set(record.sks_semester);
            active.sks_total = Set(record.sks_total);
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

            let new_record = perkuliahan_mahasiswa::ActiveModel {
                id: Set(pk_id),
                id_registrasi_mahasiswa: Set(Some(id_registrasi_mahasiswa)),
                nim: Set(record.nim.clone()),
                nama_mahasiswa: Set(record.nama_mahasiswa.clone()),
                angkatan: Set(record.angkatan.clone()),
                id_prodi: Set(record.id_prodi),
                nama_program_studi: Set(record.nama_program_studi.clone()),
                id_periode_masuk: Set(None), // Not in Detail API
                id_semester: Set(Some(id_semester.clone())),
                nama_semester: Set(record.nama_semester.clone()),
                id_status_mahasiswa: Set(record.id_status_mahasiswa.clone()),
                nama_status_mahasiswa: Set(record.nama_status_mahasiswa.clone()),
                ips: Set(record.ips),
                ipk: Set(record.ipk),
                sks_semester: Set(record.sks_semester),
                sks_total: Set(record.sks_total),
                biaya_kuliah_smt: Set(None), // Not in Detail API
                id_pembiayaan: Set(None), // Not in Detail API
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
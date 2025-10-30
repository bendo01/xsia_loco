use chrono::Local;
use loco_rs::prelude::*;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, TransactionTrait};
use serde::{Deserialize, Serialize};

use crate::models::feeder::master::kelas_kuliah::{
    _entities::kelas_kuliah, feeder_model::ModelInputListKelasKuliah,
};

pub struct Worker {
    pub ctx: AppContext,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct WorkerArgs {
    pub records: Vec<ModelInputListKelasKuliah>,
}

#[async_trait]
impl BackgroundWorker<WorkerArgs> for Worker {
    fn build(ctx: &AppContext) -> Self {
        Self { ctx: ctx.clone() }
    }

    fn class_name() -> String {
        "GetListKelasKuliah".to_string()
    }

    fn tags() -> Vec<String> {
        Vec::new()
    }

    async fn perform(&self, args: WorkerArgs) -> Result<()> {
        println!("=================GetListKelasKuliah=======================");
        println!("📦 Processing batch of {} records", args.records.len());

        let mut success_count = 0;
        let mut error_count = 0;

        for (index, record) in args.records.iter().enumerate() {
            match Self::upsert_record(&self.ctx, record).await {
                Ok(action) => {
                    success_count += 1;
                    println!(
                        "  ✅ Record {}/{}: {} - id_kelas_kuliah={}, nama_kelas={}",
                        index + 1,
                        args.records.len(),
                        action,
                        record.id_kelas_kuliah,
                        record.nama_kelas_kuliah.as_deref().unwrap_or("N/A")
                    );
                }
                Err(e) => {
                    error_count += 1;
                    eprintln!(
                        "  ❌ Record {}/{}: Failed - id_kelas_kuliah={}, error: {}",
                        index + 1,
                        args.records.len(),
                        record.id_kelas_kuliah,
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
    async fn upsert_record(
        ctx: &AppContext,
        record: &ModelInputListKelasKuliah,
    ) -> Result<String> {
        let id_kelas_kuliah = record.id_kelas_kuliah;

        let txn = ctx.db.begin().await?;
        let sync_time = Local::now().naive_local();

        let existing = kelas_kuliah::Entity::find()
            .filter(kelas_kuliah::Column::DeletedAt.is_null())
            .filter(kelas_kuliah::Column::IdKelasKuliah.eq(id_kelas_kuliah))
            .one(&txn)
            .await?;

        let action = if let Some(existing_record) = existing {
            let mut active: kelas_kuliah::ActiveModel = existing_record.into_active_model();

            active.id_prodi = Set(record.id_prodi);
            active.nama_program_studi = Set(record.nama_program_studi.clone());
            active.id_semester = Set(record.id_semester.clone());
            active.nama_semester = Set(record.nama_semester.clone());
            active.id_matkul = Set(record.id_matkul);
            active.kode_mata_kuliah = Set(record.kode_mata_kuliah.clone());
            active.nama_mata_kuliah = Set(record.nama_mata_kuliah.clone());
            active.nama_kelas_kuliah = Set(record.nama_kelas_kuliah.clone());
            active.sks = Set(record.sks);
            active.id_dosen = Set(record.id_dosen.clone());
            active.nama_dosen = Set(record.nama_dosen.clone());
            active.jumlah_mahasiswa = Set(record.jumlah_mahasiswa);
            active.apa_untuk_pditt = Set(record.apa_untuk_pditt.map(|v| v != 0));
            active.sync_at = Set(Some(sync_time));
            active.updated_at = Set(Some(sync_time));

            active.update(&txn).await?;
            "UPDATED"
        } else {
            let uuid_v7 = uuid7::uuid7();
            let uuid_string = uuid_v7.to_string();
            let pk_id = uuid::Uuid::parse_str(&uuid_string)
                .map_err(|e| Error::Message(format!("Invalid UUID: {}", e)))?;

            let new_record = kelas_kuliah::ActiveModel {
                id: Set(pk_id),
                id_kelas_kuliah: Set(id_kelas_kuliah),
                id_prodi: Set(record.id_prodi),
                nama_program_studi: Set(record.nama_program_studi.clone()),
                id_semester: Set(record.id_semester.clone()),
                nama_semester: Set(record.nama_semester.clone()),
                id_matkul: Set(record.id_matkul),
                kode_mata_kuliah: Set(record.kode_mata_kuliah.clone()),
                nama_mata_kuliah: Set(record.nama_mata_kuliah.clone()),
                nama_kelas_kuliah: Set(record.nama_kelas_kuliah.clone()),
                sks: Set(record.sks),
                id_dosen: Set(record.id_dosen.clone()),
                nama_dosen: Set(record.nama_dosen.clone()),
                jumlah_mahasiswa: Set(record.jumlah_mahasiswa),
                apa_untuk_pditt: Set(record.apa_untuk_pditt.map(|v| v != 0)),
                sks_mk: Set(None),
                sks_tm: Set(None),
                sks_prak: Set(None),
                sks_prak_lap: Set(None),
                sks_sim: Set(None),
                bahasan: Set(None),
                tanggal_mulai_efektif: Set(None),
                tanggal_akhir_efektif: Set(None),
                kapasitas: Set(None),
                tanggal_tutup_daftar: Set(None),
                prodi_penyelenggara: Set(None),
                perguruan_tinggi_penyelenggara: Set(None),
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

        txn.commit().await?;

        Ok(action.to_string())
    }
}
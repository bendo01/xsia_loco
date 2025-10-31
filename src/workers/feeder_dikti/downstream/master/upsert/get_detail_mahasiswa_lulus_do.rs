use chrono::Local;
use loco_rs::prelude::*;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, TransactionTrait};
use serde::{Deserialize, Serialize};

use crate::models::feeder::master::mahasiswa_lulusan_dropout::{
    _entities::mahasiswa_lulusan_dropout, feeder_model::ModelInputDetailMahasiswaLulusDO as ModelInput,
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
    fn build(ctx: &AppContext) -> Self {
        Self { ctx: ctx.clone() }
    }

    fn class_name() -> String {
        "GetDetailMahasiswaLulusDO".to_string()
    }

    fn tags() -> Vec<String> {
        Vec::new()
    }

    async fn perform(&self, args: WorkerArgs) -> Result<()> {
        println!("=================GetDetailMahasiswaLulusDO=======================");
        println!("📦 Processing batch of {} records", args.records.len());

        let mut success_count = 0;
        let mut error_count = 0;

        for (index, record) in args.records.iter().enumerate() {
            match Self::upsert_record(&self.ctx, record).await {
                Ok(action) => {
                    success_count += 1;
                    println!(
                        "  ✅ Record {}/{}: {} - id_registrasi_mahasiswa={}, nim={}",
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
                        "  ❌ Record {}/{}: Failed - id_registrasi_mahasiswa={}, nim={}, error: {}",
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
    async fn upsert_record(ctx: &AppContext, record: &ModelInput) -> Result<String> {
        let id_registrasi_mahasiswa = record.id_registrasi_mahasiswa;

        // Start transaction
        let txn = ctx.db.begin().await?;
        let sync_time = Local::now().naive_local();

        // Check if record exists
        let existing = mahasiswa_lulusan_dropout::Entity::find()
            .filter(mahasiswa_lulusan_dropout::Column::DeletedAt.is_null())
            .filter(mahasiswa_lulusan_dropout::Column::IdRegistrasiMahasiswa.eq(id_registrasi_mahasiswa))
            .one(&txn)
            .await?;

        let action = if let Some(existing_record) = existing {
            // Update existing record
            let mut active: mahasiswa_lulusan_dropout::ActiveModel = existing_record.into_active_model();

            active.id_prodi = Set(Some(record.id_prodi));
            active.nama_program_studi = Set(Some(record.nama_program_studi.clone()));
            active.id_mahasiswa = Set(Some(record.id_mahasiswa));
            active.nim = Set(Some(record.nim.clone()));
            active.nama_mahasiswa = Set(Some(record.nama_mahasiswa.clone()));
            active.angkatan = Set(Some(record.angkatan.clone()));
            active.id_jenis_keluar = Set(record.id_jenis_keluar.clone());
            active.nama_jenis_keluar = Set(record.nama_jenis_keluar.clone());
            active.tanggal_keluar = Set(record.tanggal_keluar);
            active.keterangan = Set(record.keterangan.clone());
            active.nomor_sk_yudisium = Set(record.nomor_sk_yudisium.clone());
            active.tanggal_sk_yudisium = Set(record.tanggal_sk_yudisium);
            active.ipk = Set(record.ipk);
            active.nomor_ijazah = Set(record.nomor_ijazah.clone());
            active.jalur_skripsi = Set(record.jalur_skripsi.clone());
            active.judul_skripsi = Set(record.judul_skripsi.clone());
            active.no_sertifikat_profesi = Set(record.no_sertifikat_profesi.clone());
            active.bulan_awal_bimbingan = Set(record.bulan_awal_bimbingan.clone());
            active.bulan_akhir_bimbingan = Set(record.bulan_akhir_bimbingan.clone());
            active.id_dosen = Set(record.id_dosen);
            active.nidn = Set(record.nidn.clone());
            active.nuptk = Set(record.nuptk.clone());
            active.nama_dosen = Set(record.nama_dosen.clone());
            active.pembimbing_ke = Set(record.pembimbing_ke);
            active.asal_ijazah = Set(record.asal_ijazah.clone());
            active.id_periode_keluar = Set(record.id_periode_keluar.clone());
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

            let new_record = mahasiswa_lulusan_dropout::ActiveModel {
                id: Set(pk_id),
                id_registrasi_mahasiswa: Set(Some(id_registrasi_mahasiswa)),
                id_prodi: Set(Some(record.id_prodi)),
                nama_program_studi: Set(Some(record.nama_program_studi.clone())),
                id_mahasiswa: Set(Some(record.id_mahasiswa)),
                nim: Set(Some(record.nim.clone())),
                nama_mahasiswa: Set(Some(record.nama_mahasiswa.clone())),
                angkatan: Set(Some(record.angkatan.clone())),
                id_jenis_keluar: Set(record.id_jenis_keluar.clone()),
                nama_jenis_keluar: Set(record.nama_jenis_keluar.clone()),
                tanggal_keluar: Set(record.tanggal_keluar),
                keterangan: Set(record.keterangan.clone()),
                nomor_sk_yudisium: Set(record.nomor_sk_yudisium.clone()),
                tanggal_sk_yudisium: Set(record.tanggal_sk_yudisium),
                ipk: Set(record.ipk),
                nomor_ijazah: Set(record.nomor_ijazah.clone()),
                jalur_skripsi: Set(record.jalur_skripsi.clone()),
                judul_skripsi: Set(record.judul_skripsi.clone()),
                no_sertifikat_profesi: Set(record.no_sertifikat_profesi.clone()),
                bulan_awal_bimbingan: Set(record.bulan_awal_bimbingan.clone()),
                bulan_akhir_bimbingan: Set(record.bulan_akhir_bimbingan.clone()),
                id_dosen: Set(record.id_dosen),
                nidn: Set(record.nidn.clone()),
                nuptk: Set(record.nuptk.clone()),
                nama_dosen: Set(record.nama_dosen.clone()),
                pembimbing_ke: Set(record.pembimbing_ke),
                asal_ijazah: Set(record.asal_ijazah.clone()),
                id_periode_keluar: Set(record.id_periode_keluar.clone()),
                sync_at: Set(Some(sync_time)),
                created_at: Set(Some(sync_time)),
                updated_at: Set(Some(sync_time)),
                created_by: Set(None),
                updated_by: Set(None),
                deleted_at: Set(None),
                id_perguruan_tinggi: Set(None),
                tgl_masuk_sp: Set(None),
                tgl_create: Set(None),
                tgl_keluar: Set(None),
                skhun: Set(None),
                no_peserta_ujian: Set(None),
                sks_diakui: Set(None),
                id_jns_daftar: Set(None),
                nm_jns_daftar: Set(None),
                id_jalur_masuk: Set(None),
                id_pembiayaan: Set(None),
                biaya_masuk_kuliah: Set(None),
                id_minat_bidang: Set(None),
                bidang_mayor: Set(None),
                bidang_minor: Set(None),
                a_pindah_mhs_asing: Set(None),
                id_pt_asal: Set(None),
                id_prodi_asal: Set(None),
                nm_pt_asal: Set(None),
                nm_prodi_asal: Set(None),
                namapt: Set(None),
                id_jur: Set(None),
                nm_smt: Set(None),
                status_sync: Set(None),
                tanggal_terbit_ijazah: Set(None),
            };

            new_record.insert(&txn).await?;
            "INSERTED"
        };

        // Commit transaction
        txn.commit().await?;

        Ok(action.to_string())
    }
}
use chrono::Local;
use loco_rs::prelude::*;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, TransactionTrait};
use serde::{Deserialize, Serialize};

use crate::models::feeder::master::mahasiswa_lulusan_dropout::{
    _entities::mahasiswa_lulusan_dropout, feeder_model::ModelInputListMahasiswaLulusDO as ModelInput,
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
        "GetListMahasiswaLulusDO".to_string()
    }

    fn tags() -> Vec<String> {
        Vec::new()
    }

    async fn perform(&self, args: WorkerArgs) -> Result<()> {
        println!("=================GetListMahasiswaLulusDO=======================");
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

            active.id_mahasiswa = Set(Some(record.id_mahasiswa));
            active.id_perguruan_tinggi = Set(Some(record.id_perguruan_tinggi));
            active.id_prodi = Set(Some(record.id_prodi));
            active.tgl_masuk_sp = Set(record.tgl_masuk_sp);
            active.tgl_keluar = Set(record.tgl_keluar);
            active.skhun = Set(record.skhun.clone());
            active.no_peserta_ujian = Set(record.no_peserta_ujian.clone());
            active.sks_diakui = Set(record.sks_diakui.clone());
            active.jalur_skripsi = Set(record.jalur_skripsi.clone());
            active.judul_skripsi = Set(record.judul_skripsi.clone());
            active.bulan_awal_bimbingan = Set(record.bln_awal_bimbingan.clone());
            active.bulan_akhir_bimbingan = Set(record.bln_akhir_bimbingan.clone());
            active.nomor_sk_yudisium = Set(record.sk_yudisium.clone());
            active.tanggal_sk_yudisium = Set(record.tgl_sk_yudisium);
            active.ipk = Set(record.ipk);
            active.no_sertifikat_profesi = Set(record.sert_prof.clone());
            active.a_pindah_mhs_asing = Set(record.a_pindah_mhs_asing.clone());
            active.id_pt_asal = Set(record.id_pt_asal);
            active.id_prodi_asal = Set(record.id_prodi_asal);
            active.nm_pt_asal = Set(record.nm_pt_asal.clone());
            active.nm_prodi_asal = Set(record.nm_prodi_asal.clone());
            active.id_jns_daftar = Set(Some(record.id_jns_daftar.clone()));
            active.id_jenis_keluar = Set(record.id_jns_keluar.clone());
            active.id_jalur_masuk = Set(Some(record.id_jalur_masuk.clone()));
            active.id_pembiayaan = Set(record.id_pembiayaan.clone());
            active.id_minat_bidang = Set(record.id_minat_bidang.clone());
            active.bidang_mayor = Set(record.bidang_mayor.clone());
            active.bidang_minor = Set(record.bidang_minor.clone());
            active.biaya_masuk_kuliah = Set(Some(record.biaya_masuk_kuliah.clone()));
            active.namapt = Set(Some(record.namapt.clone()));
            active.id_jur = Set(Some(record.id_jur.clone()));
            active.nm_jns_daftar = Set(Some(record.nm_jns_daftar.clone()));
            active.nm_smt = Set(Some(record.nm_smt.clone()));
            active.nim = Set(Some(record.nim.clone()));
            active.nama_mahasiswa = Set(Some(record.nama_mahasiswa.clone()));
            active.nama_program_studi = Set(Some(record.nama_program_studi.clone()));
            active.angkatan = Set(Some(record.angkatan.clone()));
            active.tanggal_keluar = Set(record.tanggal_keluar);
            active.id_periode_keluar = Set(record.id_periode_keluar.clone());
            active.keterangan = Set(record.keterangan.clone());
            active.nomor_ijazah = Set(record.no_seri_ijazah.clone());
            active.tanggal_terbit_ijazah = Set(record.tanggal_terbit_ijazah);
            active.status_sync = Set(Some(record.status_sync.clone()));
            active.nama_jenis_keluar = Set(record.nama_jenis_keluar.clone());
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

            let new_record = mahasiswa_lulusan_dropout::ActiveModel {
                id: Set(pk_id),
                id_registrasi_mahasiswa: Set(Some(id_registrasi_mahasiswa)),
                id_mahasiswa: Set(Some(record.id_mahasiswa)),
                id_perguruan_tinggi: Set(Some(record.id_perguruan_tinggi)),
                id_prodi: Set(Some(record.id_prodi)),
                tgl_masuk_sp: Set(record.tgl_masuk_sp),
                tgl_keluar: Set(record.tgl_keluar),
                skhun: Set(record.skhun.clone()),
                no_peserta_ujian: Set(record.no_peserta_ujian.clone()),
                sks_diakui: Set(record.sks_diakui.clone()),
                jalur_skripsi: Set(record.jalur_skripsi.clone()),
                judul_skripsi: Set(record.judul_skripsi.clone()),
                bulan_awal_bimbingan: Set(record.bln_awal_bimbingan.clone()),
                bulan_akhir_bimbingan: Set(record.bln_akhir_bimbingan.clone()),
                nomor_sk_yudisium: Set(record.sk_yudisium.clone()),
                tanggal_sk_yudisium: Set(record.tgl_sk_yudisium),
                ipk: Set(record.ipk),
                no_sertifikat_profesi: Set(record.sert_prof.clone()),
                a_pindah_mhs_asing: Set(record.a_pindah_mhs_asing.clone()),
                id_pt_asal: Set(record.id_pt_asal),
                id_prodi_asal: Set(record.id_prodi_asal),
                nm_pt_asal: Set(record.nm_pt_asal.clone()),
                nm_prodi_asal: Set(record.nm_prodi_asal.clone()),
                id_jns_daftar: Set(Some(record.id_jns_daftar.clone())),
                id_jenis_keluar: Set(record.id_jns_keluar.clone()),
                id_jalur_masuk: Set(Some(record.id_jalur_masuk.clone())),
                id_pembiayaan: Set(record.id_pembiayaan.clone()),
                id_minat_bidang: Set(record.id_minat_bidang.clone()),
                bidang_mayor: Set(record.bidang_mayor.clone()),
                bidang_minor: Set(record.bidang_minor.clone()),
                biaya_masuk_kuliah: Set(Some(record.biaya_masuk_kuliah.clone())),
                namapt: Set(Some(record.namapt.clone())),
                id_jur: Set(Some(record.id_jur.clone())),
                nm_jns_daftar: Set(Some(record.nm_jns_daftar.clone())),
                nm_smt: Set(Some(record.nm_smt.clone())),
                nim: Set(Some(record.nim.clone())),
                nama_mahasiswa: Set(Some(record.nama_mahasiswa.clone())),
                nama_program_studi: Set(Some(record.nama_program_studi.clone())),
                angkatan: Set(Some(record.angkatan.clone())),
                tanggal_keluar: Set(record.tanggal_keluar),
                id_periode_keluar: Set(record.id_periode_keluar.clone()),
                keterangan: Set(record.keterangan.clone()),
                nomor_ijazah: Set(record.no_seri_ijazah.clone()),
                tanggal_terbit_ijazah: Set(record.tanggal_terbit_ijazah),
                status_sync: Set(Some(record.status_sync.clone())),
                nama_jenis_keluar: Set(record.nama_jenis_keluar.clone()),
                tgl_create: Set(record.tgl_create),
                asal_ijazah: Set(String::new()), // Not in List model, set default
                id_dosen: Set(None),
                nidn: Set(None),
                nuptk: Set(None),
                nama_dosen: Set(None),
                pembimbing_ke: Set(None),
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
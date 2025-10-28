use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::ConnectionTrait;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let db = manager.get_connection();
        db.execute_unprepared(
            "
            CREATE SCHEMA IF NOT EXISTS feeder_master;
            ",
        )
        .await?;
        db.execute_unprepared(
            "
            CREATE TABLE IF NOT EXISTS feeder_master.mahasiswa_lulusan_dropout
            (
                id uuid DEFAULT uuid_generate_v7(),
                created_at timestamp(0) without time zone DEFAULT now(),
                updated_at timestamp(0) without time zone DEFAULT now(),
                deleted_at timestamp(0) without time zone,
                sync_at timestamp(0) without time zone,
                created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                id_registrasi_mahasiswa uuid,
                id_mahasiswa uuid,
                id_perguruan_tinggi uuid,
                id_prodi uuid,
                tgl_masuk_sp date,
                tgl_keluar date,
                skhun text,
                no_peserta_ujian text,
                no_seri_ijazah text,
                tgl_create date,
                sks_diakui real,
                jalur_skripsi text,
                judul_skripsi text,
                bln_awal_bimbingan text,
                bln_akhir_bimbingan text,
                sk_yudisium text,
                tgl_sk_yudisium date,
                ipk real,
                sert_prof text,
                a_pindah_mhs_asing text,
                id_pt_asal uuid,
                id_prodi_asal uuid,
                nm_pt_asal text,
                nm_prodi_asal text,
                id_jns_daftar text,
                id_jns_keluar text,
                id_jalur_masuk text,
                id_pembiayaan text,
                id_minat_bidang text,
                bidang_mayor text,
                bidang_minor text,
                biaya_masuk_kuliah real,
                namapt text,
                id_jur text,
                nm_jns_daftar text,
                nm_smt text,
                nim text,
                nama_mahasiswa text,
                nama_program_studi text,
                angkatan text,
                id_jenis_keluar text,
                nama_jenis_keluar text,
                tanggal_keluar date,
                id_periode_keluar text,
                keterangan text,
                no_sertifikat_profesi text,
                tanggal_terbit_ijazah date,
                status_sync text,
                CONSTRAINT feeder_master_mahasiswa_lulusan_dropout_pkey PRIMARY KEY (id)
            )
            ",
        )
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TABLE IF EXISTS feeder_master.mahasiswa_lulusan_dropout")
            .await?;

        Ok(())
    }
}

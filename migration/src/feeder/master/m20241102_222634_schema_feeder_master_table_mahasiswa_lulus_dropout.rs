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
                
                -- Registration identifiers
                id_registrasi_mahasiswa uuid,
                id_mahasiswa uuid,
                id_perguruan_tinggi uuid,
                id_prodi uuid,
                
                -- Student information
                nama_program_studi varchar(255),
                nim varchar(255),
                nama_mahasiswa varchar(255),
                angkatan varchar(255),
                
                -- Entry dates
                tgl_masuk_sp date,
                tgl_create date,
                
                -- Exit information
                tgl_keluar date,
                tanggal_keluar date,
                id_jenis_keluar varchar(255) NOT NULL,
                nama_jenis_keluar varchar(255) NOT NULL,
                id_periode_keluar varchar(255) NOT NULL,
                keterangan text,
                
                -- Graduation decree
                nomor_sk_yudisium varchar(255),
                tanggal_sk_yudisium date,
                
                -- Academic performance
                ipk real DEFAULT 0,
                
                -- Certificate information
                nomor_ijazah varchar(255),
                asal_ijazah varchar(10) NOT NULL,
                no_sertifikat_profesi varchar(255),
                tanggal_terbit_ijazah date,
                
                -- Thesis information
                jalur_skripsi varchar(255),
                judul_skripsi text,
                
                -- Guidance period
                bulan_awal_bimbingan varchar(255),
                bulan_akhir_bimbingan varchar(255),
                
                -- Advisor information
                id_dosen uuid,
                nidn varchar(255),
                nuptk varchar(255),
                nama_dosen varchar(255),
                pembimbing_ke integer,
                
                -- Admission information
                skhun varchar(255),
                no_peserta_ujian varchar(255),
                sks_diakui varchar(255),
                id_jns_daftar varchar(255),
                nm_jns_daftar varchar(255),
                id_jalur_masuk varchar(255),
                id_pembiayaan varchar(255),
                biaya_masuk_kuliah varchar(255),
                
                -- Interest fields
                id_minat_bidang varchar(255),
                bidang_mayor varchar(255),
                bidang_minor varchar(255),
                
                -- Transfer student information
                a_pindah_mhs_asing varchar(255),
                id_pt_asal uuid,
                id_prodi_asal uuid,
                nm_pt_asal varchar(255),
                nm_prodi_asal varchar(255),
                
                -- Institution information
                namapt varchar(255),
                id_jur varchar(255),
                nm_smt varchar(255),
                
                -- Sync status
                status_sync varchar(255),
                
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

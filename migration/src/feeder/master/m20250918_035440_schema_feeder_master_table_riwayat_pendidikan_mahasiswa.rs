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
            CREATE TABLE IF NOT EXISTS feeder_master.riwayat_pendidikan_mahasiswa
            (
                id uuid DEFAULT uuid_generate_v7(),
                id_registrasi_mahasiswa uuid,
                id_mahasiswa uuid,
                nim character varying(255),
                nama_mahasiswa character varying(255),
                id_jenis_daftar integer,
                nama_jenis_daftar character varying(255),
                id_jalur_daftar integer,
                id_periode_masuk character varying(255),
                nama_periode_masuk character varying(255),
                id_jenis_keluar integer,
                keterangan_keluar character varying(255),
                id_perguruan_tinggi uuid,
                nama_perguruan_tinggi character varying(255),
                id_prodi uuid,
                nama_program_studi character varying(255),
                sks_diakui integer,
                id_perguruan_tinggi_asal uuid,
                nama_perguruan_tinggi_asal character varying(255),
                id_prodi_asal uuid,
                nama_program_studi_asal character varying(255),
                jenis_kelamin character varying(255),
                tanggal_daftar date,
                nama_ibu_kandung character varying(255),
                id_pembiayaan integer,
                biaya_masuk integer,
                id_bidang_minat character varying(255),
                nm_bidang_minat character varying(255),
                id_periode_keluar character varying(255),
                tanggal_keluar date,
                last_update date,
                tgl_create date,
                status_sync character varying(255),
                nama_pembiayaan_awal character varying(255),
                created_at timestamp(0) without time zone DEFAULT now(),
                updated_at timestamp(0) without time zone DEFAULT now(),
                deleted_at timestamp(0) without time zone,
                sync_at timestamp(0) without time zone,
                created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                CONSTRAINT feeder_master_riwayat_pendidikan_mahasiswa_pkey PRIMARY KEY (id)
            )
            ",
        )
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TABLE IF EXISTS feeder_master.riwayat_pendidikan_mahasiswa")
            .await?;

        Ok(())
    }
}

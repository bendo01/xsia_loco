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
            CREATE TABLE IF NOT EXISTS feeder_master.penugasan_dosen
            (
                id uuid DEFAULT uuid_generate_v7(),
                id_registrasi_dosen uuid,
                jk character varying(255),
                id_dosen uuid,
                nama_dosen character varying(255),
                nidn character varying(255),
                id_tahun_ajaran character varying(255),
                nama_tahun_ajaran character varying(255),
                id_perguruan_tinggi uuid,
                nama_perguruan_tinggi character varying(255),
                id_prodi uuid,
                nama_program_studi character varying(255),
                nomor_surat_tugas character varying(255),
                tanggal_surat_tugas date,
                mulai_surat_tugas date,
                tgl_create date,
                tgl_ptk_keluar timestamp(0) without time zone,
                id_stat_pegawai integer,
                id_jns_keluar character varying(255),
                id_ikatan_kerja character varying(255),
                a_sp_homebase character varying(255),
                created_at timestamp(0) without time zone DEFAULT now(),
                updated_at timestamp(0) without time zone DEFAULT now(),
                deleted_at timestamp(0) without time zone,
                sync_at timestamp(0) without time zone,
                created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                CONSTRAINT feeder_master_penugasan_dosen_pkey PRIMARY KEY (id)
            )
            ",
        )
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TABLE IF EXISTS feeder_master.penugasan_dosen")
            .await?;

        Ok(())
    }
}

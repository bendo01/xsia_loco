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
            CREATE TABLE IF NOT EXISTS feeder_master.dosen_pengajar_kelas_kuliah
            (
                id uuid DEFAULT uuid_generate_v7(),
                id_aktivitas_mengajar uuid,
                id_registrasi_dosen uuid,
                id_dosen uuid,
                nidn character varying(255),
                nuptk character varying(255),
                nama_dosen character varying(255),
                id_kelas_kuliah uuid,
                nama_kelas_kuliah character varying(255),
                id_substansi uuid,
                sks_substansi_total real,
                rencana_minggu_pertemuan integer,
                realisasi_minggu_pertemuan integer,
                id_jenis_evaluasi character varying(255),
                nama_jenis_evaluasi character varying(255),
                id_prodi uuid,
                id_semester character varying(255),
                created_at timestamp(0) without time zone DEFAULT now(),
                updated_at timestamp(0) without time zone DEFAULT now(),
                deleted_at timestamp(0) without time zone,
                sync_at timestamp(0) without time zone,
                created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                CONSTRAINT pfeeder_master_dosen_pengajar_kelas_kuliah_pkey PRIMARY KEY (id)
            )
            ",
        )
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TABLE IF EXISTS feeder_master.dosen_pengajar_kelas_kuliah")
            .await?;

        Ok(())
    }
}

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
            CREATE TABLE IF NOT EXISTS feeder_master.perkuliahan_mahasiswa
            (
                id uuid DEFAULT uuid_generate_v7(),
                created_at timestamp(0) without time zone DEFAULT now(),
                updated_at timestamp(0) without time zone DEFAULT now(),
                deleted_at timestamp(0) without time zone,
                sync_at timestamp(0) without time zone,
                created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                id_registrasi_mahasiswa uuid,
                nim varchar(255),
                nama_mahasiswa varchar(255),
                angkatan varchar(255),
                id_prodi uuid,
                nama_program_studi varchar(255),
                id_periode_masuk varchar(255),
                id_semester varchar(255),
                nama_semester varchar(255),
                id_status_mahasiswa varchar(255),
                nama_status_mahasiswa varchar(255),
                ips real DEFAULT 0,
                ipk real DEFAULT 0,
                sks_semester real DEFAULT 0,
                sks_total real DEFAULT 0,
                biaya_kuliah_smt real DEFAULT 0,
                id_pembiayaan varchar(255),
                status_sync varchar(255),
                CONSTRAINT feeder_master_perkuliahan_mahasiswa_pkey PRIMARY KEY (id)
            )
            ",
        )
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TABLE IF EXISTS feeder_master.perkuliahan_mahasiswa")
            .await?;

        Ok(())
    }
}

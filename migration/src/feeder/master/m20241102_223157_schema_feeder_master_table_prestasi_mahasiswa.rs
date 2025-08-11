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
            CREATE TABLE IF NOT EXISTS feeder_master.prestasi_mahasiswa
            (
                id uuid DEFAULT uuid_generate_v7(),
                id_prestasi uuid,
                id_mahasiswa uuid,
                nama_mahasiswa character varying(255),
                id_jenis_prestasi uuid,
                nama_jenis_prestasi character varying(255),
                id_tingkat_prestasi uuid,
                nama_tingkat_prestasi character varying(255),
                nama_prestasi character varying(255),
                tahun_prestasi integer,
                penyelenggara character varying(255),
                peringkat integer,
                id_aktivitas uuid,
                created_at timestamp(0) without time zone DEFAULT now(),
                updated_at timestamp(0) without time zone DEFAULT now(),
                deleted_at timestamp(0) without time zone,
                sync_at timestamp(0) without time zone,
                created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                CONSTRAINT feeder_master_prestasi_mahasiswa_pkey PRIMARY KEY (id)
            )
            ",
        )
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TABLE IF EXISTS feeder_master.prestasi_mahasiswa")
            .await?;

        Ok(())
    }
}

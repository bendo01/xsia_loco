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
            CREATE SCHEMA IF NOT EXISTS feeder_export;
            ",
        )
        .await?;
        db.execute_unprepared(
            "
            CREATE TABLE IF NOT EXISTS feeder_export.data_aktifitas_kuliah_mahasiswa
            (
                id uuid DEFAULT uuid_generate_v7(),
                id_periode uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                nama_periode character varying(255),
                id_registrasi_mahasiswa uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                nim integer,
                nama_mahasiswa character varying(255),
                id_prodi uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                id_status_mahasiswa uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                nama_status_mahasiswa character varying(255),
                ips real DEFAULT 0.0,
                sks_semester real DEFAULT 0.0,
                ipk real DEFAULT 0.0,
                total_sks real DEFAULT 0.0,
                created_at timestamp(0) without time zone DEFAULT now(),
                updated_at timestamp(0) without time zone DEFAULT now(),
                deleted_at timestamp(0) without time zone,
                sync_at timestamp(0) without time zone,
                created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                CONSTRAINT feeder_export_data_aktifitas_kuliah_mahasiswa_pkey PRIMARY KEY (id)
            )
            ",
        )
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                "DROP TABLE IF EXISTS feeder_export.data_aktifitas_kuliah_mahasiswa",
            )
            .await?;

        Ok(())
    }
}

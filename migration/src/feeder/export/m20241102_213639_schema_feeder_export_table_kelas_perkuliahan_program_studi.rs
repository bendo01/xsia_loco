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
            CREATE TABLE IF NOT EXISTS feeder_export.kelas_perkuliahan_program_studi
            (
                id uuid DEFAULT uuid_generate_v7(),
                id_prodi uuid,
                nama_program_studi character varying(255),
                id_periode uuid,
                nama_periode character varying(255),
                id_matkul uuid,
                kode_mata_kuliah character varying(255),
                nama_mata_kulias character varying(255) ,
                id_kelas_kuliah uuid,
                nama_kelas_kuliah character varying(255),
                sks_mata_kuliah real DEFAULT 0.0,
                jumlah_krs real DEFAULT 0.0,
                jumlah_dosen integer DEFAULT 0,
                created_at timestamp(0) without time zone DEFAULT now(),
                updated_at timestamp(0) without time zone DEFAULT now(),
                deleted_at timestamp(0) without time zone,
                sync_at timestamp(0) without time zone,
                created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                CONSTRAINT feeder_export_kelas_perkuliahan_program_studi_pkey PRIMARY KEY (id)
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
                "DROP TABLE IF EXISTS feeder_export.kelas_perkuliahan_program_studi",
            )
            .await?;

        Ok(())
    }
}

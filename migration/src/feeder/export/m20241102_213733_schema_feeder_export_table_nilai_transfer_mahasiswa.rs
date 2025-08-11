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
            CREATE TABLE IF NOT EXISTS feeder_export.nilai_transfer_mahasiswa
            (
                id uuid DEFAULT uuid_generate_v7(),
                id_registrasi_mahasiswa uuid,
                id_mahasiswa uuid,
                nim integer,
                nama_mahasiswa character varying(255),
                id_prodi uuid,
                program_studi character varying(255),
                angkatan integer DEFAULT 0,
                id_transfer uuid,
                kode_mata_kuliah_asal character varying(255),
                nama_mata_kuliah_asal character varying(255) ,
                sks_mata_kuliah_asal real DEFAULT 0.0,
                nilai_huruf_asal character varying(255),
                kode_matkul_baru character varying(255),
                nama_mata_kuliah_baru character varying(255),
                sks_mata_kuliah_diakui real DEFAULT 0.0,
                nilai_huruf_diakui real DEFAULT 0.0,
                nilai_angka_diakui real DEFAULT 0.0,
                created_at timestamp(0) without time zone DEFAULT now(),
                updated_at timestamp(0) without time zone DEFAULT now(),
                deleted_at timestamp(0) without time zone,
                sync_at timestamp(0) without time zone,
                created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                CONSTRAINT feeder_export_nilai_transfer_mahasiswa_pkey PRIMARY KEY (id)
            )
            ",
        )
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TABLE IF EXISTS feeder_export.nilai_transfer_mahasiswa")
            .await?;

        Ok(())
    }
}

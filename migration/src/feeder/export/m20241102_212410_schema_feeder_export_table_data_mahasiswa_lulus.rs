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
            CREATE TABLE IF NOT EXISTS feeder_export.data_mahasiswa_lulus
            (
                id uuid DEFAULT uuid_generate_v7(),

                id_registrasi_mahasiswa uuid,
                nim integer,
                nama_mahasiswa character varying(255) ,
                jenis_kelamin character varying(255),
                id_prodi uuid,
                nama_program_studi character varying(255),
                id_periode uuid,
                nama_periode_masuk character varying(255),
                id_jenis_keluar uuid,
                nama_jenis_keluar character varying(255),
                nomor_ijazah character varying(255),
                tanggal_keluar date,
                keterangan character varying(255),
                created_at timestamp(0) without time zone DEFAULT now(),
                updated_at timestamp(0) without time zone DEFAULT now(),
                deleted_at timestamp(0) without time zone,
                sync_at timestamp(0) without time zone,
                created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                CONSTRAINT feeder_export_data_mahasiswa_lulus_pkey PRIMARY KEY (id)
            )
            ",
        )
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TABLE IF EXISTS feeder_export.data_mahasiswa_lulus")
            .await?;

        Ok(())
    }
}

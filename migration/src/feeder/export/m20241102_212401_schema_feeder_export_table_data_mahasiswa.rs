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
            CREATE TABLE IF NOT EXISTS feeder_export.data_mahasiswa
            (
                id uuid DEFAULT uuid_generate_v7(),

                id_mahasiswa uuid,
                id_registrasi_mahasiswa uuid,
                nim integer,
                nama_mahasiswa character varying(255),
                id_prodi uuid,
                program_studi character varying(255) ,
                periode_masuk character varying(255),
                status_mahasiswa character varying(255) ,
                id_jenis_daftar uuid,
                nama_jenis_daftar character varying(255),
                jenis_kelamin character varying(255),
                tempat_lahir character varying(255),
                tanggal_lahir date,
                id_agama uuid,
                nama_agama character varying(255),
                created_at timestamp(0) without time zone DEFAULT now(),
                updated_at timestamp(0) without time zone DEFAULT now(),
                deleted_at timestamp(0) without time zone,
                sync_at timestamp(0) without time zone,
                created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                CONSTRAINT feeder_export_data_mahasiswa_pkey PRIMARY KEY (id)
            )
            ",
        )
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TABLE IF EXISTS pfeeder_export.data_mahasiswa")
            .await?;

        Ok(())
    }
}

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
            CREATE TABLE IF NOT EXISTS feeder_master.mahasiswa
            (
                id uuid DEFAULT uuid_generate_v7(),
                nama_mahasiswa character varying(255),
                jenis_kelamin character varying(255),
                tanggal_lahir date,
                id_perguruan_tinggi uuid,
                nipd character varying(255),
                ipk real,
                total_sks real,
                id_sms uuid,
                id_mahasiswa uuid,
                id_agama integer,
                nama_agama character varying(255),
                id_prodi character varying(255),
                nama_program_studi character varying(255),
                id_status_mahasiswa integer,
                nama_status_mahasiswa character varying(255),
                nim character varying(255),
                id_periode character varying(255),
                nama_periode_masuk character varying(255),
                id_registrasi_mahasiswa uuid,
                id_periode_keluar character varying(255),
                tanggal_keluar date,
                last_update date,
                tgl_create date,
                status_sync character varying(255),
                created_at timestamp(0) without time zone DEFAULT now(),
                updated_at timestamp(0) without time zone DEFAULT now(),
                deleted_at timestamp(0) without time zone,
                sync_at timestamp(0) without time zone,
                created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                CONSTRAINT feeder_master_mahasiswa_pkey PRIMARY KEY (id)
            )
            ",
        )
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TABLE IF EXISTS feeder_master.mahasiswa")
            .await?;

        Ok(())
    }
}

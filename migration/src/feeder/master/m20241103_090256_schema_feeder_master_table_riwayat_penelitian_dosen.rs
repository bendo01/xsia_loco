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
            CREATE TABLE IF NOT EXISTS feeder_master.riwayat_penelitian_dosen
            (
                id uuid DEFAULT uuid_generate_v7(),
                id_dosen uuid,
                nidn character varying(255),
                nama_dosen character varying(255),
                id_penelitian uuid,
                judul_penelitian text ,
                id_kelompok_bidang uuid,
                kode_kelompok_bidang character varying(255),
                nama_kelompok_bidang character varying(255),
                id_lembaga_iptek uuid,
                nama_lembaga_iptek character varying(255),
                tahun_kegiatan character varying(255),
                created_at timestamp(0) without time zone DEFAULT now(),
                updated_at timestamp(0) without time zone DEFAULT now(),
                deleted_at timestamp(0) without time zone,
                sync_at timestamp(0) without time zone,
                created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                CONSTRAINT feeder_master_riwayat_penelitian_dosen_pkey PRIMARY KEY (id)
            )
            ",
        )
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TABLE IF EXISTS feeder_master.riwayat_penelitian_dosen")
            .await?;

        Ok(())
    }
}

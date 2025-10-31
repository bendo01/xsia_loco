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
            CREATE TABLE IF NOT EXISTS feeder_master.matakuliah
            (
                id uuid DEFAULT uuid_generate_v7(),
                created_at timestamp(0) without time zone DEFAULT now(),
                updated_at timestamp(0) without time zone DEFAULT now(),
                deleted_at timestamp(0) without time zone,
                sync_at timestamp(0) without time zone,
                created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                
                -- Feeder DIKTI fields
                id_matkul uuid,
                kode_mata_kuliah character varying(255),
                nama_mata_kuliah character varying(255),
                id_prodi uuid,
                nama_program_studi character varying(255),
                id_jenis_mata_kuliah character varying(255),
                nama_jenis_mata_kuliah character varying(255),
                id_kelompok_mata_kuliah character varying(255),
                nama_kelompok_mata_kuliah character varying(255),
                sks_mata_kuliah real,
                sks_tatap_muka real,
                sks_praktek real,
                sks_praktek_lapangan real,
                sks_simulasi real,
                metode_kuliah character varying(255),
                ada_sap boolean DEFAULT false,
                ada_silabus boolean DEFAULT false,
                ada_bahan_ajar boolean DEFAULT false,
                ada_acara_praktek boolean DEFAULT false,
                ada_diktat boolean DEFAULT false,
                tanggal_mulai_efektif timestamp(0) without time zone,
                tanggal_selesai_efektif timestamp(0) without time zone,
                id_jenj_didik character varying(5),
                tgl_create timestamp(0) without time zone,
                status_sync character varying(255),
                
                CONSTRAINT feeder_master_matakuliah_pkey PRIMARY KEY (id)
            )
            ",
        )
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TABLE IF EXISTS feeder_master.matakuliah")
            .await?;

        Ok(())
    }
}

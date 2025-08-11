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
                id_matkul uuid,
                kode_mata_kuliah character varying(255),
                nama_mata_kuliah character varying(255),
                id_prodi uuid,
                nama_program_studi character varying(255),
                id_jenis_mata_kuliah character varying(255),
                id_kelompok_mata_kuliah character varying(255),
                sks_mata_kuliah character varying(255),
                sks_tatap_muka character varying(255),
                sks_praktek character varying(255),
                sks_praktek_lapangan character varying(255),
                sks_simulasi character varying(255),
                metode_kuliah text ,
                ada_sap character varying(255),
                ada_silabus character varying(255),
                ada_bahan_ajar character varying(255),
                ada_acara_praktek character varying(255),
                ada_diktat character varying(255),
                tanggal_mulai_efektif timestamp with time zone,
                tanggal_selesai_efektif timestamp with time zone,
                created_at timestamp(0) without time zone DEFAULT now(),
                updated_at timestamp(0) without time zone DEFAULT now(),
                deleted_at timestamp(0) without time zone,
                sync_at timestamp(0) without time zone,
                created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
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

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
            CREATE TABLE IF NOT EXISTS feeder_master.kelas_kuliah
            (
                id uuid DEFAULT uuid_generate_v7(),
                created_at timestamp(0) without time zone DEFAULT now(),
                updated_at timestamp(0) without time zone DEFAULT now(),
                deleted_at timestamp(0) without time zone,
                sync_at timestamp(0) without time zone,
                created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                
                id_kelas_kuliah uuid NOT NULL,
                id_prodi uuid,
                nama_program_studi varchar(255),
                id_semester varchar(255),
                nama_semester varchar(255),
                id_matkul uuid,
                kode_mata_kuliah varchar(255),
                nama_mata_kuliah varchar(255),
                nama_kelas_kuliah varchar(255),
                sks_mk real,
                sks_tm real,
                sks_prak real,
                sks_prak_lap real,
                sks_sim real,
                bahasan text,
                tanggal_mulai_efektif date,
                tanggal_akhir_efektif date,
                kapasitas integer,
                tanggal_tutup_daftar date,
                prodi_penyelenggara varchar(255),
                perguruan_tinggi_penyelenggara varchar(255),
                sks real,
                id_dosen varchar(255),
                nama_dosen text,
                jumlah_mahasiswa integer,
                apa_untuk_pditt boolean,
                CONSTRAINT feeder_master_kelas_kuliah_pkey PRIMARY KEY (id_kelas_kuliah)
            )
            ",
        )
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TABLE IF EXISTS feeder_master.kelas_kuliah")
            .await?;

        Ok(())
    }
}

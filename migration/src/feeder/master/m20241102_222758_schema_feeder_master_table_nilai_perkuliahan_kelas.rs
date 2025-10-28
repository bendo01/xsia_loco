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
            CREATE TABLE IF NOT EXISTS feeder_master.nilai_perkuliahan_kelas
            (
                id uuid DEFAULT uuid_generate_v7(),
                created_at timestamp(0) without time zone DEFAULT now(),
                updated_at timestamp(0) without time zone DEFAULT now(),
                deleted_at timestamp(0) without time zone,
                sync_at timestamp(0) without time zone,
                created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                id_matkul uuid,
                kode_mata_kuliah text,
                nama_mata_kuliah text,
                id_kelas_kuliah uuid,
                nama_kelas_kuliah text,
                sks_mata_kuliah real DEFAULT 0,
                jumlah_mahasiswa_krs integer,
                jumlah_mahasiswa_dapat_nilai integer,
                sks_tm real DEFAULT 0,
                sks_prak real DEFAULT 0,
                sks_prak_lap real DEFAULT 0,
                sks_sim real DEFAULT 0,
                bahasan_case text,
                a_selenggara_pditt integer DEFAULT 0,
                a_pengguna_pditt integer DEFAULT 0,
                kuota_pditt integer DEFAULT 0,
                tgl_mulai_koas date,
                tgl_selesai_koas date,
                id_mou uuid,
                id_kls_pditt uuid,
                id_sms uuid,
                id_smt text,
                tgl_create date,
                lingkup_kelas integer,
                mode_kuliah text,
                nm_smt text,
                nama_prodi text,
                status_sync text,
                CONSTRAINT feeder_master_nilai_perkuliahan_kelas_pkey PRIMARY KEY (id)
            )
            ",
        )
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TABLE IF EXISTS feeder_master.nilai_perkuliahan_kelas")
            .await?;

        Ok(())
    }
}

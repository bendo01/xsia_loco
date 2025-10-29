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
            CREATE TABLE IF NOT EXISTS feeder_master.periode_perkuliahan
            (
                id uuid DEFAULT uuid_generate_v7(),
                id_prodi uuid,
                nama_program_studi text,
                id_semester varchar(10),
                nama_semester text,
                jumlah_target_mahasiswa_baru varchar(10),
                tanggal_awal_perkuliahan date,
                tanggal_akhir_perkuliahan date,
                jumlah_pendaftar_ikut_seleksi varchar(10),
                jumlah_pendaftar_lulus_seleksi varchar(10),
                jumlah_daftar_ulang varchar(10),
                jumlah_mengundurkan_diri varchar(10),
                jumlah_minggu_pertemuan varchar(10),
                metode_kul text,
                metode_kul_eks text,
                tgl_create date,
                last_update date,
                status_sync text,
                created_at timestamp(0) without time zone DEFAULT now(),
                updated_at timestamp(0) without time zone DEFAULT now(),
                deleted_at timestamp(0) without time zone,
                sync_at timestamp(0) without time zone,
                created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                CONSTRAINT feeder_master_periode_perkuliahan_pkey PRIMARY KEY (id),
                CONSTRAINT unique_periode_prodi_semester UNIQUE (id_prodi, id_semester)
            )
            ",
        )
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TABLE IF EXISTS feeder_master.periode_perkuliahan")
            .await?;

        Ok(())
    }
}

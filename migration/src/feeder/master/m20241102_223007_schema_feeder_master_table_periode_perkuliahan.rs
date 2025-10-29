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
                created_at timestamp(0) without time zone DEFAULT now(),
                updated_at timestamp(0) without time zone DEFAULT now(),
                deleted_at timestamp(0) without time zone,
                sync_at timestamp(0) without time zone,
                created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                id_prodi uuid,
                nama_program_studi varchar(255),
                id_semester varchar(10),
                nama_semester varchar(50),
                jumlah_target_mahasiswa_baru integer,
                jumlah_pendaftar_ikut_seleksi integer,
                jumlah_pendaftar_lulus_seleksi integer,
                jumlah_daftar_ulang integer,
                jumlah_mengundurkan_diri integer,
                tanggal_awal_perkuliahan date,
                tanggal_akhir_perkuliahan date,
                jumlah_minggu_pertemuan integer,
                metode_kul varchar(100),
                metode_kul_eks varchar(100),
                tgl_create date,
                last_update date,
                status_sync varchar(50),
                CONSTRAINT feeder_master_periode_perkuliahan_pkey PRIMARY KEY (id),
                CONSTRAINT feeder_master_periode_perkuliahan_unique_prodi_semester UNIQUE (id_prodi, id_semester)
            )
            ",
        )
        .await?;

        // Create indexes for faster lookups
        db.execute_unprepared(
            "
            CREATE INDEX IF NOT EXISTS idx_feeder_master_periode_perkuliahan_id_prodi
            ON feeder_master.periode_perkuliahan(id_prodi);
            ",
        )
        .await?;

        db.execute_unprepared(
            "
            CREATE INDEX IF NOT EXISTS idx_feeder_master_periode_perkuliahan_id_semester
            ON feeder_master.periode_perkuliahan(id_semester);
            ",
        )
        .await?;

        db.execute_unprepared(
            "
            CREATE INDEX IF NOT EXISTS idx_feeder_master_periode_perkuliahan_status_sync
            ON feeder_master.periode_perkuliahan(status_sync);
            ",
        )
        .await?;

        db.execute_unprepared(
            "
            CREATE INDEX IF NOT EXISTS idx_feeder_master_periode_perkuliahan_tanggal_awal
            ON feeder_master.periode_perkuliahan(tanggal_awal_perkuliahan);
            ",
        )
        .await?;

        // Add comments for documentation
        db.execute_unprepared(
            "
            COMMENT ON TABLE feeder_master.periode_perkuliahan IS 'Periode perkuliahan data from feeder - combines GetListPeriodePerkuliahan and GetDetailPeriodePerkuliahan';
            COMMENT ON COLUMN feeder_master.periode_perkuliahan.id_prodi IS 'ID program studi (UUID)';
            COMMENT ON COLUMN feeder_master.periode_perkuliahan.id_semester IS 'ID semester format YYYYS (e.g., 20201 = 2020/2021 Ganjil)';
            COMMENT ON COLUMN feeder_master.periode_perkuliahan.jumlah_pendaftar_ikut_seleksi IS 'Jumlah calon mahasiswa yang ikut seleksi';
            COMMENT ON COLUMN feeder_master.periode_perkuliahan.jumlah_pendaftar_lulus_seleksi IS 'Jumlah calon mahasiswa yang lulus seleksi';
            COMMENT ON COLUMN feeder_master.periode_perkuliahan.jumlah_daftar_ulang IS 'Jumlah mahasiswa yang daftar ulang';
            COMMENT ON COLUMN feeder_master.periode_perkuliahan.jumlah_mengundurkan_diri IS 'Jumlah mahasiswa yang mengundurkan diri';
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

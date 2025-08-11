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
                nama_program_studi character varying(255),
                id_semester character varying(255),
                nama_semester character varying(255),
                jumlah_target_mahasiswa_baru character varying(255),
                tanggal_awal_perkuliahan date,
                tanggal_akhir_perkuliahan date,
                calon_ikut_seleksi character varying(255),
                calon_lulus_seleksi character varying(255),
                daftar_sbg_mhs character varying(255),
                pst_undur_diri character varying(255),
                jml_mgu_kul character varying(255),
                metode_kul text,
                metode_kul_eks text,
                tgl_create date,
                last_update date,
                status_sync character varying(255),
                created_at timestamp(0) without time zone DEFAULT now(),
                updated_at timestamp(0) without time zone DEFAULT now(),
                deleted_at timestamp(0) without time zone,
                sync_at timestamp(0) without time zone,
                created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                CONSTRAINT feeder_master_periode_perkuliahan_pkey PRIMARY KEY (id)
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

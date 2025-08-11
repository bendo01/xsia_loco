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
            CREATE TABLE IF NOT EXISTS feeder_master.aktifitas_mahasiswa
            (
                id uuid DEFAULT uuid_generate_v7(),
                asal_data character varying(255),
                nm_asaldata character varying(255) ,
                id_aktivitas uuid,
                jenis_anggota character varying(255),
                nama_jenis_anggota character varying(255),
                id_jenis_aktivitas uuid,
                nama_jenis_aktivitas character varying(255),
                id_prodi uuid,
                nama_prodi character varying(255),
                id_semester uuid,
                nama_semester character varying(255),
                judul character varying(255) ,
                keterangan character varying(255),
                lokasi character varying(255),
                sk_tugas character varying(255),
                tanggal_sk_tugas date,
                untuk_kampus_merdeka integer DEFAULT 0,
                tanggal_mulai date,
                tanggal_selesai date,
                created_at timestamp(0) without time zone DEFAULT now(),
                updated_at timestamp(0) without time zone DEFAULT now(),
                deleted_at timestamp(0) without time zone,
                sync_at timestamp(0) without time zone,
                created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                CONSTRAINT feeder_master_aktifitas_mahasiswa_pkey PRIMARY KEY (id)
            )
            ",
        )
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TABLE IF EXISTS feeder_master.aktifitas_mahasiswa")
            .await?;

        Ok(())
    }
}

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
            CREATE TABLE IF NOT EXISTS feeder_master.nilai_transfer_pendidikan_mahasiswa
            (
                id uuid DEFAULT uuid_generate_v7(),
                id_transfer uuid,
                id_registrasi_mahasiswa uuid,
                nim character varying(255),
                nama_mahasiswa character varying(255),
                id_prodi uuid,
                nama_program_studi character varying(255),
                id_periode_masuk character varying(255),
                kode_mata_kuliah_asal character varying(255),
                nama_mata_kuliah_asal character varying(255),
                sks_mata_kuliah_asal real,
                nilai_huruf_asal character varying(255),
                id_matkul uuid,
                kode_matkul_diakui character varying(255),
                nama_mata_kuliah_diakui character varying(255),
                sks_mata_kuliah_diakui real,
                nilai_huruf_diakui character varying(255),
                nilai_angka_diakui real,
                id_perguruan_tinggi uuid,
                id_aktivitas character varying(255),
                judul text ,
                id_jenis_aktivitas character varying(255),
                nama_jenis_aktivitas character varying(255),
                id_semester character varying(255),
                nama_semester character varying(255),
                status_sync character varying(255),
                created_at timestamp(0) without time zone DEFAULT now(),
                updated_at timestamp(0) without time zone DEFAULT now(),
                deleted_at timestamp(0) without time zone,
                sync_at timestamp(0) without time zone,
                created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                CONSTRAINT feeder_master_nilai_transfer_pendidikan_mahasiswa_pkey PRIMARY KEY (id)
            )
            ",
        )
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                "DROP TABLE IF EXISTS feeder_master.nilai_transfer_pendidikan_mahasiswa",
            )
            .await?;

        Ok(())
    }
}

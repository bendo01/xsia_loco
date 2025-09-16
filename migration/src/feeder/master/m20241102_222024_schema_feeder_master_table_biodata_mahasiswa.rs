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
            CREATE TABLE IF NOT EXISTS feeder_master.biodata_mahasiswa
            (
                id uuid DEFAULT uuid_generate_v7(),
                nama_mahasiswa character varying(255),
                jenis_kelamin character varying(255),
                tempat_lahir character varying(255),
                tanggal_lahir date,
                id_mahasiswa uuid,
                id_agama integer,
                nama_agama character varying(255),
                nik character varying(255),
                nisn character varying(255),
                npwp character varying(255),
                id_negara character varying(255),
                kewarganegaraan character varying(255),
                jalan character varying(255),
                dusun character varying(255),
                rt integer,
                rw integer,
                kelurahan character varying(255),
                kode_pos character varying(255),
                id_wilayah character varying(255),
                nama_wilayah character varying(255),
                id_jenis_tinggal character varying(255),
                nama_jenis_tinggal character varying(255),
                id_alat_transportasi character varying(255),
                nama_alat_transportasi character varying(255),
                telepon character varying(255),
                handphone character varying(255),
                email character varying(255),
                penerima_kps boolean,
                nomor_kps character varying(255),
                nik_ayah character varying(255),
                nama_ayah character varying(255),
                tanggal_lahir_ayah date,
                id_pendidikan_ayah character varying(255),
                nama_pendidikan_ayah character varying(255),
                id_pekerjaan_ayah character varying(255),
                nama_pekerjaan_ayah character varying(255),
                id_penghasilan_ayah character varying(255),
                nama_penghasilan_ayah character varying(255),
                nik_ibu character varying(255),
                nama_ibu_kandung character varying(255),
                tanggal_lahir_ibu character varying(255),
                id_pendidikan_ibu character varying(255),
                nama_pendidikan_ibu character varying(255),
                id_pekerjaan_ibu character varying(255),
                nama_pekerjaan_ibu character varying(255),
                id_penghasilan_ibu character varying(255),
                nama_penghasilan_ibu character varying(255),
                nama_wali character varying(255),
                tanggal_lahir_wali character varying(255),
                id_pendidikan_wali character varying(255),
                nama_pendidikan_wali character varying(255),
                id_pekerjaan_wali character varying(255),
                nama_pekerjaan_wali character varying(255),
                id_penghasilan_wali character varying(255),
                nama_penghasilan_wali character varying(255),
                id_kebutuhan_khusus_mahasiswa character varying(255),
                nama_kebutuhan_khusus_mahasiswa character varying(255),
                id_kebutuhan_khusus_ayah character varying(255),
                nama_kebutuhan_khusus_ayah character varying(255),
                id_kebutuhan_khusus_ibu character varying(255),
                nama_kebutuhan_khusus_ibu character varying(255),
                status_sync character varying(255),
                created_at timestamp(0) without time zone DEFAULT now(),
                updated_at timestamp(0) without time zone DEFAULT now(),
                deleted_at timestamp(0) without time zone,
                sync_at timestamp(0) without time zone,
                created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                CONSTRAINT feeder_master_biodata_mahasiswa_pkey PRIMARY KEY (id)
            )
            ",
        )
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TABLE IF EXISTS feeder_master.biodata_mahasiswa")
            .await?;

        Ok(())
    }
}
